use serde::Deserialize;
use std::{fs, path::PathBuf};

use crate::app::App;

#[derive(Deserialize, Debug, Clone, serde::Serialize)]
pub struct FavoriteEntry {
    pub list: String,
    pub title: String,
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
        if !self.in_list || self.roms.is_empty() {
            return;
        }

        let selected_item = &self.roms[self.selected];
        let favorite = FavoriteEntry {
            list: self.current_list.clone(),
            title: selected_item.title.clone(),
        };

        if let Some(pos) = self
            .favorites
            .iter()
            .position(|f| f.list == favorite.list && f.title == favorite.title)
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
            .any(|f| f.list == list && f.title == title)
    }
}
