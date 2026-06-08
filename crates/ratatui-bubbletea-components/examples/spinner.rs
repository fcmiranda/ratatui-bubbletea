use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui_bubbletea_components::{Spinner, SpinnerFrames};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut spinner = Spinner::new()
        .frames(SpinnerFrames::DOTS)
        .label("Loading from a plain ratatui loop");
    let backend = TestBackend::new(48, 1);
    let mut terminal = Terminal::new(backend)?;

    spinner.tick();
    terminal.draw(|frame| {
        frame.render_widget(&spinner, Rect::new(0, 0, 48, 1));
    })?;

    Ok(())
}
