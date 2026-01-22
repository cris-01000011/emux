use crate::{
    actions::{
        commands::CommandLists, data::AppData, favorite::Favorite, navigation::Navigation,
        search::Search,
    },
    config::app::AppConfig,
    ui::UiState,
};

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

        app.init_lists();
        app.init_command_lists();
        app.init_favorites();

        app
    }
}
