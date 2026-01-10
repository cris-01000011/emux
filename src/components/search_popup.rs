use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::App;

pub fn render_search_popup(frame: &mut ratatui::Frame, app: &App, area: Rect) {
    if !app.in_search_mode {
        return;
    }

    let popup_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area)[0];

    let popup_width = 30;
    let popup_x = (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = popup_area.y + 1;

    let popup_rect = Rect {
        x: popup_x,
        y: popup_y,
        width: popup_width,
        height: 3,
    };

    frame.render_widget(Clear, popup_rect);

    let search_text = format!("{}", app.search_query);
    let search_paragraph = Paragraph::new(search_text)
        .style(Style::default().fg(Color::Rgb(180, 190, 254)))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Color::Rgb(203, 166, 247))
                .title("Search"),
        );

    frame.render_widget(search_paragraph, popup_rect);
}
