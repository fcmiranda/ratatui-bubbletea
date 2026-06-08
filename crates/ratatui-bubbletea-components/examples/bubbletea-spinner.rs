//! Port of Bubble Tea's `examples/spinner` idea.

use std::io;
use std::time::Duration;

use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::layout::Rect;
use ratatui::Terminal;
use ratatui_bubbletea_components::{Spinner, SpinnerFrames};

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = setup_terminal()?;

    let mut spinner = Spinner::new().label("Fetching charm...");
    let mut fallback = Spinner::new()
        .frames(SpinnerFrames::LINE)
        .label("ASCII fallback");

    let tick_rate = Duration::from_millis(100);

    loop {
        terminal.draw(|frame| {
            frame.render_widget(&spinner, Rect::new(0, 0, 48, 1));
            frame.render_widget(&fallback, Rect::new(0, 1, 48, 1));
        })?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
            }
        } else {
            spinner.tick();
            fallback.tick();
        }
    }

    restore_terminal(terminal)?;
    Ok(())
}
