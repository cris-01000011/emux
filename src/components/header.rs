use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{actions::system::Command, app::App};

pub fn render_header(frame: &mut ratatui::Frame, app: &App, area: Rect) {
    let commands = app.get_current_commands();

    let normal_style = Style::default()
        .bg(Color::Rgb(180, 190, 254))
        .fg(Color::Black);

    let selected_style = Style::default().bg(Color::LightBlue).fg(Color::Black);

    let line = generate_tabs(app, commands, normal_style, selected_style);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Min(0)])
        .split(area);

    let search_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(137, 180, 250)))
        .title("Search");

    let search_text = format!("{}", app.search_query);
    let search_paragraph = Paragraph::new(search_text).block(search_block);

    frame.render_widget(search_paragraph, chunks[0]);

    let tabs_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::LightBlue));

    let paragraph = Paragraph::new(line).block(tabs_block);

    frame.render_widget(paragraph, chunks[1]);
}

// Generate the footer tabs as a single Line
fn generate_tabs(
    app: &App,
    commands: Vec<Command>,
    normal_style: Style,
    selected_style: Style,
) -> Line<'static> {
    let mut spans: Vec<Span> = Vec::new();

    if !app.in_system {
        spans.push(Span::styled(" Systems ", normal_style));
        spans.push(Span::raw(" "));
        spans.push(Span::styled(" Favorites ", normal_style));
        return Line::from(spans);
    }

    for (i, cmd) in commands.iter().enumerate() {
        let style = if app.in_command_selection && i == app.selected_command {
            selected_style
        } else {
            normal_style
        };

        let text = if app.in_command_selection && i == app.selected_command {
            format!(" {} ", cmd.name)
        } else {
            format!("  {}  ", i + 1)
        };

        spans.push(Span::styled(text, style));

        if i + 1 < commands.len() {
            spans.push(Span::raw(" "));
        }
    }

    Line::from(spans)
}
