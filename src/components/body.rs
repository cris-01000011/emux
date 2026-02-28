use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Padding},
};

use crate::{
    actions::navigation::View,
    app::App,
    components::{commands::render_commands, input::InputActive, status_bar::render_status_bar},
    utils::string::extract_list_name,
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

    render_first_panel(frame, app, horizontal[0]);
    render_second_panel(frame, app, horizontal[1]);
    render_status_bar(app, frame, vertical[1]);
}

fn render_first_panel(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let styles = ListsStyles::new();
    let block = Block::default()
        .style(Style::new().bg(Color::Rgb(17, 17, 27)))
        .padding(Padding::top(1));

    render_lists(frame, app, area, &styles, block);
}

fn render_second_panel(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(area);

    let styles = ItemsStyles::new();
    let block = Block::default().style(Style::new().bg(Color::Rgb(24, 24, 37)));

    render_commands(frame, app, chunks[0]);
    render_items(frame, app, chunks[1], &styles, block);
}

struct ListsStyles {
    normal: Style,
    selected: Style,
}

impl ListsStyles {
    fn new() -> Self {
        Self {
            normal: Style::default().fg(Color::Rgb(137, 180, 250)),
            selected: Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(137, 180, 250)),
        }
    }
}

struct ItemsStyles {
    normal: Style,
    selected: Style,
    favorite: Style,
    favorite_selected: Style,
}

impl ItemsStyles {
    fn new() -> Self {
        Self {
            normal: Style::default().fg(Color::Rgb(180, 190, 254)),
            selected: Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(180, 190, 254)),
            favorite: Style::default().fg(Color::Rgb(245, 194, 231)),
            favorite_selected: Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(245, 194, 231)),
        }
    }
}

fn render_lists(
    frame: &mut ratatui::Frame,
    app: &mut App,
    area: Rect,
    styles: &ListsStyles,
    block: Block,
) {
    let items: Vec<ListItem> = if app.ui.input.active == InputActive::Search
        && !app.ui.input.search.value().is_empty()
        && app.navigation.view == View::Lists
    {
        app.search
            .lists_query
            .iter()
            .map(|&index| {
                let path = &app.data.lists[index];
                let list_name = extract_list_name(path);

                ListItem::new(Line::from(vec![Span::raw("  "), Span::raw(list_name)]))
                    .style(styles.normal)
            })
            .collect()
    } else {
        app.data
            .lists
            .iter()
            .map(|path| {
                let list_name = extract_list_name(path);

                ListItem::new(Line::from(vec![Span::raw("  "), Span::raw(list_name)]))
                    .style(styles.normal)
            })
            .collect()
    };

    let list = List::new(items)
        .block(block)
        .highlight_style(styles.selected);

    frame.render_stateful_widget(list, area, &mut app.ui.lists);
}

fn render_items(
    frame: &mut ratatui::Frame,
    app: &mut App,
    area: Rect,
    styles: &ItemsStyles,
    block: Block,
) {
    let items = app.get_current_list_items();

    let total = if app.ui.input.active == InputActive::Search
        && !app.ui.input.search.value().is_empty()
        && app.navigation.view == View::Items
    {
        app.search.items_query.len()
    } else {
        items.len()
    };

    let start = app.scroll.start;
    let end = app.scroll.end.min(total);
    app.scroll.visible = area.height as usize;

    if app.scroll.end == 0 {
        app.scroll.end = app.scroll.visible;
    }

    if app.scroll.selected > app.scroll.visible {
        app.scroll.selected = app.scroll.visible - 1;
    }

    let items_display: Vec<ListItem> = if app.ui.input.active == InputActive::Search
        && !app.ui.input.search.value().is_empty()
        && app.navigation.view == View::Items
    {
        app.search.items_query[start..end]
            .iter()
            .enumerate()
            .map(|(i, &index)| {
                let item = &items[index];
                let list = app.current_list_name();
                let is_downloaded = app.is_downloaded(&item.item);
                let is_favorite = app.favorite.is_favorite(list, &item.item);
                let selected = i == app.scroll.selected;

                let (icon, style) = match (selected, is_favorite, is_downloaded) {
                    (true, true, false) => ("  ", styles.favorite_selected),
                    (true, true, true) => ("  ", styles.favorite_selected),
                    (_, true, false) => ("  ", styles.favorite),
                    (_, true, true) => ("  ", styles.favorite),
                    (true, false, false) => ("  ", styles.selected),
                    (true, false, true) => ("  ", styles.selected),
                    (_, _, true) => ("  ", styles.normal),
                    (_, _, _) => ("  ", styles.normal),
                };

                ListItem::new(Line::from(vec![Span::raw(icon), Span::raw(&item.item)])).style(style)
            })
            .collect()
    } else {
        items[start..end]
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let list = app.current_list_name();
                let in_list = app.navigation.view == View::Items;
                let is_downloaded = app.is_downloaded(&item.item);
                let is_favorite = app.favorite.is_favorite(list, &item.item);
                let selected = i == app.scroll.selected;

                let (icon, style) = match (selected, is_favorite, in_list, is_downloaded) {
                    (true, true, true, false) => ("  ", styles.favorite_selected),
                    (true, true, true, true) => ("  ", styles.favorite_selected),
                    (_, true, _, false) => ("  ", styles.favorite),
                    (_, true, _, true) => ("  ", styles.favorite),
                    (true, false, true, false) => ("  ", styles.selected),
                    (true, false, true, true) => ("  ", styles.selected),
                    (_, _, _, true) => ("  ", styles.normal),
                    (_, _, _, _) => ("  ", styles.normal),
                };

                ListItem::new(Line::from(vec![Span::raw(icon), Span::raw(&item.item)])).style(style)
            })
            .collect()
    };

    let list = List::new(items_display).block(block);

    frame.render_widget(list, area);
}
