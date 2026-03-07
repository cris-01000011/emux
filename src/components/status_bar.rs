use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    Frame,
};

use crate::{
    actions::navigation::{ListsView, View},
    app::App,
    components::inputs::search::render_search_input,
};

pub fn render_status_bar(app: &App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Min(0)])
        .split(area);

    render_search_input(app, frame, chunks[0]);

    let downloaded_size_bytes = if app.navigation.list_view == ListsView::LocalLists {
        if app.navigation.view == View::Items {
            app.get_current_local_item_size()
        } else {
            app.get_current_local_folder_size()
        }
    } else {
        match app.navigation.view {
            View::Lists => app.get_current_list_downloaded_size(),
            View::Items => app.get_current_item_downloaded_size(),
        }
    };

    let downloaded_size_gb = downloaded_size_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let downloaded_size_mb = downloaded_size_bytes as f64 / (1024.0 * 1024.0);
    let downloaded_size_kb = downloaded_size_bytes as f64 / 1024.0;

    let size_text = match downloaded_size_bytes {
        bytes if bytes >= (1024 * 1024 * 1024) => format!(" {:.1} GBs ", downloaded_size_gb),
        bytes if bytes >= (1024 * 1024) => format!(" {:.1} MBs ", downloaded_size_mb),
        bytes if bytes >= 1024 => format!(" {:.0} KBs ", downloaded_size_kb),
        _ => format!(" {} Bs ", downloaded_size_bytes),
    };

    let label_size = Span::styled(
        size_text,
        Style::new()
            .bg(Color::Rgb(49, 50, 68))
            .fg(Color::Rgb(180, 190, 254)),
    );

    let icon = if app.navigation.list_view == ListsView::LocalLists {
        "󰉋 "
    } else {
        match app.navigation.view {
            View::Lists => " ",
            View::Items => " ",
        }
    };

    let icon_size = Span::styled(
        icon,
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
