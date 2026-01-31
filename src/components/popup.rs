use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
};

use crate::{
    app::App,
    components::popups::{downloading, new_list::render_popup_new_list},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ActivePopup {
    #[default]
    None,
    Downloading,
    NewList,
}

#[derive(Default)]
pub struct Popup {
    pub active: ActivePopup,
}

impl Popup {
    pub fn open(&mut self, popup: ActivePopup) {
        self.active = popup;
    }

    pub fn close(&mut self) {
        self.active = ActivePopup::None;
    }

    pub fn popup_area(area: Rect, x: u16, y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Length(y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Length(x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }
}

pub fn render_popup(frame: &mut Frame, app: &App) {
    match app.ui.popup.active {
        ActivePopup::Downloading => {
            downloading::render_popup(frame, app, frame.area());
        }
        ActivePopup::NewList => {
            render_popup_new_list(app, frame, frame.area());
        }
        ActivePopup::None => {}
    }
}
