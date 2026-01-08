use std::{error::Error, io};

use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

mod app;
mod ui;

use crate::app::App;
use crate::ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        let _ = terminal.draw(|f| ui(f, app));

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            // Handle search mode input
            if app.in_search_mode {
                match key.code {
                    KeyCode::Esc => app.stop_search(),
                    KeyCode::Backspace => app.remove_search_char(),
                    KeyCode::Enter => {
                        app.stop_search();
                        app.enter(); // Execute action on selected result
                    }
                    KeyCode::Up => app.search_up(),
                    KeyCode::Down => app.search_down(),
                    KeyCode::Char(c) => app.add_search_char(c),
                    _ => {}
                }
                continue;
            }

            match key.code {
                KeyCode::Up => app.move_up(),
                KeyCode::Down => app.move_down(),
                KeyCode::Right => app.enter(),
                KeyCode::Enter => app.enter(),
                KeyCode::Left => app.go_back(),
                KeyCode::Backspace => app.go_back(),
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('x') => app.jump_to_random(),
                KeyCode::Char('b') => app.open_browser_search(),
                KeyCode::Char('s') => app.start_search(),
                KeyCode::Char('f') => app.toggle_favorite(),
                KeyCode::Char('g') => app.go_to_first_item(),
                KeyCode::Char('G') => app.go_to_last_item(),
                KeyCode::Tab => {
                    if !app.in_command_selection {
                        app.toggle_command_selection();
                    } else {
                        app.next_command();
                    }
                }
                KeyCode::BackTab => app.prev_command(),
                _ => {}
            }
        }
    }
}
