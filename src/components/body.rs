use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListItem},
};

use crate::app::App;

pub fn render_body(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(65),
            Constraint::Percentage(20),
        ])
        .split(area);

    render_left_panel(frame, app, horizontal[0]);
    render_center_panel(frame, app, horizontal[1]);
    render_right_panel(frame, app, horizontal[2]);
}

fn render_left_panel(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let styles = LeftPanelStyles::new();
    let panel_block = create_panel_block();

    match (app.in_list, app.in_search_mode) {
        (false, false) => render_directory_list(frame, app, area, &styles, &panel_block),
        (false, true) => render_directory_search(frame, app, area, &styles, &panel_block),
        _ => render_directory_list(frame, app, area, &styles, &panel_block),
    }
}

fn render_center_panel(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let styles = CenterPanelStyles::new();
    let panel_block = create_panel_block();

    match (app.in_list, app.in_search_mode) {
        (true, true) => render_items_search(frame, app, area, &styles, &panel_block),
        (true, false) => render_items_list(frame, app, area, &styles, &panel_block),
        (false, _) => render_empty_panel(frame, area, &panel_block),
    }
}

fn render_right_panel(frame: &mut ratatui::Frame, _app: &App, area: Rect) {
    let panel_block = create_panel_block();
    frame.render_widget(panel_block, area);
}

// Styles
struct LeftPanelStyles {
    normal: Style,
    selected: Style,
}

impl LeftPanelStyles {
    fn new() -> Self {
        Self {
            normal: Style::default().fg(Color::Rgb(137, 180, 250)),
            selected: Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(137, 180, 250)),
        }
    }
}

struct CenterPanelStyles {
    normal: Style,
    selected: Style,
    favorite: Style,
    favorite_selected: Style,
    icon: Style,
}

impl CenterPanelStyles {
    fn new() -> Self {
        Self {
            normal: Style::default().fg(Color::Rgb(148, 226, 213)),
            selected: Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(148, 226, 213)),
            favorite: Style::default().fg(Color::Rgb(245, 194, 231)),
            favorite_selected: Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(245, 194, 231)),
            icon: Style::default().fg(Color::Rgb(249, 226, 175)),
        }
    }
}

// Panel utilities
fn create_panel_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Color::Rgb(137, 180, 250))
}

// Directory list rendering
fn render_directory_list(
    frame: &mut ratatui::Frame,
    app: &mut App,
    area: Rect,
    styles: &LeftPanelStyles,
    block: &Block<'static>,
) {
    let visible_height = area.height.saturating_sub(2) as usize;
    app.ensure_selection_visible_directory(visible_height);

    let items: Vec<ListItem> = app
        .entries
        .iter()
        .enumerate()
        .map(|(_index, path)| {
            let name = extract_display_name(path);
            ListItem::new(Line::from(vec![
                Span::styled(" ", styles.normal),
                Span::styled(name, styles.normal),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(block.clone())
        .highlight_symbol("")
        .highlight_style(styles.selected)
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(list, area, &mut app.directory_list_state);
}

fn render_directory_search(
    frame: &mut ratatui::Frame,
    app: &App,
    area: Rect,
    styles: &LeftPanelStyles,
    block: &Block<'static>,
) {
    let items: Vec<ListItem> = app
        .search_results
        .iter()
        .enumerate()
        .map(|(search_idx, &entry_idx)| {
            let path = &app.entries[entry_idx];
            let name = extract_display_name(path);
            let style = if search_idx == app.search_selected {
                styles.selected
            } else {
                styles.normal
            };

            ListItem::new(Line::from(vec![
                Span::styled(" ", style),
                Span::styled(name, style),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(block.clone())
        .highlight_symbol("")
        .highlight_style(styles.selected)
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_widget(list, area);
}

// Items list rendering
fn render_items_list(
    frame: &mut ratatui::Frame,
    app: &mut App,
    area: Rect,
    styles: &CenterPanelStyles,
    block: &Block<'static>,
) {
    let visible_height = area.height.saturating_sub(2) as usize;
    app.ensure_selection_visible_items(visible_height);

    let items: Vec<ListItem> = app
        .roms
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let is_favorite = app.is_favorite(&app.current_list, &item.item);

            let (icon, icon_style) = if is_favorite {
                ("󰋑 ", styles.favorite)
            } else {
                (" ", styles.icon)
            };

            let (text_style, selected_style) = if is_favorite {
                (styles.favorite, styles.favorite_selected)
            } else {
                (styles.normal, styles.selected)
            };

            let item_style = if Some(index) == app.items_list_state.selected() {
                selected_style
            } else {
                text_style
            };

            ListItem::new(Line::from(vec![
                Span::styled(icon, icon_style),
                Span::styled(item.item.clone(), item_style),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(block.clone())
        .highlight_symbol("")
        .highlight_style(styles.selected)
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(list, area, &mut app.items_list_state);
}

fn render_items_search(
    frame: &mut ratatui::Frame,
    app: &App,
    area: Rect,
    styles: &CenterPanelStyles,
    block: &Block<'static>,
) {
    let items: Vec<ListItem> = app
        .search_results
        .iter()
        .enumerate()
        .map(|(search_idx, &rom_idx)| {
            let rom = &app.roms[rom_idx];
            let is_favorite = app.is_favorite(&app.current_list, &rom.item);

            let (icon, icon_style) = if is_favorite {
                ("󰋑 ", styles.favorite)
            } else {
                (" ", styles.icon)
            };

            let (text_style, selected_style) = if is_favorite {
                (styles.favorite, styles.favorite_selected)
            } else {
                (styles.normal, styles.selected)
            };

            let item_style = if search_idx == app.search_selected {
                selected_style
            } else {
                text_style
            };

            ListItem::new(Line::from(vec![
                Span::styled(icon, icon_style),
                Span::styled(rom.item.clone(), item_style),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(block.clone())
        .highlight_symbol("")
        .highlight_style(styles.selected)
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_widget(list, area);
}

fn render_empty_panel(frame: &mut ratatui::Frame, area: Rect, block: &Block<'static>) {
    let list = List::new(Vec::<ListItem>::new())
        .block(block.clone())
        .highlight_symbol("")
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_widget(list, area);
}

// Utility functions
fn extract_display_name(path: &std::path::Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("<unknown>")
        .to_string()
}
