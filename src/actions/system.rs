use serde::Deserialize;

use crate::app::App;

#[derive(Deserialize, Debug, Clone)]
pub struct Command {
    pub name: String,
    pub command: String,
}

impl App {
    pub fn load_system_commands(&mut self) {
        let commands_path = Self::emux_base_path().join("system_commands.json");
        let data = std::fs::read_to_string(&commands_path).unwrap_or_default();
        self.system_commands = serde_json::from_str(&data).unwrap_or_default();
    }

    fn clean_system_name(system: &str) -> String {
        system
            .split('(')
            .next()
            .unwrap_or(system)
            .trim()
            .to_string()
    }

    pub fn get_current_commands(&self) -> Vec<Command> {
        if self.current_system.is_empty() {
            return Vec::new();
        }

        let clean_system = Self::clean_system_name(&self.current_system);

        self.system_commands
            .iter()
            .find(|sc| sc.system == clean_system)
            .map(|sc| sc.commands.clone())
            .unwrap_or_default()
    }
}
