use std::path::PathBuf;

use rand::Rng;
use ratatui::widgets::ListState;

use crate::app::App;

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq)]
pub enum View {
    #[default]
    Lists,
    Items,
}

#[derive(Default)]
pub struct Navigation {
    pub view: View,
    pub current_list_path: Option<PathBuf>,
}

impl App {
    fn current_list_state(&mut self) -> &mut ListState {
        match self.navigation.view {
            View::Lists => &mut self.ui_state.lists,
            View::Items => &mut self.ui_state.items_in_list,
        }
    }

    fn current_list_len(&self) -> usize {
        match self.navigation.view {
            View::Lists => self.data.lists.len(),
            View::Items => self.data.items_in_list.len(),
        }
    }

    pub fn move_up(&mut self) {
        self.current_list_state().select_previous();
        self.reload_data();
    }

    pub fn move_down(&mut self) {
        self.current_list_state().select_next();
        self.reload_data();
    }

    pub fn go_to_first_item(&mut self) {
        self.current_list_state().select_first();
        self.reload_data();
    }

    pub fn go_to_last_item(&mut self) {
        let len = self.current_list_len();

        if len > 0 {
            self.current_list_state().select(Some(len - 1));
            self.reload_data();
        }
    }

    pub fn jump_to_random(&mut self) {
        let len = self.current_list_len();

        if len > 0 {
            let random_index = rand::thread_rng().gen_range(0..len);
            self.current_list_state().select(Some(random_index));
            self.reload_data();
        }
    }

    pub fn open_list(&mut self) {
        self.navigation.view = View::Items;
    }

    pub fn open_file(&mut self) {
        if self.navigation.view == View::Lists {
            return self.open_list();
        }

        if let Err(e) = self.download_rom() {
            eprintln!("error downloading item: {}", e);
        }
    }

    pub fn go_back(&mut self) {
        if self.navigation.view == View::Lists {
            return;
        }

        let list = self.current_list_name();

        if let Some(item_selected) = self.ui_state.items_in_list.selected() {
            self.data
                .items_in_list_selections
                .insert(list.into(), item_selected);
        }

        self.navigation.view = View::Lists;
        self.save_item_selected();
        self.reload_data();
    }
}
