//! Port of Bubble Tea's `examples/spinner` idea.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui_bubbletea_components::{Spinner, SpinnerFrames};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut spinner = Spinner::new().label("Fetching charm...");
    spinner.tick();
    spinner.tick();

    let mut fallback = Spinner::new()
        .frames(SpinnerFrames::LINE)
        .label("ASCII fallback");
    fallback.tick();

    let backend = TestBackend::new(48, 2);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        frame.render_widget(&spinner, Rect::new(0, 0, 48, 1));
        frame.render_widget(&fallback, Rect::new(0, 1, 48, 1));
    })?;

    Ok(())
}
