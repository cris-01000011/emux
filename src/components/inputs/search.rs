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
    }
}

impl App {
    pub fn start_search(&mut self) {
        self.ui.search.mode = InputMode::Editing;

        match self.navigation.view {
            View::Lists => self.ui.lists.select_first(),
            View::Items => self.ui.items_in_list.select_first(),
        };
    }

    pub fn stop_search(&mut self) {
        self.ui.search.mode = InputMode::Normal;
        self.ui.search.clear();
        self.reload_data();
    }
}
