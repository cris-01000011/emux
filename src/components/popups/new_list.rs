use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Clear, Paragraph},
};

use crate::{
    app::App,
    components::{
        input::InputActive,
        inputs::{
            new_list_name::render_new_list_name_input, new_list_url::render_new_list_url_input,
        },
        popup::{ActivePopup, Popup},
    },
    ui::UiState,
};

impl UiState {
    pub fn close_new_list_popup(&mut self) {
        self.popup.active = ActivePopup::None;
        self.input.active = InputActive::None;
        self.input.new_list_name.reset();
        self.input.new_list_url.reset();
    }
}

pub fn render_popup_new_list(app: &App, frame: &mut Frame, area: Rect) {
    let area = Popup::popup_area(area, 40, 8);
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
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .horizontal_margin(2)
    .split(area);

    render_title(frame, chunks[0]);
    render_new_list_name_input(app, frame, chunks[2]);
    render_new_list_url_input(app, frame, chunks[4]);
    render_button(frame, chunks[6]);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let span = Span::raw(" New List ")
        .style(
            Style::new()
                .bg(Color::Rgb(203, 166, 247))
                .fg(Color::Rgb(30, 30, 46)),
        )
        .into_centered_line();

    frame.render_widget(span, area);
}

fn render_button(frame: &mut Frame, area: Rect) {
    let style = Style::new()
        .bg(Color::Rgb(148, 226, 213))
        .fg(Color::Rgb(30, 30, 46));

    let span = Paragraph::new("  Create  ")
        .alignment(Alignment::Center)
        .style(style);

    frame.render_widget(span, area);
}
