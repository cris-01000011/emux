use home;
use ratatui::widgets::ListState;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::PathBuf};

use crate::actions::{
    favorite::FavoriteEntry,
    system::{Command, ListItem},
};

#[derive(Deserialize, Debug)]
struct EmuxConfig {
    paths: PathsConfig,
}

#[derive(Deserialize, Debug)]
struct PathsConfig {
    root: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListCommand {
    pub list: String,
    pub commands: Vec<Command>,
}

pub struct App {
    pub lists_state: ListState,
    pub items_in_list_state: ListState,

    pub current_path: PathBuf,
    pub lists: Vec<PathBuf>,

    pub in_list: bool,
    pub items_in_list: Vec<ListItem>,
    pub current_list: String,
    pub lists_commands: Vec<ListCommand>,
    pub selected_command: usize,

    // Search state
    pub in_search_mode: bool,
    pub search_query: String,

    // Favorites state
    pub favorites: Vec<FavoriteEntry>,
    pub favorites_mode: bool,

    // Memory for last selections - store ListState selected indices
    pub directory_selections: HashMap<String, usize>,
    pub list_selections: HashMap<String, usize>,
}

impl App {
    pub fn emux_base_path() -> PathBuf {
        let Some(home) = home::home_dir() else {
            return PathBuf::from("./Emux");
        };

        let config_path = home.join(".config/emux/emux.toml");

        let Ok(config_content) = std::fs::read_to_string(&config_path) else {
            return home.join("Emux");
        };

        let Ok(config) = toml::from_str::<EmuxConfig>(&config_content) else {
            return home.join("Emux");
        };

        if config.paths.root != "default" {
            return PathBuf::from(config.paths.root);
        }

        home.join("Emux")
    }

    pub fn lists_path() -> PathBuf {
        Self::emux_base_path().join("lists")
    }

    pub fn new() -> App {
        let mut app = App {
            lists_state: ListState::default().with_selected(Some(0)),
            items_in_list_state: ListState::default(),
            current_path: Self::lists_path().clone(),
            lists: Vec::new(),

            in_list: false,
            items_in_list: Vec::new(),
            current_list: String::new(),
            lists_commands: Vec::new(),
            selected_command: 0,

            in_search_mode: false,
            search_query: String::new(),

            favorites: Vec::new(),
            favorites_mode: false,

            directory_selections: HashMap::new(),
            list_selections: HashMap::new(),
        };

        app.init_lists();
        app.load_lists_commands();
        app.load_list();
        app.load_favorites();

        app
    }

    pub fn init_lists(&mut self) {
        let dir = Self::lists_path();

        self.lists = fs::read_dir(&dir)
            .map(|rd| rd.filter_map(|e| e.ok()).map(|e| e.path()).collect())
            .unwrap_or_default();

        self.lists.sort();
    }
}
