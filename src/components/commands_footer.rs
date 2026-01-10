use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
};
use unicode_width::UnicodeWidthStr;

use crate::app::App;

pub fn render_commands_footer(frame: &mut ratatui::Frame, app: &App, area: Rect) {
    let commands = app.get_current_commands();
    if commands.is_empty() {
        return;
    }

    let mut constraints = Vec::with_capacity(commands.len() * 2 - 1);

    for (i, cmd) in commands.iter().enumerate() {
        let width = UnicodeWidthStr::width(cmd.name.as_str()) + 2;
        constraints.push(Constraint::Length(width as u16));

        if i + 1 < commands.len() {
            constraints.push(Constraint::Length(1));
        }
    }

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    let normal_style = Style::default()
        .bg(Color::Rgb(203, 195, 245))
        .fg(Color::Black);

    let selected_style = Style::default()
        .bg(Color::Rgb(203, 166, 247))
        .fg(Color::Black);

    let mut chunk_index = 0;

    for (i, cmd) in commands.iter().enumerate() {
        let style = if app.in_command_selection && i == app.selected_command {
            selected_style
        } else {
            normal_style
        };

        let label = Paragraph::new(format!(" {} ", cmd.name)).style(style);
        frame.render_widget(label, chunks[chunk_index]);

        chunk_index += 2;
    }
}
