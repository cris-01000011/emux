use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::{
    actions::navigation::View, app::App, components::input::InputActive,
    utils::string::sanitize_filename,
};

#[derive(Default)]
pub struct AppData {
    pub lists: Vec<PathBuf>,
    pub items_in_list: HashMap<PathBuf, Vec<ListItem>>,
    pub list_downloaded_sizes: HashMap<PathBuf, u64>,
    pub local_lists: Vec<PathBuf>,
    pub items_in_local_list: HashMap<PathBuf, Vec<ListItem>>,
    pub local_list_downloaded_sizes: HashMap<PathBuf, u64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListItem {
    pub item: String,
    pub url: String,
}

impl App {
    pub fn init_lists(&mut self) {
        self.load_default_lists();
        self.load_all_list_items();
        self.load_all_list_sizes();
        self.load_local_lists();
        self.load_all_local_list_items();

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

    pub fn load_local_lists(&mut self) {
        let dirs: Vec<PathBuf> = match fs::read_dir(self.config.base_dir.join("local")) {
            Ok(entries) => entries
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let metadata = entry.metadata().ok()?;
                    if metadata.is_dir() {
                        Some(entry.path())
                    } else {
                        None
                    }
                })
                .collect(),
            Err(_) => Vec::new(),
        };
        self.data.local_lists = dirs;
    }

    fn load_all_list_items(&mut self) {
        self.data.items_in_list.clear();

        for list_path in &self.data.lists {
            if let Some(data) = self.load_list_data(list_path) {
                self.data.items_in_list.insert(list_path.clone(), data);
            }
        }
    }

    pub fn load_all_local_list_items(&mut self) {
        self.data.items_in_local_list.clear();

        for dir in &self.data.local_lists {
            let items = self.load_local_list_items(dir);
            self.data.items_in_local_list.insert(dir.clone(), items);
        }
    }

    fn load_list_data(&self, path: &Path) -> Option<Vec<ListItem>> {
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            return None;
        }

        let data = fs::read_to_string(path).ok()?;
        serde_json::from_str::<Vec<ListItem>>(&data).ok()
    }

    fn load_all_list_sizes(&mut self) {
        self.data.list_downloaded_sizes.clear();
        self.data.local_list_downloaded_sizes.clear();

        for list_path in &self.data.lists {
            let size = self.calculate_list_downloaded_size(list_path);
            self.data
                .list_downloaded_sizes
                .insert(list_path.to_path_buf(), size);
        }

        for local_list_path in &self.data.local_lists {
            let size = self.calculate_list_downloaded_size(local_list_path);
            self.data
                .local_list_downloaded_sizes
                .insert(local_list_path.to_path_buf(), size);
        }
    }

    pub fn reload_data(&mut self) {
        if self.navigation.view == View::Lists
            && self.ui.input.active == InputActive::Search
            && !self.ui.input.search.value().is_empty()
            && let Some(selected_index) = self.ui.lists.selected()
            && let Some(&real_list_index) = self.search.lists_query.get(selected_index)
            && let Some(path) = self.data.lists.get(real_list_index)
        {
            self.navigation.current_list_path = Some(path.clone());
        } else if let Some(path) = self.selected_json_path() {
            self.load_list(&path);
        }

        self.load_current_commands();
    }

    pub fn current_list_name(&self) -> &str {
        if self.navigation.list_view == crate::actions::navigation::ListsView::LocalLists {
            return self.current_local_list_name();
        }

        self.navigation
            .current_list_path
            .as_ref()
            .and_then(|p| p.file_stem())
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
    }

    pub fn current_local_list_name(&self) -> &str {
        let path = match &self.navigation.current_local_list_path {
            Some(p) => p,
            None => {
                let selected = self.ui.lists.selected().unwrap_or_default();
                return self
                    .data
                    .local_lists
                    .get(selected)
                    .and_then(|p| p.file_name())
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");
            }
        };

        path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
    }

    pub fn get_current_list_items(&self) -> Vec<ListItem> {
        if self.navigation.list_view == crate::actions::navigation::ListsView::LocalLists {
            return self.get_current_local_list_items();
        }

        if let Some(path) = &self.navigation.current_list_path
            && let Some(items) = self.data.items_in_list.get(path)
        {
            let mut result = items.clone();

            if self.favorite.in_favorites {
                let list = self.current_list_name();
                result.retain(|item| self.favorite.is_favorite(list, &item.item));
            }

            result
        } else {
            Vec::new()
        }
    }

    pub fn get_current_local_list_items(&self) -> Vec<ListItem> {
        let path = match &self.navigation.current_local_list_path {
            Some(p) => p.clone(),
            None => {
                let selected = self.ui.lists.selected().unwrap_or_default();
                if let Some(local_list) = self.data.local_lists.get(selected) {
                    local_list.clone()
                } else {
                    return Vec::new();
                }
            }
        };

        if let Some(cached) = self.data.items_in_local_list.get(&path) {
            return cached.clone();
        }

        if !path.exists() {
            return Vec::new();
        }

        self.load_local_list_items(&path)
    }

    pub fn load_local_list_items(&self, path: &std::path::Path) -> Vec<ListItem> {
        let mut entries: Vec<ListItem> = fs::read_dir(path)
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .map(|e| {
                        let entry_path = e.path();
                        let name = entry_path
                            .file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or("")
                            .to_string();
                        let is_dir = entry_path.is_dir();
                        ListItem {
                            item: name,
                            url: if is_dir {
                                String::new()
                            } else {
                                entry_path.to_string_lossy().to_string()
                            },
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        entries.sort_by(|a, b| {
            let a_is_dir = !a.url.is_empty();
            let b_is_dir = !b.url.is_empty();
            if a_is_dir != b_is_dir {
                b_is_dir.cmp(&a_is_dir)
            } else {
                a.item.to_lowercase().cmp(&b.item.to_lowercase())
            }
        });

        entries
    }

    pub fn get_current_list_items_count(&self) -> usize {
        if self.navigation.list_view == crate::actions::navigation::ListsView::LocalLists {
            return self.get_current_local_list_items_count();
        }
        self.get_current_list_items().len()
    }

    pub fn get_current_local_list_items_count(&self) -> usize {
        self.get_current_local_list_items().len()
    }

    pub fn search_lists(&mut self) {
        let query = self.ui.input.search.value().to_lowercase();
        self.search.lists_query.clear();

        for (index, path) in self.data.lists.iter().enumerate() {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str())
                && name.to_lowercase().contains(&query)
            {
                self.search.lists_query.push(index);
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
    }

    fn calculate_list_downloaded_size(&self, list_path: &Path) -> u64 {
        let list_name = list_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        let download_dir = self.config.base_dir.join("downloads").join(list_name);

        if !download_dir.exists() {
            return 0;
        }

        match fs::read_dir(&download_dir) {
            Ok(entries) => entries
                .filter_map(|e| e.ok())
                .filter_map(|e| e.metadata().ok())
                .filter(|m| m.is_file())
                .map(|m| m.len())
                .sum(),
            Err(_) => 0,
        }
    }

    pub fn get_current_list_downloaded_size(&self) -> u64 {
        if let Some(path) = &self.navigation.current_list_path {
            return *self.data.list_downloaded_sizes.get(path).unwrap_or(&0);
        }

        0
    }

    pub fn refresh_current_list_size(&mut self) {
        if let Some(path) = &self.navigation.current_list_path {
            let size = self.calculate_list_downloaded_size(path);
            self.data.list_downloaded_sizes.insert(path.clone(), size);
        }
    }

    pub fn get_current_item_downloaded_size(&self) -> u64 {
        let selected = self.scroll.index_in_list();

        let item_index = if self.ui.input.active == InputActive::Search {
            match self.search.items_query.get(selected) {
                Some(idx) => *idx,
                None => return 0,
            }
        } else {
            selected
        };

        let items = self.get_current_list_items();
        let item = match items.get(item_index) {
            Some(item) => item,
            None => return 0,
        };

        let list_name = self.current_list_name();
        let download_dir = self.config.base_dir.join("downloads").join(list_name);

        let clean_filename = sanitize_filename(&item.item);
        let file_path = download_dir.join(clean_filename);

        match fs::metadata(&file_path) {
            Ok(metadata) if metadata.is_file() => metadata.len(),
            _ => 0,
        }
    }

    pub fn is_downloaded(&self, item_name: &str) -> bool {
        let list_name = self.current_list_name();
        let download_dir = self.config.base_dir.join("downloads").join(list_name);

        let clean_filename = sanitize_filename(item_name);
        let file_path = download_dir.join(clean_filename);

        match fs::metadata(file_path) {
            Ok(metadata) => metadata.is_file(),
            Err(_) => false,
        }
    }
}
