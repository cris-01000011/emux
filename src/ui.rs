use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::app::App;
use crate::components::{header, main_list};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    let list_area = vertical[1];
    let visible_height = list_area.height as usize;
    app.update_scroll_for_height(visible_height);

    main_list::render_main_list(frame, app, list_area);
    header::render_header(frame, app, vertical[0]);
}
