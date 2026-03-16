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
                self.search.saved_item_selection = Some(self.scroll.index_in_list());
            }
        }

        match self.navigation.view {
            View::Lists => {
                self.search_lists();
                self.ui.lists.select_first();
            }
            View::Items => {
                self.search_items();
                self.scroll.select_first();
            }
        };
    }

    pub fn stop_search(&mut self) {
        self.ui.input.active = InputActive::None;

        let items = self.get_current_list_items_slice();
        self.scroll.total = items.len();

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
                    if saved < self.scroll.visible {
                        self.scroll.start = 0;
                        self.scroll.end = self.scroll.visible;
                        self.scroll.selected = saved;
                        return;
                    }

                    if saved > self.scroll.total - self.scroll.visible {
                        self.scroll.start = self.scroll.total - self.scroll.visible;
                        self.scroll.end = self.scroll.total;
                        self.scroll.selected = saved - self.scroll.start;
                        return;
                    }

                    self.scroll.start = (saved - self.scroll.visible) + self.scroll.visible / 2;
                    self.scroll.end = saved + self.scroll.visible / 2;
                    self.scroll.selected = self.scroll.end - saved + (self.scroll.visible % 2);
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
                    self.scroll.select_first();
                    self.update_selected_list_from_search();
                    self.reload_data();
                }
                View::Items => {
                    self.search_items();
                    self.scroll.select_first();
                    self.update_selected_list_from_search();
                }
            };
        }
    }
}
