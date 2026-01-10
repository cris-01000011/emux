use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::App;

pub fn render_main_list(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = if app.in_search_mode {
        if app.in_system {
            render_search_rom_items(app)
        } else {
            render_search_system_items(app)
        }
    } else if app.in_system {
        render_rom_items(app)
    } else {
        render_system_items(app)
    };

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_style(Color::Rgb(203, 166, 247)),
    );

    frame.render_widget(list, area);
}

fn render_search_rom_items(app: &App) -> Vec<ListItem<'_>> {
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

fn render_search_system_items(app: &App) -> Vec<ListItem<'_>> {
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

fn render_rom_items(app: &App) -> Vec<ListItem<'_>> {
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
                    .bg(Color::Rgb(180, 190, 254))
            } else if is_fav {
                Style::default().fg(Color::Rgb(203, 166, 247))
            } else {
                Style::default().fg(Color::Rgb(180, 190, 224))
            };

            let icon = if is_fav { "󰋑 " } else { "󰊖 " };

            ListItem::new(Line::from(vec![
                Span::styled(icon, style),
                Span::styled(r.title.clone(), style),
            ]))
        })
        .collect()
}

fn render_system_items(app: &App) -> Vec<ListItem<'_>> {
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
                    .bg(Color::Rgb(180, 190, 254))
            } else {
                Style::default().fg(Color::Rgb(180, 190, 254))
            };

            ListItem::new(Line::from(vec![
                Span::styled(" ", style),
                Span::styled(name, style),
            ]))
        })
        .collect()
}
