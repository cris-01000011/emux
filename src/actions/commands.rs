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
    // Flat vector with ALL commands from all lists
    pub commands: Vec<Command>,
    // Start/end indices for each list in the flat commands vector
    pub list_ranges: Vec<(usize, usize)>, // (start, end) for each list
}

impl CommandLists {
    pub fn build_flat_commands(&mut self) {
        self.commands.clear();
        self.list_ranges.clear();

        let mut current_start = 0;
        for list in &self.lists {
            let list_len = list.commands.len();
            self.list_ranges
                .push((current_start, current_start + list_len));
            self.commands.extend_from_slice(&list.commands);
            current_start += list_len;
        }
    }

    pub fn get_current_list_range(&self) -> (usize, usize) {
        self.list_ranges
            .get(self.selected_list)
            .copied()
            .unwrap_or((0, 0))
    }

    pub fn get_current_list_commands_count(&self) -> usize {
        let (start, end) = self.get_current_list_range();
        end - start
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

        // Build flat commands vector once during initialization
        self.commands.build_flat_commands();
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

        // Reset selected command when switching lists
        self.commands.selected_command = 0;
    }
}
