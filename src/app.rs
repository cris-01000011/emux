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
    // Separate ListState for left and center panels
    pub directory_list_state: ListState,
    pub items_list_state: ListState,

    pub current_path: PathBuf,
    pub entries: Vec<PathBuf>,

    pub in_list: bool,
    pub roms: Vec<ListItem>,
    pub current_list: String,
    pub lists_commands: Vec<ListCommand>,
    pub selected_command: usize,
    pub in_command_selection: bool,

    // Search state
    pub in_search_mode: bool,
    pub search_query: String,
    pub search_results: Vec<usize>,
    pub search_selected: usize,

    // Favorites state
    pub favorites: Vec<FavoriteEntry>,
    pub favorites_mode: bool,

    // Memory for last selections - store ListState selected indices
    pub directory_selections: HashMap<String, usize>,
    pub list_selections: HashMap<String, usize>,
    pub command_selections: HashMap<String, usize>,
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

    pub fn new() -> App {
        let start = Self::lists_path();

        let mut app = App {
            directory_list_state: ListState::default(),
            items_list_state: ListState::default(),
            current_path: start.clone(),
            entries: Vec::new(),

            in_list: false,
            roms: Vec::new(),
            current_list: String::new(),
            lists_commands: Vec::new(),
            selected_command: 0,
            in_command_selection: false,

            in_search_mode: false,
            search_query: String::new(),
            search_results: Vec::new(),
            search_selected: 0,

            favorites: Vec::new(),
            favorites_mode: false,

            directory_selections: HashMap::new(),
            list_selections: HashMap::new(),
            command_selections: HashMap::new(),
        };

        app.load_dir(start);
        app.load_lists_commands();
        app.load_list();
        app.load_favorites();

        app
    }

    pub fn load_dir(&mut self, dir: PathBuf) {
        // Save current selection before loading new directory
        if let Some(selected) = self.directory_list_state.selected() {
            let path_str = self.current_path.to_string_lossy().to_string();
            self.directory_selections.insert(path_str, selected);
        }

        self.entries = fs::read_dir(&dir)
            .map(|rd| rd.filter_map(|e| e.ok()).map(|e| e.path()).collect())
            .unwrap_or_default();

        self.entries.sort();

        self.current_path = dir.clone();

        // Restore saved selection for this directory, default to 0 if not found
        let path_str = self.current_path.to_string_lossy().to_string();
        let saved_selected = self
            .directory_selections
            .get(&path_str)
            .copied()
            .unwrap_or(0);
        self.directory_list_state.select(Some(saved_selected));

        self.in_list = false;
        self.selected_command = 0;
        self.in_command_selection = false;
    }

    pub fn ensure_selection_visible_directory(&mut self, visible_height: usize) {
        if visible_height == 0 || self.entries.is_empty() {
            return;
        }

        let Some(selected) = self.directory_list_state.selected() else {
            self.directory_list_state.select(Some(0));
            return;
        };

        if selected >= self.entries.len() {
            self.directory_list_state
                .select(Some(self.entries.len().saturating_sub(1)));
        }
    }

    pub fn ensure_selection_visible_items(&mut self, visible_height: usize) {
        if visible_height == 0 || self.roms.is_empty() {
            return;
        }

        let Some(selected) = self.items_list_state.selected() else {
            self.items_list_state.select(Some(0));
            return;
        };

        if selected >= self.roms.len() {
            self.items_list_state
                .select(Some(self.roms.len().saturating_sub(1)));
        }
    }
}
