use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
};

use crate::components::{body, header, inputs::search::SearchState, popup::Popup};
use crate::{app::App, components::footer};

#[derive(Default)]
pub struct UiState {
    pub lists: ListState,
    pub items_in_list: ListState,
    pub search: SearchState,
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

    body::render_body(frame, app, list_area);
    header::render_header(frame, app, vertical[0]);
    footer::render_footer(frame, app, vertical[2]);
    app.ui.popup.render_popup(frame, app);
}
