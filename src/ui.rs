use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::app::App;
use crate::components::{commands_footer, main_list, search_popup};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(frame.area());

    let list_area = vertical[0];
    let visible_height = list_area.height as usize;
    app.update_scroll_for_height(visible_height);

    main_list::render_main_list(frame, app, list_area);
    commands_footer::render_commands_footer(frame, app, vertical[1]);
    search_popup::render_search_popup(frame, app, frame.area());
}
