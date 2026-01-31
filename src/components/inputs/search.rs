use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
};

use crate::{app::App, components::input::InputActive};

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

    let input_value = app.ui.input.search.value();
    let width = chunks[2].width.max(1);
    let scroll = app.ui.input.search.visual_scroll(width as usize);

    let input = Paragraph::new(input_value)
        .style(Style::default().bg(Color::Rgb(30, 30, 46)).fg(
            match app.ui.input.active == InputActive::Search {
                true => Color::Rgb(180, 190, 254),
                false => Color::Rgb(137, 180, 250),
            },
        ))
        .scroll((0, scroll as u16));

    frame.render_widget(input, chunks[2]);

    if app.ui.input.active == InputActive::Search {
        let cursor_x = (app.ui.input.search.visual_cursor().max(scroll) - scroll) as u16;
        frame.set_cursor_position((chunks[2].x + cursor_x, chunks[2].y));
    }
}
