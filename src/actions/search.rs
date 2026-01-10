use crate::app::App;

impl App {
    pub fn start_search(&mut self) {
        self.in_search_mode = true;
        self.search_query.clear();
        self.search_results.clear();
        self.search_selected = 0;
        self.update_search_results();
    }

    pub fn stop_search(&mut self) {
        if let Some(original_index) = self.get_current_search_index() {
            self.selected = original_index;
        }
        self.in_search_mode = false;
        self.search_query.clear();
        self.search_results.clear();
    }

    pub fn add_search_char(&mut self, c: char) {
        self.search_query.push(c);
        self.update_search_results();
    }

    pub fn remove_search_char(&mut self) {
        self.search_query.pop();
        self.update_search_results();
    }

    fn update_search_results(&mut self) {
        if self.search_query.is_empty() {
            // When search query is empty, show all items (no filtering)
            if self.in_system {
                // Show all ROMs
                self.search_results = (0..self.roms.len()).collect();
            } else {
                // Show all systems
                self.search_results = (0..self.entries.len()).collect();
            }
            return;
        }

        let query_lower = self.search_query.to_lowercase();

        if self.in_system {
            // Search in ROMs
            self.search_results = self
                .roms
                .iter()
                .enumerate()
                .filter(|(_, rom)| rom.title.to_lowercase().contains(&query_lower))
                .map(|(index, _)| index)
                .collect();
        } else {
            // Search in systems (entries)
            self.search_results = self
                .entries
                .iter()
                .enumerate()
                .filter(|(_, path)| {
                    path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query_lower)
                })
                .map(|(index, _)| index)
                .collect();
        }

        // Reset search selection
        self.search_selected = 0;
    }

    pub fn get_current_search_index(&self) -> Option<usize> {
        self.search_results.get(self.search_selected).copied()
    }

    pub fn search_up(&mut self) {
        if !self.search_results.is_empty() {
            self.search_selected = if self.search_selected == 0 {
                self.search_results.len() - 1
            } else {
                self.search_selected - 1
            };
        }
    }

    pub fn search_down(&mut self) {
        if !self.search_results.is_empty() {
            self.search_selected = (self.search_selected + 1) % self.search_results.len();
        }
    }
}
