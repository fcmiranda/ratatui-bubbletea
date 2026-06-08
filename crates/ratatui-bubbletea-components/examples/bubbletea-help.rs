//! Port of Bubble Tea's `examples/help` idea.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui_bubbletea_components::{Help, KeyBinding};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bindings = [
        KeyBinding::with_keys(["↑", "k"], "up"),
        KeyBinding::with_keys(["↓", "j"], "down"),
        KeyBinding::new("enter", "choose"),
        KeyBinding::new("q", "quit"),
        KeyBinding::new("?", "toggle help"),
    ];

    let compact = Help::new(bindings.clone()).compact();
    let expanded = Help::new(bindings).expanded();
    let backend = TestBackend::new(64, 7);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        frame.render_widget(&compact, Rect::new(0, 0, 64, 1));
        frame.render_widget(&expanded, Rect::new(0, 2, 64, 5));
    })?;

    Ok(())
}
