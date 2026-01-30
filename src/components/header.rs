use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Tabs},
};

use crate::{
    actions::{commands::Command, navigation::View},
    app::App,
    components::inputs::search::InputMode,
};

pub fn render_header(frame: &mut ratatui::Frame, app: &App, area: Rect) {
    let commands = app.commands.get_current_commands();

    let selected_style = Style::default()
        .bg(Color::Rgb(203, 166, 247))
        .fg(Color::Black);

    let line = generate_tabs(app, commands.to_vec());

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Min(0)])
        .split(area);

    render_input(app, frame, chunks[0]);

    let tabs_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Color::Rgb(203, 166, 247));

    let tabs = Tabs::new(line)
        .block(tabs_block)
        .select(app.commands.selected_command)
        .highlight_style(selected_style)
        .divider("");

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
        .block(
            Block::bordered()
                .title("Search")
                .border_type(BorderType::Rounded),
        );
    frame.render_widget(input, area);

    if app.ui.search.mode == InputMode::Editing {
        let x = app.ui.search.input.visual_cursor().max(scroll) - scroll + 1;
        frame.set_cursor_position((area.x + x as u16, area.y + 1))
    }
}

fn generate_tabs(app: &App, commands: Vec<Command>) -> Line<'static> {
    if app.navigation.view == View::Lists {
        return Line::default();
    }

    let mut spans: Vec<Span> = Vec::new();

    for (i, cmd) in commands.iter().enumerate() {
        let style = Style::default()
            .bg(Color::Rgb(180, 190, 254))
            .fg(Color::Black);

        let text = if app.navigation.view == View::Items && i == app.commands.selected_command {
            format!(" {} ", cmd.name)
        } else {
            format!("  {}  ", i + 1)
        };

        spans.push(Span::styled(text, style));
    }

    Line::from(spans)
}
