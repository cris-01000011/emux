use serde::Deserialize;
use std::{fs, path::PathBuf};

use crate::{actions::navigation::View, app::App};

#[derive(Deserialize, Debug, Clone, serde::Serialize)]
pub struct FavoriteEntry {
    pub list: String,
    pub item: String,
    pub url: String,
}

#[derive(Default)]
pub struct Favorite {
    path: PathBuf,

    pub list: Vec<FavoriteEntry>,
    pub in_favorites: bool,
}

impl Favorite {
    pub fn update_favorites(&self) {
        if let Some(parent) = self.path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(new_favorites_json) = serde_json::to_string_pretty(&self.list) {
            let _ = std::fs::write(&self.path, new_favorites_json);
        }
    }

    pub fn is_favorite(&self, list: &str, item: &str) -> bool {
        self.list.iter().any(|f| f.list == list && f.item == item)
    }
}

impl App {
    pub fn init_favorites(&mut self) {
        self.favorite.path = self
            .config
            .base_dir
            .join("system-lists")
            .join("favorites.json");

        if let Ok(data) = std::fs::read_to_string(&self.favorite.path) {
            if let Ok(favorites) = serde_json::from_str::<Vec<FavoriteEntry>>(&data) {
                self.favorite.list = favorites;
            }
        }
    }

    pub fn toggle_favorite(&mut self) {
        if self.navigation.view == View::Lists || self.data.items_in_list.is_empty() {
            return;
        }

        let selected_item_index = self.ui_state.items_in_list.selected().unwrap_or(0);
        let selected_item = &self.data.items_in_list[selected_item_index];

        let list = self.current_list_name();
        let new_favorite = FavoriteEntry {
            list: list.to_string(),
            item: selected_item.item.clone(),
            url: selected_item.url.clone(),
        };

        if let Some(already_favorite) = self
            .favorite
            .list
            .iter()
            .position(|f| f.list == new_favorite.list && f.item == new_favorite.item)
        {
            self.favorite.list.remove(already_favorite);
        } else {
            self.favorite.list.push(new_favorite);
        }

        self.favorite.update_favorites();
        self.reload_data();
    }

    pub fn toggle_favorites_mode(&mut self) {
        if !self.favorite.in_favorites {
            self.ui_state.items_in_list.select_first();
        }

        self.favorite.in_favorites = !self.favorite.in_favorites;
        self.reload_data();
    }
}
