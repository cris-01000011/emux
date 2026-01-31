use crate::{actions::navigation::View, app::App, components::input::InputActive};

#[derive(Default)]
pub struct Search {
    pub lists_query: Vec<usize>,
    pub items_query: Vec<usize>,
    pub saved_list_selection: Option<usize>,
    pub saved_item_selection: Option<usize>,
}

impl App {
    pub fn clear_search(&mut self) {
        self.ui.input.search.reset();
        self.search.lists_query.clear();
        self.search.items_query.clear();
        self.search.saved_list_selection = None;
        self.search.saved_item_selection = None;
    }

    pub fn start_search(&mut self) {
        self.ui.input.active = InputActive::Search;

        match self.navigation.view {
            View::Lists => {
                self.search.saved_list_selection = self.ui.lists.selected();
            }
            View::Items => {
                self.search.saved_item_selection = self.ui.items_in_list.selected();
            }
        }

        match self.navigation.view {
            View::Lists => {
                self.search_lists();
                self.ui.lists.select_first();
            }
            View::Items => {
                self.search_items();
                self.ui.items_in_list.select_first();
            }
        };
    }

    pub fn stop_search(&mut self) {
        self.ui.input.active = InputActive::None;

        self.restore_selection();
    }

    fn restore_selection(&mut self) {
        match self.navigation.view {
            View::Lists => {
                if let Some(saved) = self.search.saved_list_selection {
                    self.clear_search();
                    self.ui.lists.select(Some(saved));
                    self.reload_data();
                }
            }
            View::Items => {
                if let Some(saved) = self.search.saved_item_selection {
                    self.clear_search();
                    self.reload_data();
                    self.ui.items_in_list.select(Some(saved));
                }
            }
        }
    }

    pub fn update_search(&mut self) {
        if self.ui.input.active == InputActive::Search {
            match self.navigation.view {
                View::Lists => {
                    self.search_lists();
                    self.ui.lists.select_first();
                    self.ui.items_in_list.select_first();
                    self.update_selected_list_from_search();
                    self.reload_data();
                }
                View::Items => {
                    self.search_items();
                    self.ui.items_in_list.select_first();
                    self.update_selected_list_from_search();
                }
            };
        }
    }
}
