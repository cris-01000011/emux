use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    actions::{
        commands::CommandLists, favorite::Favorite, navigation::Navigation, search::Search,
        system::ListItem,
    },
    config::app::AppConfig,
    ui::UiState,
};

#[derive(Default)]
pub struct AppData {
    pub lists: Vec<PathBuf>,
    pub items_in_list: Vec<ListItem>,
    pub list_selections: HashMap<String, usize>,
}

pub struct App {
    pub commands: CommandLists,
    pub config: AppConfig,
    pub data: AppData,
    pub favorite: Favorite,
    pub navigation: Navigation,
    pub search: Search,
    pub ui_state: UiState,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            commands: Default::default(),
            config: AppConfig::load(),
            data: Default::default(),
            favorite: Default::default(),
            navigation: Default::default(),
            search: Default::default(),
            ui_state: Default::default(),
        };

        app.load_default_lists();
        app.load_list();
        app.init_command_lists();
        app.init_favorites();

        app
    }

    pub fn load_default_lists(&mut self) {
        let dir = &self.config.lists_dir;

        self.data.lists = fs::read_dir(dir)
            .map(|rd| rd.filter_map(|e| e.ok()).map(|e| e.path()).collect())
            .unwrap_or_default();

        self.data.lists.sort();
    }
}
