use crate::app::App;

impl App {
    pub fn next_command(&mut self) {
        let commands = self.get_current_commands();
        if !commands.is_empty() {
            self.selected_command = (self.selected_command + 1) % commands.len();
        }
    }

    pub fn prev_command(&mut self) {
        let commands = self.get_current_commands();
        if !commands.is_empty() {
            self.selected_command = if self.selected_command == 0 {
                commands.len() - 1
            } else {
                self.selected_command - 1
            };
        }
    }
}
