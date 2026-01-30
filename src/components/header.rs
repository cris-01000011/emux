use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::{actions::commands::Command, app::App, components::inputs::search::InputMode};

pub fn render_header(frame: &mut ratatui::Frame, app: &App, area: Rect) {
    let commands = app.commands.get_current_commands();

    let line = generate_tabs(app, commands.to_vec());

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(0), Constraint::Min(0)])
        .split(area);

    render_input(app, frame, chunks[0]);

    let tabs = Paragraph::new(line);

    let block = Block::default().style(Style::new().bg(Color::Rgb(49, 50, 68)));
    frame.render_widget(block, chunks[1]);

    frame.render_widget(tabs, chunks[1]);
}

fn render_input(app: &App, frame: &mut Frame, area: Rect) {
    let width = area.width.max(3) - 3;
    let scroll = app.ui.search.input.visual_scroll(width as usize);
    let style = match app.ui.search.mode {
        InputMode::Normal => Color::Rgb(137, 180, 250),
        InputMode::Editing => Color::Rgb(180, 190, 254),
    };
    let input = Paragraph::new(app.ui.search.input.value())
        .style(style)
        .scroll((0, scroll as u16))
        .block(Block::default().title("Search"));
    frame.render_widget(input, area);

    if app.ui.search.mode == InputMode::Editing {
        let x = app.ui.search.input.visual_cursor().max(scroll) - scroll + 1;
        frame.set_cursor_position((area.x + x as u16, area.y + 1))
    }
}

fn generate_tabs(app: &App, commands: Vec<Command>) -> Line<'static> {
    let mut spans: Vec<Span> = Vec::new();

    for (i, cmd) in commands.iter().enumerate() {
        let is_selected = i == app.commands.selected_command;

        let style = if is_selected {
            Style::default()
                .bg(Color::Rgb(24, 24, 37))
                .fg(Color::Rgb(180, 190, 254))
        } else {
            Style::default().fg(Color::Rgb(180, 190, 254))
        };

        let text = if is_selected {
            format!(" {} ", cmd.name)
        } else {
            format!("  {}  ", i + 1)
        };

        spans.push(Span::styled(text, style));
    }

    Line::from(spans)
}
