use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
};

use crate::{app::App, components::inputs::search::render_search_input};

pub fn render_status_bar(app: &App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Min(0)])
        .split(area);

    render_search_input(app, frame, chunks[0]);

    let label_size = Span::styled(
        " 50 MBs ",
        Style::new()
            .bg(Color::Rgb(49, 50, 68))
            .fg(Color::Rgb(180, 190, 254)),
    );

    let icon_size = Span::styled(
        " ",
        Style::new()
            .bg(Color::Rgb(148, 226, 213))
            .fg(Color::Rgb(24, 24, 37)),
    );

    let circle_size = Span::styled("", Style::new().fg(Color::Rgb(148, 226, 213)));

    let circle_buf = Span::styled(
        "",
        Style::new()
            .bg(Color::Rgb(49, 50, 68))
            .fg(Color::Rgb(249, 226, 175)),
    );

    let label_buf = Span::raw("  Lists  ").style(
        Style::default()
            .bg(Color::Rgb(249, 226, 175))
            .fg(Color::Rgb(24, 24, 37)),
    );

    let line = Line::from(vec![
        circle_size,
        icon_size,
        label_size,
        circle_buf,
        label_buf,
    ])
    .style(Style::new().bg(Color::Rgb(30, 30, 46)))
    .right_aligned();

    frame.render_widget(line, chunks[1]);
}
