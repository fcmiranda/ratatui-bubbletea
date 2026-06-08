use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui_bubbletea_components::{Help, KeyBinding};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let help = Help::new([
        KeyBinding::with_keys(["q", "esc", "ctrl+c"], "quit"),
        KeyBinding::with_keys(["j", "down"], "down"),
        KeyBinding::with_keys(["k", "up"], "up"),
        KeyBinding::new("?", "help"),
    ]);

    let backend = TestBackend::new(48, 1);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        frame.render_widget(&help, Rect::new(0, 0, 48, 1));
    })?;

    Ok(())
}
