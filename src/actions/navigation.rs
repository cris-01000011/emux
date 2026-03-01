use std::path::PathBuf;

use rand::Rng;

use crate::{app::App, components::input::InputActive};

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq)]
pub enum ListsView {
    #[default]
    Lists,
    LocalLists,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq)]
pub enum View {
    #[default]
    Lists,
    Items,
}

#[derive(Default)]
pub struct Navigation {
    pub list_view: ListsView,
    pub view: View,
    pub current_list_path: Option<PathBuf>,
}

impl Navigation {
    pub fn next_view(&mut self) {
        match self.list_view {
            ListsView::Lists => self.list_view = ListsView::LocalLists,
            ListsView::LocalLists => self.list_view = ListsView::Lists,
        }
    }

    pub fn prev_view(&mut self) {
        match self.list_view {
            ListsView::Lists => self.list_view = ListsView::LocalLists,
            ListsView::LocalLists => self.list_view = ListsView::Lists,
        }
    }
}

impl App {
    pub fn update_selected_list_from_search(&mut self) {
        if self.navigation.view == View::Lists
            && self.ui.input.active == InputActive::Search
            && !self.ui.input.search.value().is_empty()
            && let Some(selected_index) = self.ui.lists.selected()
            && let Some(&real_list_index) = self.search.lists_query.get(selected_index)
        {
            self.commands.selected_list = real_list_index;

            if let Some(path) = self.data.lists.get(real_list_index) {
                self.navigation.current_list_path = Some(path.clone());
            }

            self.search.saved_list_selection = Some(real_list_index);
        }

        if self.navigation.view == View::Items
            && self.ui.input.active == InputActive::Search
            && !self.ui.input.search.value().is_empty()
        {
            let selected_index = self.scroll.index_in_list();
            if let Some(&real_item_index) = self.search.items_query.get(selected_index) {
                self.search.saved_item_selection = Some(real_item_index);
            }
        }
    }

    pub fn move_up(&mut self) {
        match self.navigation.view {
            View::Lists => self.ui.lists.select_previous(),
            View::Items => self.scroll.move_up(),
        }

        self.update_selected_list_from_search();

        if self.navigation.view == View::Lists {
            self.reload_data();
        }
    }

    pub fn move_down(&mut self) {
        match self.navigation.view {
            View::Lists => self.ui.lists.select_next(),
            View::Items => self.scroll.move_down(),
        }

        self.update_selected_list_from_search();

        if self.navigation.view == View::Lists {
            self.reload_data();
        }
    }

    pub fn go_to_first_item(&mut self) {
        match self.navigation.view {
            View::Lists => self.ui.lists.select_first(),
            View::Items => self.scroll.select_first(),
        }

        self.update_selected_list_from_search();

        if self.navigation.view == View::Lists {
            self.reload_data();
        }
    }

    pub fn go_to_last_item(&mut self) {
        let len = self.data.lists.len();

        if len > 0 {
            match self.navigation.view {
                View::Lists => self.ui.lists.select(Some(len - 1)),
                View::Items => self.scroll.select_last(),
            }

            self.update_selected_list_from_search();

            if self.navigation.view == View::Lists {
                self.reload_data();
            }
        }
    }

    pub fn jump_to_random(&mut self) {
        let len = self.data.lists.len();

        if len > 0 {
            let random_index = rand::thread_rng().gen_range(0..len);
            match self.navigation.view {
                View::Lists => self.ui.lists.select(Some(random_index)),
                View::Items => self.scroll.select_random(),
            }

            self.update_selected_list_from_search();

            if self.navigation.view == View::Lists {
                self.reload_data();
            }
        }
    }

    pub fn open_list(&mut self) {
        self.navigation.view = View::Items;
        let items = self.get_current_list_items();
        self.scroll.total = items.len();
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

        self.scroll.select_first();
        self.scroll.total = 0;

        self.navigation.view = View::Lists;
        self.reload_data();
    }
}
