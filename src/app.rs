use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    actions::{
        commands::CommandLists, favorite::Favorite, navigation::Navigation, search::Search,
        system::ListItem,
    },
    config::app::AppConfig,
    ui::UiState,
};

pub struct App {
    pub ui_state: UiState,

    pub lists: Vec<PathBuf>,

    pub items_in_list: Vec<ListItem>,

    pub config: AppConfig,
    pub navigation: Navigation,
    pub search: Search,
    pub commands: CommandLists,
    pub favorite: Favorite,

    pub list_selections: HashMap<String, usize>,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            ui_state: Default::default(),
            lists: Vec::new(),
            items_in_list: Vec::new(),
            config: AppConfig::load(),
            navigation: Default::default(),
            search: Default::default(),
            commands: Default::default(),
            favorite: Default::default(),

            list_selections: HashMap::new(),
        };

        app.load_default_lists();
        app.load_list();
        app.init_command_lists();
        app.init_favorites();

        app
    }

    pub fn load_default_lists(&mut self) {
        let dir = &self.config.lists_dir;

        self.lists = fs::read_dir(dir)
            .map(|rd| rd.filter_map(|e| e.ok()).map(|e| e.path()).collect())
            .unwrap_or_default();

        self.lists.sort();
    }
}
