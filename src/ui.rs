use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
};

use crate::app::App;
use crate::components::{
    body::render_body,
    footer::render_footer,
    input::Inputs,
    popup::{Popup, render_popup},
};

#[derive(Default)]
pub struct UiState {
    pub lists: ListState,
    pub items_in_list: ListState,
    pub input: Inputs,
    pub popup: Popup,
}

pub fn render_ui(frame: &mut Frame, app: &mut App) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(0),
            Constraint::Min(0),
            Constraint::Max(0),
        ])
        .split(frame.area());

    let list_area = vertical[1];

    render_body(frame, app, list_area);
    render_footer(frame, app, vertical[2]);
    render_popup(frame, app);
}
