use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::app::App;

pub fn render_main_list(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(area);

    let left_panel = lists_area(app);
    frame.render_widget(left_panel, horizontal[0]);

    let right_panel = items_in_list_area(app);
    frame.render_widget(right_panel, horizontal[1]);
}

fn lists_area(app: &App) -> List<'_> {
    let items = match (app.in_search_mode, app.in_system) {
        (true, false) => searched_lists(app),
        (false, false) => lists(app),
        _ => lists(app), // no items to show in other states
    };

    List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Color::Rgb(137, 180, 250)),
    )
}

fn items_in_list_area(app: &App) -> List<'_> {
    let items = match (app.in_search_mode, app.in_system) {
        (true, true) => searched_items_in_list(app),
        _ => items_in_list(app),
    };

    List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Color::Rgb(137, 180, 250)),
    )
}

fn searched_items_in_list(app: &App) -> Vec<ListItem<'_>> {
    app.search_results
        .iter()
        .enumerate()
        .map(|(search_idx, &rom_idx)| {
            let rom = &app.roms[rom_idx];
            let is_fav = app.is_favorite(&app.current_system, &rom.title);

            let style = if search_idx == app.search_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Rgb(180, 190, 254))
            } else if is_fav {
                Style::default().fg(Color::Rgb(203, 166, 247))
            } else {
                Style::default().fg(Color::Rgb(180, 190, 224))
            };

            let icon = if is_fav { "󰋑 " } else { "󰊖 " };

            ListItem::new(Line::from(vec![
                Span::styled(icon, style),
                Span::styled(rom.title.clone(), style),
            ]))
        })
        .collect()
}

fn searched_lists(app: &App) -> Vec<ListItem<'_>> {
    app.search_results
        .iter()
        .enumerate()
        .map(|(search_idx, &entry_idx)| {
            let path = &app.entries[entry_idx];
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("<unknown>");
            let style = if search_idx == app.search_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Rgb(180, 190, 254))
            } else {
                Style::default().fg(Color::Rgb(180, 190, 254))
            };

            ListItem::new(Line::from(vec![
                Span::styled(" ", style),
                Span::styled(name.to_string(), style),
            ]))
        })
        .collect()
}

fn items_in_list(app: &App) -> Vec<ListItem<'_>> {
    app.roms
        .iter()
        .skip(app.roms_scroll_offset)
        .enumerate()
        .map(|(i, r)| {
            let actual_index = i + app.roms_scroll_offset;
            let is_fav = app.is_favorite(&app.current_system, &r.title);

            let style = if actual_index == app.selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Rgb(148, 226, 213))
            } else if is_fav {
                Style::default().fg(Color::Rgb(245, 194, 231))
            } else {
                Style::default().fg(Color::Rgb(148, 226, 213))
            };

            let icon = if is_fav { "󰋑 " } else { "󰊖 " };

            let style_icon = if actual_index == app.selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Rgb(148, 226, 213))
            } else if is_fav {
                Style::default().fg(Color::Rgb(245, 194, 231))
            } else {
                Style::default().fg(Color::Rgb(249, 226, 175))
            };

            ListItem::new(Line::from(vec![
                Span::styled(icon, style_icon),
                Span::styled(r.title.clone(), style),
            ]))
        })
        .collect()
}

fn lists(app: &App) -> Vec<ListItem<'_>> {
    app.entries
        .iter()
        .skip(app.scroll_offset)
        .enumerate()
        .map(|(i, p)| {
            let actual_index = i + app.scroll_offset;
            let name = p
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("<unknown>");

            let style = if actual_index == app.selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Rgb(137, 180, 250))
            } else {
                Style::default().fg(Color::Rgb(137, 180, 250))
            };

            ListItem::new(Line::from(vec![
                Span::styled(" ", style),
                Span::styled(name, style),
            ]))
        })
        .collect()
}
