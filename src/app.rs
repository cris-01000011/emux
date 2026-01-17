use home;
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
    pub current_path: PathBuf,
    pub entries: Vec<PathBuf>,
    pub selected: usize,
    pub scroll_offset: usize,

    pub in_list: bool,
    pub roms: Vec<ListItem>,
    pub current_list: String,
    pub lists_commands: Vec<ListCommand>,
    pub selected_command: usize,
    pub in_command_selection: bool,
    pub roms_scroll_offset: usize,

    // Search state
    pub in_search_mode: bool,
    pub search_query: String,
    pub search_results: Vec<usize>,
    pub search_selected: usize,

    // Favorites state
    pub favorites: Vec<FavoriteEntry>,

    // Memory for last selections
    pub directory_selections: HashMap<String, usize>,
    pub list_selections: HashMap<String, usize>,
    pub command_selections: HashMap<String, usize>,
    pub directory_scroll_selections: HashMap<String, usize>,
    pub list_scroll_selections: HashMap<String, usize>,
}

impl App {
    pub fn emux_base_path() -> PathBuf {
        // Try to read config from ~/.config/emux/emux.toml
        if let Some(home) = home::home_dir() {
            let config_path = home.join(".config/emux/emux.toml");
            if let Ok(config_content) = std::fs::read_to_string(&config_path) {
                if let Ok(config) = toml::from_str::<EmuxConfig>(&config_content) {
                    if config.paths.root != "default" {
                        return PathBuf::from(config.paths.root);
                    }
                }
            }
            // Fallback to default
            home.join("Emux")
        } else {
            PathBuf::from("./Emux")
        }
    }

    pub fn lists_path() -> PathBuf {
        Self::emux_base_path().join("lists")
    }

    pub fn new() -> Self {
        let start = Self::lists_path();

        let mut app = App {
            current_path: start.clone(),
            entries: Vec::new(),
            selected: 0,
            scroll_offset: 0,

            in_list: false,
            roms: Vec::new(),
            current_list: String::new(),
            lists_commands: Vec::new(),
            selected_command: 0,
            in_command_selection: false,
            roms_scroll_offset: 0,

            in_search_mode: false,
            search_query: String::new(),
            search_results: Vec::new(),
            search_selected: 0,

            favorites: Vec::new(),

            directory_selections: HashMap::new(),
            list_selections: HashMap::new(),
            command_selections: HashMap::new(),
            directory_scroll_selections: HashMap::new(),
            list_scroll_selections: HashMap::new(),
        };

        app.load_dir(start);
        app.load_lists_commands();
        app.load_favorites();
        app.load_list();

        app
    }

    pub fn load_dir(&mut self, dir: PathBuf) {
        // Save current selection and scroll before loading new directory
        if !self.in_list {
            let path_str = self.current_path.to_string_lossy().to_string();
            self.directory_selections
                .insert(path_str.clone(), self.selected);
            self.directory_scroll_selections
                .insert(path_str, self.scroll_offset);
        }

        self.entries = fs::read_dir(&dir)
            .map(|rd| rd.filter_map(|e| e.ok()).map(|e| e.path()).collect())
            .unwrap_or_default();

        self.entries.sort();

        self.current_path = dir.clone();

        // Restore saved selection for this directory, default to 0 if not found
        let path_str = self.current_path.to_string_lossy().to_string();
        self.selected = self
            .directory_selections
            .get(&path_str)
            .copied()
            .unwrap_or(0);

        // Restore saved scroll offset for this directory, default to 0 if not found
        self.scroll_offset = self
            .directory_scroll_selections
            .get(&path_str)
            .copied()
            .unwrap_or(0);

        self.in_list = false;
        self.current_list.clear();
        self.selected_command = 0;
        self.in_command_selection = false;
        self.scroll_offset = 0;
    }
}
