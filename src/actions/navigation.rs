use rand::Rng;
use serde::Deserialize;
use std::path::PathBuf;

use crate::app::App;

#[derive(Deserialize, Debug, Clone)]
pub struct RomEntry {
    pub title: String,
    pub url: String,
}

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

    pub fn open_folder(&mut self) {
        if self.in_list {
            return;
        }

        if let Some(path) = self.entries.get(self.selected).cloned() {
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                self.enter_to_list(path);
                return;
            }
        }
    }

    pub fn open_file_folder(&mut self) {
        if !self.in_list {
            return self.open_folder();
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
            self.current_list.clear();
            self.selected_command = 0;
            self.in_command_selection = false;
            self.roms_scroll_offset = 0;

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

    fn enter_to_list(&mut self, path: PathBuf) {
        self.load_list();

        self.current_list = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        self.in_list = true;

        self.selected = self
            .list_selections
            .get(&self.current_list)
            .copied()
            .unwrap_or(0);
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

    pub fn load_list(&mut self) {
        let path = match (!self.in_list, self.entries.get(self.selected).cloned()) {
            (true, Some(p)) if p.extension().and_then(|s| s.to_str()) == Some("json") => Some(p),
            _ => None,
        };

        let Some(path) = path else {
            return;
        };

        let path_str = self.current_path.to_string_lossy().to_string();
        self.directory_selections
            .insert(path_str.clone(), self.selected);
        self.directory_scroll_selections
            .insert(path_str, self.scroll_offset);

        let data = std::fs::read_to_string(&path).unwrap_or_default();

        match serde_json::from_str::<Vec<RomEntry>>(&data) {
            Ok(roms) => {
                self.roms = roms;
            }
            Err(e) => {
                eprintln!("Error parsing JSON for {}: {}", path.display(), e);
                self.roms = Vec::new();
            }
        }
    }
}
