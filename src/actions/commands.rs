use serde::Deserialize;

use crate::{actions::navigation::View, app::App, utils::string::cut_at};

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
    pub cached_commands: Vec<(usize, usize)>,
    pub current_list_start: usize,
    pub current_list_end: usize,
}

impl CommandLists {
    pub fn get_current_commands(&self) -> &[Command] {
        self.lists
            .get(self.selected_list)
            .map(|l| l.commands.as_slice())
            .unwrap_or(&[])
    }

    pub fn build_cache(&mut self) {
        self.cached_commands.clear();

        for (list_idx, list) in self.lists.iter().enumerate() {
            for cmd_idx in 0..list.commands.len() {
                self.cached_commands.push((list_idx, cmd_idx));
            }
        }
    }

    pub fn update_current_list_range(&mut self) {
        if self.selected_list < self.lists.len() {
            let mut start = 0;
            let mut end = 0;

            for (list_idx, list) in self.lists.iter().enumerate() {
                if list_idx == self.selected_list {
                    start = end;
                    end += list.commands.len();
                    break;
                }
                end += list.commands.len();
            }

            self.current_list_start = start;
            self.current_list_end = end;
            self.selected_command = 0; // Reset to first command when switching lists
        }
    }

    pub fn get_current_list_commands_count(&self) -> usize {
        self.current_list_end - self.current_list_start
    }
}

impl App {
    pub fn next_command(&mut self) {
        if self.navigation.view != View::Items {
            return;
        }

        let command_count = self.commands.get_current_list_commands_count();
        if command_count > 0 {
            self.commands.selected_command = (self.commands.selected_command + 1) % command_count;
        }
    }

    pub fn prev_command(&mut self) {
        if self.navigation.view != View::Items {
            return;
        }

        let command_count = self.commands.get_current_list_commands_count();
        if command_count > 0 {
            self.commands.selected_command = if self.commands.selected_command == 0 {
                command_count - 1
            } else {
                self.commands.selected_command - 1
            };
        }
    }

    pub fn init_command_lists(&mut self) {
        let commands_path = self.config.base_dir.join("lists_commands.json");
        let command_lists = std::fs::read_to_string(&commands_path).unwrap_or_default();
        self.commands.lists = serde_json::from_str(&command_lists).unwrap_or_default();

        // Build the cache once during initialization
        self.commands.build_cache();
        self.load_current_commands();
    }

    pub fn load_current_commands(&mut self) {
        let list = self.current_list_name();
        let clean_list = cut_at(list, '(');

        self.commands.selected_list = self
            .commands
            .lists
            .iter()
            .position(|lc| lc.list == clean_list)
            .unwrap_or(0);

        // Update the range for the current list instead of recalculating each time
        self.commands.update_current_list_range();
    }
}
