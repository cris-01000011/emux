use rand::Rng;
use ratatui::widgets::ListState;

use crate::app::App;

impl App {
    fn current_list_state(&mut self) -> &mut ListState {
        if self.in_list {
            &mut self.items_in_list_state
        } else {
            &mut self.lists_state
        }
    }

    fn current_list_len(&self) -> usize {
        if self.in_list {
            self.items_in_list.len()
        } else {
            self.lists.len()
        }
    }

    pub fn move_up(&mut self) {
        self.current_list_state().select_previous();
        self.load_list();
    }

    pub fn move_down(&mut self) {
        self.current_list_state().select_next();
        self.load_list();
    }

    pub fn go_to_first_item(&mut self) {
        self.current_list_state().select_first();
        self.load_list();
    }

    pub fn go_to_last_item(&mut self) {
        let len = self.current_list_len();

        if len > 0 {
            self.current_list_state().select(Some(len - 1));
            self.load_list();
        }
    }

    pub fn jump_to_random(&mut self) {
        let len = self.current_list_len();

        if len > 0 {
            let random_index = rand::thread_rng().gen_range(0..len);
            self.current_list_state().select(Some(random_index));
            self.load_list();
        }
    }

    pub fn open_list(&mut self) {
        self.in_list = true;
    }

    pub fn open_file(&mut self) {
        if !self.in_list {
            return self.open_list();
        }

        if let Err(e) = self.download_rom() {
            eprintln!("error downloading item: {}", e);
        }
    }

    pub fn go_back(&mut self) {
        if !self.in_list {
            return;
        }

        if let Some(item_selected) = self.items_in_list_state.selected() {
            self.list_selections
                .insert(self.current_list.clone(), item_selected);
        }

        self.in_list = false;
    }
}
