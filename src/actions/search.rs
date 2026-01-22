use crate::{actions::navigation::View, app::App};

#[derive(Default)]
pub struct Search {
    pub in_search: bool,
    pub search_query: String,
}

impl App {
    pub fn start_search(&mut self) {
        self.search.in_search = true;
        self.search.search_query.clear();

        match self.navigation.view {
            View::Lists => self.ui_state.lists.select_first(),
            View::Items => self.ui_state.items_in_list.select_first(),
        };
    }

    pub fn stop_search(&mut self) {
        self.search.in_search = false;
        self.search.search_query.clear();
        self.load_list();
    }

    pub fn add_search_char(&mut self, c: char) {
        self.search.search_query.push(c);
        self.load_list();
    }

    pub fn remove_search_char(&mut self) {
        self.search.search_query.pop();
        self.load_list();
    }
}
