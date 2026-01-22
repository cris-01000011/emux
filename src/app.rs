use ratatui::widgets::ListState;
use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    actions::{
        commands::CommandLists, favorite::Favorite, navigation::Navigation, search::Search,
        system::ListItem,
    },
    config::app::AppConfig,
};

pub struct App {
    pub lists_state: ListState,
    pub items_in_list_state: ListState,

    pub lists: Vec<PathBuf>,

    pub items_in_list: Vec<ListItem>,
    pub lists_dir: PathBuf,

    pub navigation: Navigation,
    pub search: Search,
    pub commands: CommandLists,
    pub favorite: Favorite,

    pub list_selections: HashMap<String, usize>,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            lists_state: ListState::default().with_selected(Some(0)),
            items_in_list_state: ListState::default(),
            lists: Vec::new(),
            items_in_list: Vec::new(),
            lists_dir: AppConfig::get_lists_dir(),

            navigation: Default::default(),
            search: Default::default(),
            commands: Default::default(),
            favorite: Default::default(),

            list_selections: HashMap::new(),
        };

        app.load_default_lists();
        app.load_list();
        app.commands.init_command_lists();
        app.favorite.init_favorites();

        app
    }

    pub fn load_default_lists(&mut self) {
        let dir = AppConfig::get_lists_dir();

        self.lists = fs::read_dir(&dir)
            .map(|rd| rd.filter_map(|e| e.ok()).map(|e| e.path()).collect())
            .unwrap_or_default();

        self.lists.sort();
    }
}
