use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
};

#[derive(Default)]
pub struct UiState {
    pub lists_state: ListState,
    pub items_in_list_state: ListState,
}

use crate::components::{body, header};
use crate::{app::App, components::footer};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Max(7),
        ])
        .split(frame.area());

    let list_area = vertical[1];

    body::render_body(frame, app, list_area);
    header::render_header(frame, app, vertical[0]);
    footer::render_footer(frame, app, vertical[2]);
}
