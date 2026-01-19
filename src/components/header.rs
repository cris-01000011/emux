use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
};

use crate::{actions::system::Command, app::App};

pub fn render_header(frame: &mut ratatui::Frame, app: &App, area: Rect) {
    let commands = app.get_current_commands();

    let selected_style = Style::default()
        .bg(Color::Rgb(203, 166, 247))
        .fg(Color::Black);

    let line = generate_tabs(app, commands);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Min(0)])
        .split(area);

    let search_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(137, 180, 250)))
        .title("Search");

    let search_text = app.search_query.clone();
    let search_paragraph = Paragraph::new(search_text)
        .style(Color::Rgb(180, 190, 254))
        .block(search_block);

    frame.render_widget(search_paragraph, chunks[0]);

    let tabs_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Color::Rgb(203, 166, 247));

    let tabs = Tabs::new(line)
        .block(tabs_block)
        .select(app.selected_command)
        .highlight_style(selected_style)
        .divider("");

    frame.render_widget(tabs, chunks[1]);
}

fn generate_tabs(app: &App, commands: Vec<Command>) -> Line<'static> {
    if !app.in_list {
        return Line::default();
    }

    let mut spans: Vec<Span> = Vec::new();

    for (i, cmd) in commands.iter().enumerate() {
        let style = Style::default()
            .bg(Color::Rgb(180, 190, 254))
            .fg(Color::Black);

        let text = if app.in_list && i == app.selected_command {
            format!(" {} ", cmd.name)
        } else {
            format!("  {}  ", i + 1)
        };

        spans.push(Span::styled(text, style));
    }

    Line::from(spans)
}
