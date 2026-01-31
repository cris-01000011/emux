use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
};
use tui_input::Input;

use crate::{actions::navigation::View, app::App};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Default)]
pub struct SearchState {
    pub input: Input,
    pub mode: InputMode,
    // Search results for different contexts
    pub lists_query: Vec<usize>, // Indices of matching lists
    pub items_query: Vec<usize>, // Indices of matching items
    // Remember selected positions to restore after search
    pub saved_list_selection: Option<usize>,
    pub saved_item_selection: Option<usize>,
}

pub fn render_input(app: &App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(area);

    let label = Paragraph::new("    ").style(
        Style::default()
            .bg(Color::Rgb(116, 199, 236))
            .fg(Color::Rgb(24, 24, 37)),
    );
    frame.render_widget(label, chunks[0]);

    let input_value = app.ui.search.input.value();
    let width = chunks[2].width.max(1);
    let scroll = app.ui.search.input.visual_scroll(width as usize);

    let input = Paragraph::new(input_value)
        .style(
            Style::default()
                .bg(Color::Rgb(30, 30, 46))
                .fg(match app.ui.search.mode {
                    InputMode::Normal => Color::Rgb(137, 180, 250),
                    InputMode::Editing => Color::Rgb(180, 190, 254),
                }),
        )
        .scroll((0, scroll as u16));

    frame.render_widget(input, chunks[2]);

    if app.ui.search.mode == InputMode::Editing {
        let cursor_x = (app.ui.search.input.visual_cursor().max(scroll) - scroll) as u16;
        frame.set_cursor_position((chunks[2].x + cursor_x, chunks[2].y));
    }
}

impl SearchState {
    pub fn clear(&mut self) {
        self.input.reset();
        self.lists_query.clear();
        self.items_query.clear();
        self.saved_list_selection = None;
        self.saved_item_selection = None;
    }
}

impl App {
    pub fn start_search(&mut self) {
        self.ui.search.mode = InputMode::Editing;

        match self.navigation.view {
            View::Lists => {
                self.ui.search.saved_list_selection = self.ui.lists.selected();
            }
            View::Items => {
                self.ui.search.saved_item_selection = self.ui.items_in_list.selected();
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
        self.ui.search.mode = InputMode::Normal;

        self.restore_selection();
    }

    fn restore_selection(&mut self) {
        match self.navigation.view {
            View::Lists => {
                if let Some(saved) = self.ui.search.saved_list_selection {
                    self.ui.search.clear();
                    self.ui.lists.select(Some(saved));
                    self.reload_data();
                }
            }
            View::Items => {
                if let Some(saved) = self.ui.search.saved_item_selection {
                    self.ui.search.clear();
                    self.reload_data();
                    self.ui.items_in_list.select(Some(saved));
                }
            }
        }
    }

    pub fn update_search(&mut self) {
        if self.ui.search.mode == InputMode::Editing {
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
