use std::{io::Result, time::Duration};
use tokio_stream::StreamExt;

use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use tui_input::backend::crossterm::EventHandler;

use crate::{
    actions::{
        commands::CommandLists,
        data::AppData,
        download::{Download, DownloadEvent},
        favorite::Favorite,
        navigation::Navigation,
    },
    components::{inputs::search::InputMode, popup::ActivePopup},
    config::app::AppConfig,
    ui::{UiState, render_ui},
};

#[derive(Default)]
pub struct AppState {
    should_quit: bool,
}

pub struct App {
    pub commands: CommandLists,
    pub config: AppConfig,
    pub data: AppData,
    pub download: Download,
    pub favorite: Favorite,
    pub navigation: Navigation,
    pub state: AppState,
    pub ui: UiState,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub fn new() -> App {
        let mut app = App {
            commands: Default::default(),
            config: AppConfig::load(),
            data: Default::default(),
            download: Default::default(),
            favorite: Default::default(),
            navigation: Default::default(),
            state: Default::default(),
            ui: Default::default(),
        };

        app.init_lists();
        app.init_command_lists();
        app.init_favorites();

        app
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while !self.state.should_quit {
            tokio::select! {
                _ = interval.tick() => {
                    terminal.draw(|frame| render_ui(frame, &mut self))?;
                },

                Some(event) = async {
                    self.download
                        .rx
                        .as_mut()
                        .and_then(|rx| rx.try_recv().ok())
                } => {
                    match event {
                        DownloadEvent::Progress { percent, downloaded, total } => {
                            let p = (percent * 100.0).min(100.0);
                            self.download.progress = p;
                            self.download.downloaded = downloaded;
                            self.download.total = total;
                        }

                        DownloadEvent::Finished => {
                            self.download.rx = None;
                        }

                        DownloadEvent::Error => {
                            self.download.rx = None;
                        }
                    }
                },

                Some(Ok(event)) = events.next() => {
                    self.handle_event(&event);
                }
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) {
        if let Some(key) = event.as_key_press_event() {
            if self.ui.popup.active == ActivePopup::Downloading {
                match key.code {
                    KeyCode::Esc => self.ui.popup.close(),
                    KeyCode::Enter => self.execute_command(),
                    KeyCode::Tab => self.commands.next_command(),
                    KeyCode::BackTab => self.commands.prev_command(),
                    _ => {}
                }
                return;
            }

            match self.ui.search.mode {
                InputMode::Editing => {
                    match key.code {
                        KeyCode::Esc => self.stop_search(),
                        KeyCode::Enter => self.stop_search(),
                        KeyCode::Up => self.move_up(),
                        KeyCode::Down => self.move_down(),
                        _ => {
                            self.ui.search.input.handle_event(&event);
                            self.reload_data();
                        }
                    }
                    return;
                }
                InputMode::Normal => {}
            }

            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up => self.move_up(),
                    KeyCode::Down => self.move_down(),
                    KeyCode::Right => self.open_list(),
                    KeyCode::Left => self.go_back(),
                    KeyCode::Enter => self.open_file(),
                    KeyCode::Backspace => self.go_back(),
                    KeyCode::Tab => self.commands.next_command(),
                    KeyCode::BackTab => self.commands.prev_command(),
                    KeyCode::Char('/') => self.start_search(),
                    KeyCode::Char('b') => self.open_browser_search(),
                    KeyCode::Char('F') => self.toggle_favorites_mode(),
                    KeyCode::Char('f') => self.toggle_favorite(),
                    KeyCode::Char('g') => self.go_to_first_item(),
                    KeyCode::Char('G') => self.go_to_last_item(),
                    KeyCode::Char('x') => self.jump_to_random(),
                    KeyCode::Char('q') => self.state.should_quit = true,
                    _ => {}
                }
            }
        }
    }
}
