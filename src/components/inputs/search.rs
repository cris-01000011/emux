use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{app::App, components::input::InputActive};

pub fn render_search_input(app: &App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(9), Constraint::Min(0)])
        .split(area);

    let label = Span::raw("    ").style(
        Style::default()
            .bg(Color::Rgb(116, 199, 236))
            .fg(Color::Rgb(24, 24, 37)),
    );

    let icon_label = Span::styled(
        " ",
        Style::new()
            .bg(Color::Rgb(69, 71, 90))
            .fg(Color::Rgb(116, 199, 236)),
    );

    let icon = Span::styled(
        " ",
        Style::new()
            .bg(Color::Rgb(30, 30, 46))
            .fg(Color::Rgb(69, 71, 90)),
    );

    let line = Line::from(vec![label, icon_label, icon]);

    frame.render_widget(line, chunks[0]);

    let input_value = app.ui.input.search.value();
    let width = chunks[1].width.max(1);
    let scroll = app.ui.input.search.visual_scroll(width as usize);

    let input = Paragraph::new(input_value)
        .style(
            Style::default()
                .bg(Color::Rgb(30, 30, 46))
                .fg(Color::Rgb(180, 190, 254)),
        )
        .scroll((0, scroll as u16));

    frame.render_widget(input, chunks[1]);

    if app.ui.input.active == InputActive::Search {
        let cursor_x = (app.ui.input.search.visual_cursor().max(scroll) - scroll) as u16;
        frame.set_cursor_position((chunks[1].x + cursor_x, chunks[1].y));
    }
}
