use crate::app::App;

impl App {
    pub fn start_search(&mut self) {
        self.in_search_mode = true;
        self.search_query.clear();

        if self.in_list {
            self.items_in_list_state.select_first();
        } else {
            self.lists_state.select_first();
        }
    }

    pub fn stop_search(&mut self) {
        self.in_search_mode = false;
        self.search_query.clear();
        self.load_list();
    }

    pub fn add_search_char(&mut self, c: char) {
        self.search_query.push(c);
        self.load_list();
    }

    pub fn remove_search_char(&mut self) {
        self.search_query.pop();
        self.load_list();
    }
}
