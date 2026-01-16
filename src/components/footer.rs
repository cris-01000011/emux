use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::{Block, BorderType, Borders},
};

use crate::app::App;

pub fn render_footer(frame: &mut Frame, _app: &App, area: Rect) {
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left_panel = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Color::Rgb(137, 180, 250));

    frame.render_widget(left_panel, horizontal[0]);

    let right_panel = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Color::Rgb(137, 180, 250));

    frame.render_widget(right_panel, horizontal[1]);
}
