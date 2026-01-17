use serde::Deserialize;

use crate::app::App;

#[derive(Deserialize, Debug, Clone)]
pub struct Command {
    pub name: String,
    pub command: String,
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
}
