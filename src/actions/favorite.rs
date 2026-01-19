use serde::Deserialize;
use std::{fs, path::PathBuf};

use crate::app::App;

#[derive(Deserialize, Debug, Clone, serde::Serialize)]
pub struct FavoriteEntry {
    pub list: String,
    pub item: String,
    pub url: String,
}

impl App {
    fn favorites_path() -> PathBuf {
        Self::emux_base_path()
            .join("system-lists")
            .join("favorites.json")
    }

    pub fn load_favorites(&mut self) {
        let favorites_path = Self::favorites_path();
        if let Ok(data) = std::fs::read_to_string(favorites_path) {
            if let Ok(favorites) = serde_json::from_str::<Vec<FavoriteEntry>>(&data) {
                self.favorites = favorites;
            }
        }
    }

    pub fn save_favorites(&self) {
        let favorites_path = Self::favorites_path();
        if let Some(parent) = favorites_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(json) = serde_json::to_string_pretty(&self.favorites) {
            let _ = std::fs::write(&favorites_path, json);
        }
    }

    pub fn toggle_favorite(&mut self) {
        if !self.in_list || self.items_in_list.is_empty() {
            return;
        }

        let selected_item = &self.items_in_list[self.items_in_list_state.selected().unwrap_or(0)];
        let favorite = FavoriteEntry {
            list: self.current_list.clone(),
            item: selected_item.item.clone(),
            url: selected_item.url.clone(),
        };

        if let Some(pos) = self
            .favorites
            .iter()
            .position(|f| f.list == favorite.list && f.item == favorite.item)
        {
            self.favorites.remove(pos);
        } else {
            self.favorites.push(favorite);
        }

        self.save_favorites();
    }

    pub fn is_favorite(&self, list: &str, title: &str) -> bool {
        self.favorites
            .iter()
            .any(|f| f.list == list && f.item == title)
    }
}
