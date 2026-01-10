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
            self.selected = if self.in_system {
                self.roms.len().saturating_sub(1)
            } else {
                self.entries.len().saturating_sub(1)
            };
        }
    }

    pub fn move_down(&mut self) {
        if self.in_system {
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
    }

    pub fn go_to_first_item(&mut self) {
        self.selected = 0;
    }

    pub fn go_to_last_item(&mut self) {
        if self.in_system {
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
        if self.in_system {
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
        if self.in_system {
            return;
        }

        if let Some(path) = self.entries.get(self.selected).cloned() {
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                self.load_json_system(path);
                return;
            }
        }
    }

    pub fn open_file_folder(&mut self) {
        if !self.in_system {
            return self.open_folder();
        }

        if let Err(e) = self.download_rom() {
            eprintln!("Error downloading ROM: {}", e);
        }
    }

    pub fn go_back(&mut self) {
        if self.in_system {
            // Save current selections and scroll before leaving system
            self.system_selections
                .insert(self.current_system.clone(), self.selected);
            self.command_selections
                .insert(self.current_system.clone(), self.selected_command);
            self.system_scroll_selections
                .insert(self.current_system.clone(), self.roms_scroll_offset);

            // return to system list
            self.in_system = false;
            self.roms.clear();
            self.current_system.clear();
            self.selected_command = 0;
            self.in_command_selection = false;
            self.roms_scroll_offset = 0; // Reset ROM scroll offset

            // Restore directory selection for this path
            let path_str = self.current_path.to_string_lossy().to_string();
            self.selected = self
                .directory_selections
                .get(&path_str)
                .copied()
                .unwrap_or(0);
            return;
        }

        if self.current_path == Self::games_path() {
            return;
        }

        if let Some(parent) = self.current_path.parent() {
            if parent.starts_with(Self::games_path()) {
                self.load_dir(parent.to_path_buf());
            }
        }
    }

    fn load_json_system(&mut self, path: PathBuf) {
        // Save current directory selection and scroll before entering system
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

        // Extract system name from filename (handle quotes properly)
        self.current_system = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        self.in_system = true;

        // Restore saved selections for this system
        self.selected = self
            .system_selections
            .get(&self.current_system)
            .copied()
            .unwrap_or(0);
        self.selected_command = self
            .command_selections
            .get(&self.current_system)
            .copied()
            .unwrap_or(0);
        self.in_command_selection = true; // Auto-select first command

        // Restore saved ROM scroll offset for this system, default to 0 if not found
        self.roms_scroll_offset = self
            .system_scroll_selections
            .get(&self.current_system)
            .copied()
            .unwrap_or(0);
    }
}
