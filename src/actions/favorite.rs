use serde::Deserialize;
use std::{fs, path::PathBuf};

use crate::app::App;

#[derive(Deserialize, Debug, Clone, serde::Serialize)]
pub struct FavoriteEntry {
    pub system: String,
    pub title: String,
}

impl App {
    fn favorites_path() -> PathBuf {
        Self::emux_base_path().join("lists").join("favorites.json")
    }

    pub fn load_favorites(&mut self) {
        let favorites_path = Self::favorites_path();
        if let Ok(data) = std::fs::read_to_string(&favorites_path) {
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
        if !self.in_system || self.roms.is_empty() {
            return;
        }

        let selected_rom = &self.roms[self.selected];
        let favorite = FavoriteEntry {
            system: self.current_system.clone(),
            title: selected_rom.title.clone(),
        };

        // Check if already favorite
        if let Some(pos) = self
            .favorites
            .iter()
            .position(|f| f.system == favorite.system && f.title == favorite.title)
        {
            // Remove from favorites
            self.favorites.remove(pos);
        } else {
            // Add to favorites
            self.favorites.push(favorite);
        }

        self.save_favorites();
    }

    pub fn is_favorite(&self, system: &str, title: &str) -> bool {
        self.favorites
            .iter()
            .any(|f| f.system == system && f.title == title)
    }
}
