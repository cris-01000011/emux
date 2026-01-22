use serde::Deserialize;

use crate::{actions::navigation::View, app::App};

#[derive(Deserialize, Debug, Clone)]
pub struct ListItem {
    pub item: String,
    pub url: String,
}

impl App {
    pub fn load_list(&mut self) {
        if self.navigation.view == View::Items {
            self.load_current_commands();
            let selected_item = self
                .data
                .items_in_list
                .get(self.ui_state.items_in_list_state.selected().unwrap_or(0))
                .cloned();

            self.load_items_for_current_list();

            if self.favorite.in_favorites {
                self.apply_favorites_filter_items();
            }

            if self.search.in_search {
                self.apply_search_filter_items();
            } else {
                if let Some(item) = selected_item {
                    if let Some(new_index) = self
                        .data
                        .items_in_list
                        .iter()
                        .position(|x| x.item == item.item)
                    {
                        self.ui_state.items_in_list_state.select(Some(new_index));
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
        if !self.search.in_search || self.search.search_query.is_empty() {
            let selected_path = self
                .data
                .lists
                .get(self.ui_state.lists_state.selected().unwrap_or(0))
                .cloned();

            self.load_default_lists();

            if let Some(path) = selected_path {
                if let Some(new_index) = self.data.lists.iter().position(|p| p == &path) {
                    self.ui_state.lists_state.select(Some(new_index));
                }
            }
        }

        let q = self.search.search_query.to_lowercase();

        self.data.lists.retain(|path| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .map(|name| name.to_lowercase().contains(&q))
                .unwrap_or(false)
        });
    }

    fn load_items_for_current_list(&mut self) {
        let path = self
            .config
            .lists_dir
            .join(format!("{}.json", self.navigation.current_list));

        let data = std::fs::read_to_string(&path).unwrap_or_default();

        match serde_json::from_str::<Vec<ListItem>>(&data) {
            Ok(roms) => {
                // Always load full data first
                self.data.items_in_list = roms;
            }
            Err(e) => {
                eprintln!("Error parsing JSON for {}: {}", path.display(), e);
                self.data.items_in_list = Vec::new();
            }
        }
    }

    fn apply_favorites_filter_items(&mut self) {
        let current_list = self.navigation.current_list.clone();
        let favorites = self.favorite.list.clone();

        self.data.items_in_list.retain(|item| {
            favorites
                .iter()
                .any(|f| f.list == current_list && f.item == item.item)
        });
    }

    fn apply_search_filter_items(&mut self) {
        let query_lower = self.search.search_query.to_lowercase();

        self.data.items_in_list = self
            .data
            .items_in_list
            .iter()
            .filter(|rom| rom.item.to_lowercase().contains(&query_lower))
            .cloned()
            .collect();
    }

    fn fix_items_selection(&mut self) {
        if let Some(selected) = self.ui_state.items_in_list_state.selected() {
            let len = self.data.items_in_list.len();
            if selected >= len && len > 0 {
                self.ui_state
                    .items_in_list_state
                    .select(Some(len.saturating_sub(1)));
            }
        }
    }

    fn selected_json_path(&self) -> Option<std::path::PathBuf> {
        let selected = self.ui_state.lists_state.selected().unwrap_or(0);
        let path = self.data.lists.get(selected)?.clone();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            Some(path)
        } else {
            None
        }
    }

    fn enter_selected_list(&mut self, path: &std::path::Path) {
        // Set current list name
        self.navigation.current_list = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Restore previous selection
        let restored = self
            .data
            .list_selections
            .get(&self.navigation.current_list)
            .copied()
            .unwrap_or(0);

        self.ui_state.items_in_list_state.select(Some(restored));

        // Load items
        let data = std::fs::read_to_string(path).unwrap_or_default();

        match serde_json::from_str::<Vec<ListItem>>(&data) {
            Ok(mut roms) => {
                if self.favorite.in_favorites {
                    roms.retain(|item| {
                        self.favorite
                            .is_favorite(&self.navigation.current_list, &item.item)
                    });
                }

                self.data.items_in_list = roms;
            }
            Err(e) => {
                eprintln!("Error parsing JSON for {}: {}", path.display(), e);
                self.data.items_in_list = Vec::new();
            }
        }
    }
}
