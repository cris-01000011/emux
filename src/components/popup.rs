use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
};

use crate::{app::App, components::popups::downloading};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ActivePopup {
    #[default]
    None,
    Downloading,
}

#[derive(Default)]
pub struct Popup {
    pub active: ActivePopup,
}

impl Popup {
    pub fn render_popup(&self, frame: &mut Frame, app: &App) {
        match self.active {
            ActivePopup::Downloading => {
                downloading::render_popup(frame, app, frame.area());
            }
            ActivePopup::None => {}
        }
    }

    pub fn popup_area(area: Rect, x: u16, y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Length(y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Length(x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }

    pub fn open(&mut self, popup: ActivePopup) {
        self.active = popup;
    }

    pub fn close(&mut self) {
        self.active = ActivePopup::None;
    }
}
