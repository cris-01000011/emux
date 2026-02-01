use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::{
    actions::download, actions::navigation::View, app::App, components::input::InputActive,
};

#[derive(Default)]
pub struct AppData {
    pub lists: Vec<PathBuf>,
    pub all_list_items: HashMap<PathBuf, Vec<ListItem>>, // All items for each list loaded once
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
        self.load_all_list_items(); // Load all items upfront
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

    fn load_all_list_items(&mut self) {
        self.data.all_list_items.clear();

        for list_path in &self.data.lists {
            if let Some(data) = self.load_list_data(&list_path) {
                self.data.all_list_items.insert(list_path.clone(), data);
            }
        }
    }

    fn load_list_data(&self, path: &Path) -> Option<Vec<ListItem>> {
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            return None;
        }

        let data = fs::read_to_string(path).ok()?;
        serde_json::from_str::<Vec<ListItem>>(&data).ok()
    }

    pub fn reload_data(&mut self) {
        // Simply update the current list reference, no data reloading
        if self.navigation.view == View::Lists {
            // When in Lists view, check if we're in search mode
            if self.ui.input.active == InputActive::Search
                && !self.ui.input.search.value().is_empty()
            {
                // Use the real list index from search results
                if let Some(selected_index) = self.ui.lists.selected() {
                    if let Some(&real_list_index) = self.search.lists_query.get(selected_index) {
                        if let Some(path) = self.data.lists.get(real_list_index) {
                            self.navigation.current_list_path = Some(path.clone());
                        }
                    }
                }
            } else {
                // Use normal selection
                if let Some(path) = self.selected_json_path() {
                    self.load_list(&path);
                }
            }
        } else {
            // For Items view, just reload commands
            if let Some(path) = self.selected_json_path() {
                self.load_list(&path);
            }
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

    pub fn get_current_list_items(&self) -> Vec<ListItem> {
        if let Some(path) = &self.navigation.current_list_path {
            if let Some(items) = self.data.all_list_items.get(path) {
                let mut result = items.clone();

                // Apply favorites filter if needed
                if self.favorite.in_favorites {
                    let list = self.current_list_name();
                    result.retain(|item| self.favorite.is_favorite(list, &item.item));
                }

                return result;
            }
        }
        Vec::new()
    }

    pub fn get_current_list_items_count(&self) -> usize {
        self.get_current_list_items().len()
    }

    // Search methods that populate SearchState queries
    pub fn search_lists(&mut self) {
        let query = self.ui.input.search.value().to_lowercase();
        self.search.lists_query.clear();

        for (index, path) in self.data.lists.iter().enumerate() {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                if name.to_lowercase().contains(&query) {
                    self.search.lists_query.push(index);
                }
            }
        }
    }

    pub fn search_items(&mut self) {
        let query = self.ui.input.search.value().to_lowercase();
        self.search.items_query.clear();

        let items = self.get_current_list_items();
        for (index, item) in items.iter().enumerate() {
            if item.item.to_lowercase().contains(&query) {
                self.search.items_query.push(index);
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

    pub fn get_current_list_downloaded_size(&self) -> u64 {
        let list_name = self.current_list_name();
        let download_dir = self.config.base_dir.join("downloads").join(list_name);

        if !download_dir.exists() {
            return 0;
        }

        match fs::read_dir(&download_dir) {
            Ok(entries) => entries
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| entry.metadata().ok())
                .filter(|metadata| metadata.is_file())
                .map(|metadata| metadata.len())
                .sum(),
            Err(_) => 0,
        }
    }

    pub fn get_current_item_downloaded_size(&self) -> u64 {
        if self.navigation.view != crate::actions::navigation::View::Items {
            return 0;
        }

        let current_items = self.get_current_list_items();
        let selected_index = self.ui.items_in_list.selected().unwrap_or(0);

        if selected_index >= current_items.len() {
            return 0;
        }

        let selected_item = &current_items[selected_index];
        let list_name = self.current_list_name();
        let download_dir = self.config.base_dir.join("downloads").join(list_name);

        let clean_filename = download::sanitize_filename(&selected_item.item);
        let file_path = download_dir.join(&clean_filename);

        match fs::metadata(&file_path) {
            Ok(metadata) if metadata.is_file() => metadata.len(),
            _ => 0,
        }
    }
}
