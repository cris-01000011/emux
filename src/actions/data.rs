use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::{actions::navigation::View, app::App, components::inputs::search::InputMode};

#[derive(Default)]
pub struct AppData {
    pub lists: Vec<PathBuf>,
    pub items_in_list: Vec<ListItem>,
    pub items_in_list_selections: HashMap<PathBuf, usize>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListItem {
    pub item: String,
    pub url: String,
}

impl App {
    pub fn init_lists(&mut self) {
        self.load_default_lists();
        self.data.lists.sort();
        self.ui.lists.select_first();

        let path = self.data.lists.first();
        if let Some(path) = path {
            self.load_list(&path.to_path_buf());
        }
    }

    fn load_default_lists(&mut self) {
        let dir = &self.config.lists_dir;

        self.data.lists = fs::read_dir(dir)
            .map(|rd| rd.filter_map(|e| e.ok()).map(|e| e.path()).collect())
            .unwrap_or_default();
    }

    pub fn reload_data(&mut self) {
        if self.navigation.view == View::Items {
            if self.ui.search.mode == InputMode::Editing {
                self.load_items();
                self.search_items_in_list();
            } else {
                self.restore_items();
            }

            return;
        }

        if self.ui.search.mode == InputMode::Editing {
            self.load_default_lists();
            self.search_lists();
        } else {
            self.restore_lists();
        }

        if let Some(path) = self.selected_json_path() {
            self.load_list(&path);
        }

        self.load_current_commands();
    }

    pub fn current_list_name(&self) -> &str {
        self.navigation
            .current_list_path
            .as_ref()
            .and_then(|p| p.file_stem())
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
    }

    fn restore_lists(&mut self) {
        let selected_path = self
            .data
            .lists
            .get(self.ui.lists.selected().unwrap_or_default())
            .cloned();

        self.load_default_lists();

        if let Some(path) = selected_path {
            if let Some(new_index) = self.data.lists.iter().position(|p| p == &path) {
                self.ui.lists.select(Some(new_index));
            }
        }
    }

    fn restore_items(&mut self) {
        let selected_item = self
            .data
            .items_in_list
            .get(self.ui.items_in_list.selected().unwrap_or(0))
            .cloned();

        self.load_items();

        if let Some(item) = selected_item {
            if let Some(new_index) = self
                .data
                .items_in_list
                .iter()
                .position(|x| x.item == item.item)
            {
                self.ui.items_in_list.select(Some(new_index));
            }
        }
    }

    fn search_lists(&mut self) {
        let query = self.ui.search.input.value().to_lowercase();

        self.data.lists.retain(|path| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .map(|name| name.to_lowercase().contains(&query))
                .unwrap_or(false)
        });
    }

    fn search_items_in_list(&mut self) {
        let query = self.ui.search.input.value().to_lowercase();

        self.data.items_in_list = self
            .data
            .items_in_list
            .iter()
            .filter(|rom| rom.item.to_lowercase().contains(&query))
            .cloned()
            .collect();
    }

    fn load_items(&mut self) {
        let Some(path) = &self.navigation.current_list_path else {
            self.data.items_in_list.clear();
            return;
        };

        let data = fs::read_to_string(path).unwrap_or_default();

        match serde_json::from_str::<Vec<ListItem>>(&data) {
            Ok(mut items) => {
                if self.favorite.in_favorites {
                    let list = self.current_list_name();
                    items.retain(|item| self.favorite.is_favorite(list, &item.item));
                }

                self.data.items_in_list = items;
            }
            Err(e) => {
                eprintln!("error parsing JSON for {}: {}", path.display(), e);
                self.data.items_in_list.clear();
            }
        }
    }

    fn selected_json_path(&self) -> Option<PathBuf> {
        let selected = self.ui.lists.selected().unwrap_or_default();
        let path = self.data.lists.get(selected)?.clone();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            return Some(path);
        }

        None
    }

    fn load_list(&mut self, path: &Path) {
        self.navigation.current_list_path = Some(path.to_path_buf());

        self.restore_item_selected();
        self.load_items();
    }

    pub fn save_item_selected(&mut self) {
        if let Some(path) = &self.navigation.current_list_path {
            if let Some(selected) = self.ui.items_in_list.selected() {
                self.data
                    .items_in_list_selections
                    .insert(path.to_path_buf(), selected);
            }
        }
    }

    fn restore_item_selected(&mut self) {
        if self.favorite.in_favorites {
            self.ui.items_in_list.select(Some(0));
            return;
        }

        let Some(path) = &self.navigation.current_list_path else {
            return;
        };

        let restored = self
            .data
            .items_in_list_selections
            .get(path)
            .copied()
            .unwrap_or_default();

        self.ui.items_in_list.select(Some(restored));
    }
}
