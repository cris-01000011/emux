use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, HighlightSpacing, List, ListItem},
};

use crate::{
    actions::navigation::View,
    app::App,
    components::{commands::render_commands, input::InputActive, status_bar::render_status_bar},
};

pub fn render_body(frame: &mut Frame, app: &mut App, area: Rect) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(22), Constraint::Min(0)])
        .split(vertical[0]);

    render_left_panel(frame, app, horizontal[0]);
    render_center_panel(frame, app, horizontal[1]);
    render_status_bar(app, frame, vertical[1]);
}

fn render_left_panel(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(area);

    let block = Block::default().style(Style::new().bg(Color::Rgb(17, 17, 27)));
    frame.render_widget(block, chunks[0]);

    let styles = LeftPanelStyles::new();
    let panel_block = Block::default().style(Style::new().bg(Color::Rgb(17, 17, 27)));

    render_directory_list(frame, app, chunks[1], &styles, &panel_block);
}

fn render_center_panel(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(area);

    let styles = CenterPanelStyles::new();
    let panel_block = Block::default().style(Style::new().bg(Color::Rgb(24, 24, 37)));

    render_commands(frame, app, chunks[0]);
    render_items_list(frame, app, chunks[1], &styles, &panel_block);
}

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
    _favorite_selected: Style,
    icon: Style,
}

impl CenterPanelStyles {
    fn new() -> Self {
        Self {
            normal: Style::default().fg(Color::Rgb(180, 190, 254)),
            selected: Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(180, 190, 254)),
            favorite: Style::default().fg(Color::Rgb(245, 194, 231)),
            _favorite_selected: Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(245, 194, 231)),
            icon: Style::default().fg(Color::Rgb(180, 190, 254)),
        }
    }
}

fn render_directory_list(
    frame: &mut ratatui::Frame,
    app: &mut App,
    area: Rect,
    styles: &LeftPanelStyles,
    block: &Block<'static>,
) {
    let items: Vec<ListItem> = if app.ui.input.active == InputActive::Search
        && !app.ui.input.search.value().is_empty()
        && app.navigation.view == View::Lists
    {
        // Show search results only when searching in Lists
        app.search
            .lists_query
            .iter()
            .map(|&index| {
                let path = &app.data.lists[index];
                let name = extract_display_name(path);
                ListItem::new(Line::from(vec![
                    Span::styled("  ", styles.normal),
                    Span::styled(name, styles.normal),
                ]))
            })
            .collect()
    } else {
        // Show all lists (normal view or when searching in Items)
        app.data
            .lists
            .iter()
            .enumerate()
            .map(|(_index, path)| {
                let name = extract_display_name(path);
                ListItem::new(Line::from(vec![
                    Span::styled("  ", styles.normal),
                    Span::styled(name, styles.normal),
                ]))
            })
            .collect()
    };

    let list = List::new(items)
        .block(block.clone())
        .highlight_symbol("")
        .highlight_style(styles.selected)
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(list, area, &mut app.ui.lists);
}

fn render_items_list(
    frame: &mut ratatui::Frame,
    app: &mut App,
    area: Rect,
    styles: &CenterPanelStyles,
    block: &Block<'static>,
) {
    let items = app.get_current_list_items();
    let items_display: Vec<ListItem> = if app.ui.input.active == InputActive::Search
        && !app.ui.input.search.value().is_empty()
        && app.navigation.view == View::Items
    {
        // Show search results only when searching in Items
        app.search
            .items_query
            .iter()
            .map(|&index| {
                let item = &items[index];
                let list = app.current_list_name();
                let is_favorite = app.favorite.is_favorite(list, &item.item);
                let in_list = app.navigation.view == View::Items;

                let (icon, icon_style) = if is_favorite {
                    (" 󰋑 ", styles.favorite)
                } else {
                    ("  ", styles.icon)
                };

                let text_style = match (in_list, is_favorite) {
                    (true, false) => styles.normal,
                    (_, true) => styles.favorite,
                    _ => styles.normal,
                };

                ListItem::new(Line::from(vec![
                    Span::styled(icon, icon_style),
                    Span::styled(item.item.clone(), text_style),
                ]))
            })
            .collect()
    } else {
        // Show all items (normal view or when searching in Lists)
        items
            .iter()
            .enumerate()
            .map(|(_, item)| {
                let list = app.current_list_name();
                let is_favorite = app.favorite.is_favorite(list, &item.item);
                let in_list = app.navigation.view == View::Items;

                let (icon, icon_style) = if is_favorite {
                    (" 󰋑 ", styles.favorite)
                } else {
                    ("  ", styles.icon)
                };

                let text_style = match (in_list, is_favorite) {
                    (true, false) => styles.normal,
                    (_, true) => styles.favorite,
                    _ => styles.normal,
                };

                ListItem::new(Line::from(vec![
                    Span::styled(icon, icon_style),
                    Span::styled(item.item.clone(), text_style),
                ]))
            })
            .collect()
    };

    let mut list = List::new(items_display).block(block.clone());

    if app.navigation.view == View::Items {
        list = list.highlight_style(styles.selected);
    }

    frame.render_stateful_widget(list, area, &mut app.ui.items_in_list);
}

fn extract_display_name(path: &std::path::Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("<unknown>")
        .to_string()
}
