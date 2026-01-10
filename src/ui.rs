use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &mut App) {
    // Screen → list + commands footer
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // list content
            Constraint::Length(1), // commands footer
        ])
        .split(frame.area());

    // Calculate visible height for list (account for borders)
    let list_area = vertical[0];
    let visible_height = list_area.height as usize;
    app.update_scroll_for_height(visible_height);

    // -------- MAIN LIST --------
    let items: Vec<ListItem> = if app.in_search_mode {
        // Search results
        if app.in_system {
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
        } else {
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

                    ListItem::new(Line::from(Span::styled(name.to_string(), style)))
                })
                .collect()
        }
    } else if app.in_system {
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
    } else {
        // Systems (*.json) with scroll
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
    };

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_style(Color::Rgb(203, 166, 247)),
    );

    frame.render_widget(list, vertical[0]);

    // -------- COMMANDS FOOTER --------
    let commands = app.get_current_commands();

    if !commands.is_empty() {
        // Calculate exact width needed for each command name (including padding spaces)
        let mut constraints = Vec::new();
        for (i, cmd) in commands.iter().enumerate() {
            let name_length = cmd.name.len() + 2; // Add 2 for padding spaces
            constraints.push(Constraint::Length(name_length as u16)); // Exact width needed
            if i < commands.len() - 1 {
                constraints.push(Constraint::Length(1)); // Spacing between labels
            }
        }

        let command_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(vertical[1]);

        // Render each command name as a simple colored label with padding
        for (i, cmd) in commands.iter().enumerate() {
            let label_index = if i == 0 { 0 } else { i * 2 }; // Account for spacing
            if label_index < command_chunks.len() {
                let padded_name = format!(" {} ", cmd.name); // Add padding spaces

                // Highlight selected command
                let style = if app.in_command_selection && i == app.selected_command {
                    Style::default()
                        .bg(Color::Rgb(203, 166, 247))
                        .fg(Color::Black)
                } else {
                    Style::default()
                        .bg(Color::Rgb(203, 195, 245))
                        .fg(Color::Black)
                };

                let command_label = Paragraph::new(padded_name).style(style);

                frame.render_widget(command_label, command_chunks[label_index]);
            }
        }
    }

    // -------- SEARCH POPUP --------
    if app.in_search_mode {
        // Create popup area (centered at top)
        let popup_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // popup height
                Constraint::Min(0),    // rest
            ])
            .split(frame.area())[0];

        let popup_width = 30; // Fixed width for better centering
        let popup_x = (frame.area().width.saturating_sub(popup_width)) / 2;
        let popup_y = popup_area.y + 1;

        let popup_rect = ratatui::layout::Rect {
            x: popup_x,
            y: popup_y,
            width: popup_width,
            height: 3,
        };

        // Clear the area behind popup
        frame.render_widget(Clear, popup_rect);

        // Render search input
        let search_text = format!("Search: {}", app.search_query);
        let search_paragraph = Paragraph::new(search_text)
            .style(Style::default().fg(Color::Rgb(203, 166, 247)))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Color::Rgb(203, 166, 247)),
            );

        frame.render_widget(search_paragraph, popup_rect);
    }
}
