use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::Paragraph,
};

use crate::{app::App, components::input::InputActive};

pub fn render_new_list_url_input(app: &App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(area);

    let label = Span::raw("▍").style(
        Style::default()
            .bg(Color::Rgb(49, 50, 68))
            .fg(Color::Rgb(116, 199, 236)),
    );
    frame.render_widget(label, chunks[0]);

    let width = chunks[1].width.max(1);
    let scroll = app.ui.input.new_list_url.visual_scroll(width as usize);

    let input_value = match app.ui.input.new_list_url.value().is_empty() {
        true => "Curl",
        false => app.ui.input.new_list_url.value(),
    };

    let fg = match app.ui.input.new_list_url.value().is_empty() {
        true => Color::Rgb(166, 173, 200),
        false => Color::Rgb(180, 190, 254),
    };

    let input = Paragraph::new(input_value)
        .style(Style::default().bg(Color::Rgb(49, 50, 68)).fg(fg))
        .scroll((0, scroll as u16));

    frame.render_widget(input, chunks[1]);

    if app.ui.input.active == InputActive::NewListUrl {
        let cursor_x = (app.ui.input.new_list_url.visual_cursor().max(scroll) - scroll) as u16;
        frame.set_cursor_position((chunks[1].x + cursor_x, chunks[1].y));
    }
}
