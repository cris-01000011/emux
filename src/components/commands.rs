use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::app::App;

pub fn render_commands(frame: &mut ratatui::Frame, app: &App, area: Rect) {
    let line = generate_tabs(app);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0)])
        .split(area);

    let tabs = Paragraph::new(line);

    let block = Block::default().style(Style::new().bg(Color::Rgb(49, 50, 68)));
    frame.render_widget(block, chunks[0]);

    frame.render_widget(tabs, chunks[0]);
}

fn generate_tabs(app: &App) -> Line<'static> {
    let mut spans: Vec<Span> = Vec::new();

    if let Some(current_list) = app.commands.lists.get(app.commands.selected_list) {
        for (i, command) in current_list.commands.iter().enumerate() {
            let is_selected = i == app.commands.selected_command;

            let style = if is_selected {
                Style::default()
                    .bg(Color::Rgb(24, 24, 37))
                    .fg(Color::Rgb(180, 190, 254))
            } else {
                Style::default().fg(Color::Rgb(180, 190, 254))
            };

            let text = if is_selected {
                format!(" {} ", command.name.clone())
            } else {
                format!("  {}  ", i + 1)
            };

            spans.push(Span::styled(text, style));
        }
    }

    Line::from(spans)
}
