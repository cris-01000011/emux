use serde::Deserialize;

use crate::app::App;

#[derive(Deserialize, Debug, Clone)]
pub struct Command {
    pub name: String,
    pub command: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListItem {
    pub item: String,
    pub url: String,
}

impl App {
    pub fn load_lists_commands(&mut self) {
        let commands_path = Self::emux_base_path().join("lists_commands.json");
        let data = std::fs::read_to_string(&commands_path).unwrap_or_default();
        self.lists_commands = serde_json::from_str(&data).unwrap_or_default();
    }

    fn clean_list_name(list: &str) -> String {
        list.split('(').next().unwrap_or(list).trim().to_string()
    }

    pub fn get_current_commands(&self) -> Vec<Command> {
        if self.current_list.is_empty() {
            return Vec::new();
        }

        let clean_list = Self::clean_list_name(&self.current_list);

        self.lists_commands
            .iter()
            .find(|sc| sc.list == clean_list)
            .map(|sc| sc.commands.clone())
            .unwrap_or_default()
    }

    pub fn load_list(&mut self) {
        if self.in_list && self.favorites_mode {
            let current_list = self.current_list.clone();
            let favorites = self.favorites.clone();
            self.roms.retain(|item| {
                favorites
                    .iter()
                    .any(|f| f.list == current_list && f.item == item.item)
            });

            // Adjust items selection if needed
            if let Some(selected) = self.items_list_state.selected() {
                if selected >= self.roms.len() && !self.roms.is_empty() {
                    self.items_list_state
                        .select(Some(self.roms.len().saturating_sub(1)));
                }
            }

            return;
        } else if self.in_list && !self.favorites_mode {
            let path = self
                .current_path
                .join(format!("{}.json", self.current_list));
            let data = std::fs::read_to_string(&path).unwrap_or_default();

            match serde_json::from_str::<Vec<ListItem>>(&data) {
                Ok(roms) => {
                    self.roms = roms;
                }
                Err(e) => {
                    eprintln!("Error parsing JSON for {}: {}", path.display(), e);
                    self.roms = Vec::new();
                }
            }
            return;
        }

        let path = match self
            .entries
            .get(self.directory_list_state.selected().unwrap_or(0))
            .cloned()
        {
            Some(p) if p.extension().and_then(|s| s.to_str()) == Some("json") => p,
            _ => return,
        };

        self.current_list = path
            .file_stem()
            .and_then(|s: &std::ffi::OsStr| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let path_str = self.current_path.to_string_lossy().to_string();
        if let Some(selected) = self.directory_list_state.selected() {
            self.directory_selections.insert(path_str.clone(), selected);
        }

        let data = std::fs::read_to_string(&path).unwrap_or_default();

        match serde_json::from_str::<Vec<ListItem>>(&data) {
            Ok(mut roms) => {
                if self.favorites_mode {
                    roms.retain(|item| self.is_favorite(&self.current_list, &item.item));
                }
                self.roms = roms;
            }
            Err(e) => {
                eprintln!("Error parsing JSON for {}: {}", path.display(), e);
                self.roms = Vec::new();
            }
        }
    }

    pub fn toggle_favorites_mode(&mut self) {
        self.favorites_mode = !self.favorites_mode;
        self.load_list();
    }
}
