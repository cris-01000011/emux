use tui_input::Input;

use crate::{actions::navigation::View, app::App};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Default)]
pub struct SearchState {
    pub input: Input,
    pub mode: InputMode,
}

impl SearchState {
    pub fn clear(&mut self) {
        self.input.reset();
    }
}

impl App {
    pub fn start_search(&mut self) {
        self.ui_state.search.mode = InputMode::Editing;

        match self.navigation.view {
            View::Lists => self.ui_state.lists.select_first(),
            View::Items => self.ui_state.items_in_list.select_first(),
        };
    }

    pub fn stop_search(&mut self) {
        self.ui_state.search.mode = InputMode::Normal;
        self.ui_state.search.clear();
        self.reload_data();
    }
}
