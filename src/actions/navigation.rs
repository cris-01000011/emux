use std::path::PathBuf;

use rand::Rng;
use ratatui::widgets::ListState;

use crate::{app::App, components::input::InputActive};

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq)]
pub enum View {
    #[default]
    Lists,
    Items,
}

#[derive(Default)]
pub struct Navigation {
    pub view: View,
    pub current_list_path: Option<PathBuf>,
}

impl App {
    fn current_list_state(&mut self) -> &mut ListState {
        match self.navigation.view {
            View::Lists => &mut self.ui.lists,
            View::Items => &mut self.ui.items_in_list,
        }
    }

    pub fn update_selected_list_from_search(&mut self) {
        // When searching in Lists, need to update the actual selected list
        if self.navigation.view == View::Lists
            && self.ui.input.active == InputActive::Search
            && !self.ui.input.search.value().is_empty()
        {
            if let Some(selected_index) = self.ui.lists.selected() {
                if let Some(&real_list_index) = self.search.lists_query.get(selected_index) {
                    // Update the actual selected list based on search results
                    self.commands.selected_list = real_list_index; // Update for command context
                    // Update current list path
                    if let Some(path) = self.data.lists.get(real_list_index) {
                        self.navigation.current_list_path = Some(path.clone());
                    }
                    // Update saved selection to restore after search ends
                    self.search.saved_list_selection = Some(real_list_index);
                }
            }
        }

        // Also update saved selection for items when searching in Items
        if self.navigation.view == View::Items
            && self.ui.input.active == InputActive::Search
            && !self.ui.input.search.value().is_empty()
        {
            if let Some(selected_index) = self.ui.items_in_list.selected() {
                if let Some(&real_item_index) = self.search.items_query.get(selected_index) {
                    self.search.saved_item_selection = Some(real_item_index);
                }
            }
        }
    }

    fn current_list_len(&self) -> usize {
        match self.navigation.view {
            View::Lists => {
                if self.ui.input.active == InputActive::Search
                    && !self.ui.input.search.value().is_empty()
                {
                    self.search.lists_query.len()
                } else {
                    self.data.lists.len()
                }
            }
            View::Items => {
                if self.ui.input.active == InputActive::Search
                    && !self.ui.input.search.value().is_empty()
                {
                    self.search.items_query.len()
                } else {
                    self.get_current_list_items_count()
                }
            }
        }
    }

    pub fn move_up(&mut self) {
        self.current_list_state().select_previous();

        self.update_selected_list_from_search();

        if self.navigation.view == View::Lists {
            self.reload_data();
        }
    }

    pub fn move_down(&mut self) {
        self.current_list_state().select_next();

        self.update_selected_list_from_search();

        if self.navigation.view == View::Lists {
            self.reload_data();
        }
    }

    pub fn go_to_first_item(&mut self) {
        self.current_list_state().select_first();

        self.update_selected_list_from_search();

        if self.navigation.view == View::Lists {
            self.reload_data();
        }
    }

    pub fn go_to_last_item(&mut self) {
        let len = self.current_list_len();

        if len > 0 {
            self.current_list_state().select(Some(len - 1));

            self.update_selected_list_from_search();

            if self.navigation.view == View::Lists {
                self.reload_data();
            }
        }
    }

    pub fn jump_to_random(&mut self) {
        let len = self.current_list_len();

        if len > 0 {
            let random_index = rand::thread_rng().gen_range(0..len);
            self.current_list_state().select(Some(random_index));

            self.update_selected_list_from_search();

            if self.navigation.view == View::Lists {
                self.reload_data();
            }
        }
    }

    pub fn open_list(&mut self) {
        self.navigation.view = View::Items;
        self.load_current_commands();
    }

    pub fn open_file(&mut self) {
        if self.navigation.view == View::Lists {
            self.open_list();
            return;
        }

        self.download_rom()
    }

    pub fn go_back(&mut self) {
        if self.navigation.view == View::Lists {
            return;
        }

        let list = self.current_list_name();

        if let Some(item_selected) = self.ui.items_in_list.selected() {
            self.data
                .items_in_list_selections
                .insert(list.into(), item_selected);
        }

        self.navigation.view = View::Lists;
        self.save_item_selected();
        self.reload_data();
    }
}
