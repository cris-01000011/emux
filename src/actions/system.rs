use serde::Deserialize;

use crate::app::App;

#[derive(Deserialize, Debug, Clone)]
pub struct Command {
    pub name: String,
    pub command: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListItem {
    pub title: String,
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
        let path = match (!self.in_list, self.entries.get(self.selected).cloned()) {
            (true, Some(p)) if p.extension().and_then(|s| s.to_str()) == Some("json") => Some(p),
            _ => None,
        };

        let Some(path) = path else {
            return;
        };

        self.current_list = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let path_str = self.current_path.to_string_lossy().to_string();
        self.directory_selections
            .insert(path_str.clone(), self.selected);
        self.directory_scroll_selections
            .insert(path_str, self.scroll_offset);

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
    }
}
