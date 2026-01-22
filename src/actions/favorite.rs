use serde::Deserialize;
use std::{fs, path::PathBuf};

use crate::{actions::navigation::View, app::App, config::app::AppConfig};

#[derive(Deserialize, Debug, Clone, serde::Serialize)]
pub struct FavoriteEntry {
    pub list: String,
    pub item: String,
    pub url: String,
}

#[derive(Default)]
pub struct Favorite {
    favorites_path: PathBuf,

    pub list_favorites: Vec<FavoriteEntry>,
    pub in_favorites: bool,
}

impl Favorite {
    fn get_favorites_path() -> PathBuf {
        AppConfig::base_dir()
            .join("system-lists")
            .join("favorites.json")
    }

    pub fn init_favorites(&mut self) {
        self.favorites_path = Self::get_favorites_path();
        if let Ok(data) = std::fs::read_to_string(&self.favorites_path) {
            if let Ok(favorites) = serde_json::from_str::<Vec<FavoriteEntry>>(&data) {
                self.list_favorites = favorites;
            }
        }
    }

    pub fn update_favorites(&self) {
        if let Some(parent) = self.favorites_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(new_favorites_json) = serde_json::to_string_pretty(&self.list_favorites) {
            let _ = std::fs::write(&self.favorites_path, new_favorites_json);
        }
    }

    pub fn is_favorite(&self, list: &str, item: &str) -> bool {
        self.list_favorites
            .iter()
            .any(|f| f.list == list && f.item == item)
    }
}

impl App {
    pub fn toggle_favorite(&mut self) {
        if self.navigation.view == View::Lists || self.items_in_list.is_empty() {
            return;
        }

        let selected_item_index = self.items_in_list_state.selected().unwrap_or(0);
        let selected_item = &self.items_in_list[selected_item_index];

        let new_favorite = FavoriteEntry {
            list: self.navigation.current_list.clone(),
            item: selected_item.item.clone(),
            url: selected_item.url.clone(),
        };

        if let Some(already_favorite) = self
            .favorite
            .list_favorites
            .iter()
            .position(|f| f.list == new_favorite.list && f.item == new_favorite.item)
        {
            self.favorite.list_favorites.remove(already_favorite);
        } else {
            self.favorite.list_favorites.push(new_favorite);
        }

        self.favorite.update_favorites();
        self.load_list();
    }

    pub fn toggle_favorites_mode(&mut self) {
        if !self.favorite.in_favorites {
            self.items_in_list_state.select_first();
        }

        self.favorite.in_favorites = !self.favorite.in_favorites;
        self.load_list();
    }
}
