use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListItem},
};

use crate::{actions::navigation::View, app::App};

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

    render_directory_list(frame, app, area, &styles, &panel_block);
}

fn render_center_panel(frame: &mut ratatui::Frame, app: &mut App, area: Rect) {
    let styles = CenterPanelStyles::new();
    let panel_block = create_panel_block();

    render_items_list(frame, app, area, &styles, &panel_block);
}

fn render_right_panel(frame: &mut ratatui::Frame, _app: &App, area: Rect) {
    let panel_block = create_panel_block();
    frame.render_widget(panel_block, area);
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
            normal: Style::default().fg(Color::Rgb(148, 226, 213)),
            selected: Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(148, 226, 213)),
            favorite: Style::default().fg(Color::Rgb(245, 194, 231)),
            _favorite_selected: Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(245, 194, 231)),
            icon: Style::default().fg(Color::Rgb(249, 226, 175)),
        }
    }
}

fn create_panel_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Color::Rgb(137, 180, 250))
}

fn render_directory_list(
    frame: &mut ratatui::Frame,
    app: &mut App,
    area: Rect,
    styles: &LeftPanelStyles,
    block: &Block<'static>,
) {
    let items: Vec<ListItem> = app
        .data
        .lists
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

    frame.render_stateful_widget(list, area, &mut app.ui_state.lists);
}

fn render_items_list(
    frame: &mut ratatui::Frame,
    app: &mut App,
    area: Rect,
    styles: &CenterPanelStyles,
    block: &Block<'static>,
) {
    let items: Vec<ListItem> = app
        .data
        .items_in_list
        .iter()
        .enumerate()
        .map(|(_, item)| {
            let is_favorite = app
                .favorite
                .is_favorite(&app.navigation.current_list, &item.item);
            let in_list = app.navigation.view == View::Items;

            let (icon, icon_style) = if is_favorite {
                ("󰋑 ", styles.favorite)
            } else {
                (" ", styles.icon)
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
        .collect();

    let mut list = List::new(items).block(block.clone());

    if app.navigation.view == View::Items {
        list = list.highlight_style(styles.selected);
    }

    frame.render_stateful_widget(list, area, &mut app.ui_state.items_in_list);
}

fn extract_display_name(path: &std::path::Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("<unknown>")
        .to_string()
}
