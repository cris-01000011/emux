use serde::Deserialize;

use crate::{app::App, utils::string::remove_parentheses};

#[derive(Deserialize, Debug, Clone)]
pub struct Command {
    pub name: String,
    pub command: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CommandList {
    pub list: String,
    pub commands: Vec<Command>,
}

#[derive(Default)]
pub struct CommandLists {
    pub lists: Vec<CommandList>,
    pub selected_list: usize,
    pub selected_command: usize,
}

impl CommandLists {
    pub fn get_current_commands(&self) -> &[Command] {
        self.lists
            .get(self.selected_list)
            .map(|l| l.commands.as_slice())
            .unwrap_or(&[])
    }

    pub fn next_command(&mut self) {
        let command_list_len = self.get_current_commands().len();
        if command_list_len > 0 {
            self.selected_command = (self.selected_command + 1) % command_list_len;
        }
    }

    pub fn prev_command(&mut self) {
        let command_list_len = self.get_current_commands().len();
        if command_list_len > 0 {
            self.selected_command = if self.selected_command == 0 {
                command_list_len - 1
            } else {
                self.selected_command - 1
            };
        }
    }
}

impl App {
    pub fn init_command_lists(&mut self) {
        let commands_path = self.config.base_dir.join("lists_commands.json");
        let command_lists = std::fs::read_to_string(&commands_path).unwrap_or_default();
        self.commands.lists = serde_json::from_str(&command_lists).unwrap_or_default();
    }

    pub fn load_current_commands(&mut self) {
        let clean_list = remove_parentheses(&self.navigation.current_list);

        self.commands.selected_list = self
            .commands
            .lists
            .iter()
            .position(|lc| lc.list == clean_list)
            .unwrap_or(0);

        self.commands.selected_command = 0;
    }
}
