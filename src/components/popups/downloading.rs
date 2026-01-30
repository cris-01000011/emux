use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Clear, Gauge},
};

use crate::{app::App, components::popup::Popup};

pub fn render_popup(frame: &mut ratatui::Frame, app: &App, area: Rect) {
    let area = Popup::popup_area(area, 40, 6);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Block::default().style(Style::new().bg(Color::Rgb(30, 30, 46))),
        area,
    );

    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .horizontal_margin(2)
    .split(area);

    render_title(frame, chunks[0]);
    render_gauge(app, frame, chunks[2]);
    render_progress_text(app, frame, chunks[4]);
    render_button(app, frame, chunks[4]);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let span = Span::raw(" Downloading ")
        .style(
            Style::new()
                .bg(Color::Rgb(203, 166, 247))
                .fg(Color::Rgb(30, 30, 46)),
        )
        .into_centered_line();

    frame.render_widget(span, area);
}

fn render_gauge(app: &App, frame: &mut Frame, area: Rect) {
    let percent = app.download.progress;
    let label = Span::styled(
        format!("{:.1}/100", percent),
        Style::new().italic().bold().fg(Color::Rgb(30, 30, 46)),
    );

    let gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .bg(Color::Rgb(180, 190, 254))
                .fg(Color::Rgb(137, 180, 250)),
        )
        .label(label)
        .percent(percent as u16);

    frame.render_widget(gauge, area);
}

fn render_progress_text(app: &App, frame: &mut Frame, area: Rect) {
    let downloaded_kb = app.download.downloaded / 1024;
    let total_kb = app.download.total / 1024;
    let progress_text = if total_kb > 0 {
        format!(
            "{:.1} MB / {:.1} MB",
            downloaded_kb as f64 / 1024.0,
            total_kb as f64 / 1024.0
        )
    } else {
        format!("{:.1} MB", downloaded_kb as f64 / 1024.0)
    };

    let span = Span::raw(progress_text)
        .style(Style::new().fg(Color::Rgb(137, 180, 250)))
        .into_left_aligned_line();

    frame.render_widget(span, area);
}

fn render_button(app: &App, frame: &mut Frame, area: Rect) {
    let bg = match app.download.progress < 100.0 {
        true => Color::Rgb(180, 190, 254),
        false => Color::Rgb(148, 226, 213),
    };

    let style = Style::new().bg(bg).fg(Color::Rgb(30, 30, 46));

    let span = Span::raw("  Launch  ")
        .style(style)
        .into_right_aligned_line();

    frame.render_widget(span, area);
}
