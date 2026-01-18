use rand::Rng;
use ratatui::widgets::ListState;

use crate::app::App;

impl App {
    fn current_state(&mut self) -> &mut ListState {
        if self.in_list {
            &mut self.items_list_state
        } else {
            &mut self.directory_list_state
        }
    }

    pub fn move_up(&mut self) {
        self.current_state().select_previous();
        self.load_list();
    }

    pub fn move_down(&mut self) {
        self.current_state().select_next();
        self.load_list();
    }

    pub fn go_to_first_item(&mut self) {
        self.current_state().select_first();
    }

    pub fn go_to_last_item(&mut self) {
        self.current_state().select_last();
    }

    pub fn jump_to_random(&mut self) {
        let items_len = if self.in_list {
            self.roms.len()
        } else {
            self.entries.len()
        };

        if items_len > 0 {
            let random_index = rand::thread_rng().gen_range(0..items_len);
            if self.in_list {
                self.items_list_state.select(Some(random_index));
            } else {
                self.directory_list_state.select(Some(random_index));
            }
        }
    }

    pub fn open_file_folder(&mut self) {
        if !self.in_list {
            return self.open_list();
        }

        if let Err(e) = self.download_rom() {
            eprintln!("Error downloading ROM: {}", e);
        }
    }

    pub fn go_back(&mut self) {
        if self.in_list {
            // Save current items selection
            if let Some(selected) = self.items_list_state.selected() {
                self.list_selections
                    .insert(self.current_list.clone(), selected);
            }
            self.command_selections
                .insert(self.current_list.clone(), self.selected_command);

            self.in_list = false;
            self.selected_command = 0;
            self.in_command_selection = false;

            // Restore directory selection
            let path_str = self.current_path.to_string_lossy().to_string();
            let saved_selected = self
                .directory_selections
                .get(&path_str)
                .copied()
                .unwrap_or(0);
            self.directory_list_state.select(Some(saved_selected));

            return;
        }

        if self.current_path == Self::lists_path() {
            return;
        }
    }

    pub fn open_list(&mut self) {
        if self.in_list {
            return;
        }

        // Save directory selection before switching
        if let Some(selected) = self.directory_list_state.selected() {
            let path_str = self.current_path.to_string_lossy().to_string();
            self.directory_selections.insert(path_str, selected);
        }

        self.load_list();

        self.in_list = true;

        let index_item_selected = self
            .list_selections
            .get(&self.current_list)
            .copied()
            .unwrap_or(0);

        let items_selected = if !self.favorites_mode
            || (self.favorites_mode && index_item_selected <= self.roms.len().saturating_sub(1))
        {
            index_item_selected
        } else {
            self.roms.len().saturating_sub(1)
        };

        self.items_list_state.select(Some(items_selected));

        self.selected_command = self
            .command_selections
            .get(&self.current_list)
            .copied()
            .unwrap_or(0);

        self.in_command_selection = true;
    }
}
