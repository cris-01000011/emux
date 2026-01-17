use rand::Rng;

use crate::app::App;

impl App {
    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = if self.in_list {
                self.roms.len().saturating_sub(1)
            } else {
                self.entries.len().saturating_sub(1)
            };
        }

        self.load_list();
    }

    pub fn move_down(&mut self) {
        if self.in_list {
            if self.selected + 1 < self.roms.len() {
                self.selected += 1;
            } else {
                self.selected = 0;
            }
        } else if self.selected + 1 < self.entries.len() {
            self.selected += 1;
        } else {
            self.selected = 0;
        }

        self.load_list();
    }

    pub fn go_to_first_item(&mut self) {
        self.selected = 0;
    }

    pub fn go_to_last_item(&mut self) {
        if self.in_list {
            if !self.roms.is_empty() {
                self.selected = self.roms.len().saturating_sub(1);
            }
        } else {
            if !self.entries.is_empty() {
                self.selected = self.entries.len().saturating_sub(1);
            }
        }
    }

    pub fn jump_to_random(&mut self) {
        if self.in_list {
            if !self.roms.is_empty() {
                self.selected = rand::thread_rng().gen_range(0..self.roms.len());
            }
        } else {
            if !self.entries.is_empty() {
                self.selected = rand::thread_rng().gen_range(0..self.entries.len());
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
            self.list_selections
                .insert(self.current_list.clone(), self.selected);
            self.command_selections
                .insert(self.current_list.clone(), self.selected_command);
            self.list_scroll_selections
                .insert(self.current_list.clone(), self.roms_scroll_offset);

            self.in_list = false;
            self.selected_command = 0;
            self.in_command_selection = false;
            self.roms_scroll_offset = self
                .list_scroll_selections
                .get(&self.current_list)
                .copied()
                .unwrap_or(0);

            let path_str = self.current_path.to_string_lossy().to_string();
            self.selected = self
                .directory_selections
                .get(&path_str)
                .copied()
                .unwrap_or(0);

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

        self.load_list();

        self.in_list = true;

        let index_item_selected = self
            .list_selections
            .get(&self.current_list)
            .copied()
            .unwrap_or(0);

        if !self.favorites_mode
            || (self.favorites_mode && index_item_selected <= self.roms.len() - 1)
        {
            self.selected = index_item_selected;
        } else {
            self.selected = self.roms.len() - 1;
        }

        self.selected_command = self
            .command_selections
            .get(&self.current_list)
            .copied()
            .unwrap_or(0);

        self.in_command_selection = true;

        self.roms_scroll_offset = self
            .list_scroll_selections
            .get(&self.current_list)
            .copied()
            .unwrap_or(0);
    }
}
