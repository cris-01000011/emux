use serde::Deserialize;

use crate::{app::App, config::app::AppConfig};

#[derive(Deserialize, Debug, Clone)]
pub struct Command {
    pub name: String,
    pub command: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListItem {
    pub item: String,
    pub url: String,
}

impl App {
    pub fn load_lists_commands(&mut self) {
        let commands_path = AppConfig::base_dir().join("lists_commands.json");
        let data = std::fs::read_to_string(&commands_path).unwrap_or_default();
        self.lists_commands = serde_json::from_str(&data).unwrap_or_default();
    }

    fn clean_list_name(list: &str) -> String {
        list.split('(').next().unwrap_or(list).trim().to_string()
    }

    pub fn get_current_commands(&self) -> Vec<Command> {
        if self.current_list.is_empty() {
            return Vec::new();
        }

        let clean_list = Self::clean_list_name(&self.current_list);

        self.lists_commands
            .iter()
            .find(|sc| sc.list == clean_list)
            .map(|sc| sc.commands.clone())
            .unwrap_or_default()
    }

    pub fn load_list(&mut self) {
        if self.in_list {
            let selected_item = self
                .items_in_list
                .get(self.items_in_list_state.selected().unwrap_or(0))
                .cloned();

            self.load_items_for_current_list();

            if self.favorite.in_favorites {
                self.apply_favorites_filter_items();
            }

            if self.in_search_mode {
                self.apply_search_filter_items();
            } else {
                if let Some(item) = selected_item {
                    if let Some(new_index) =
                        self.items_in_list.iter().position(|x| x.item == item.item)
                    {
                        self.items_in_list_state.select(Some(new_index));
                    }
                }
            }

            self.fix_items_selection();
            return;
        }

        // We are in the list of JSON files (systems / lists)
        self.apply_search_filter_lists();

        if let Some(path) = self.selected_json_path() {
            self.enter_selected_list(&path);
        }
    }

    fn apply_search_filter_lists(&mut self) {
        if !self.in_search_mode || self.search_query.is_empty() {
            let selected_path = self
                .lists
                .get(self.lists_state.selected().unwrap_or(0))
                .cloned();

            self.init_lists();

            if let Some(path) = selected_path {
                if let Some(new_index) = self.lists.iter().position(|p| p == &path) {
                    self.lists_state.select(Some(new_index));
                }
            }
        }

        let q = self.search_query.to_lowercase();

        self.lists.retain(|path| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .map(|name| name.to_lowercase().contains(&q))
                .unwrap_or(false)
        });
    }

    fn load_items_for_current_list(&mut self) {
        let path = self
            .current_path
            .join(format!("{}.json", self.current_list));

        let data = std::fs::read_to_string(&path).unwrap_or_default();

        match serde_json::from_str::<Vec<ListItem>>(&data) {
            Ok(roms) => {
                // Always load full data first
                self.items_in_list = roms;
            }
            Err(e) => {
                eprintln!("Error parsing JSON for {}: {}", path.display(), e);
                self.items_in_list = Vec::new();
            }
        }
    }

    fn apply_favorites_filter_items(&mut self) {
        let current_list = self.current_list.clone();
        let favorites = self.favorite.list_favorites.clone();

        self.items_in_list.retain(|item| {
            favorites
                .iter()
                .any(|f| f.list == current_list && f.item == item.item)
        });
    }

    fn apply_search_filter_items(&mut self) {
        let query_lower = self.search_query.to_lowercase();

        self.items_in_list = self
            .items_in_list
            .iter()
            .filter(|rom| rom.item.to_lowercase().contains(&query_lower))
            .cloned()
            .collect();
    }

    fn fix_items_selection(&mut self) {
        if let Some(selected) = self.items_in_list_state.selected() {
            let len = self.items_in_list.len();
            if selected >= len && len > 0 {
                self.items_in_list_state.select(Some(len.saturating_sub(1)));
            }
        }
    }

    fn selected_json_path(&self) -> Option<std::path::PathBuf> {
        let selected = self.lists_state.selected().unwrap_or(0);
        let path = self.lists.get(selected)?.clone();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            Some(path)
        } else {
            None
        }
    }

    fn enter_selected_list(&mut self, path: &std::path::Path) {
        // Set current list name
        self.current_list = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Restore previous selection
        let restored = self
            .list_selections
            .get(&self.current_list)
            .copied()
            .unwrap_or(0);

        self.items_in_list_state.select(Some(restored));

        // Save directory selection
        let path_str = self.current_path.to_string_lossy().to_string();
        if let Some(selected) = self.lists_state.selected() {
            self.directory_selections.insert(path_str, selected);
        }

        // Load items
        let data = std::fs::read_to_string(path).unwrap_or_default();

        match serde_json::from_str::<Vec<ListItem>>(&data) {
            Ok(mut roms) => {
                if self.favorite.in_favorites {
                    roms.retain(|item| self.favorite.is_favorite(&self.current_list, &item.item));
                }

                self.items_in_list = roms;
            }
            Err(e) => {
                eprintln!("Error parsing JSON for {}: {}", path.display(), e);
                self.items_in_list = Vec::new();
            }
        }
    }
}
