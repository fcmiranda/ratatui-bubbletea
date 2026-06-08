//! Port of Bubble Tea's `examples/textinput` idea.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui_bubbletea_components::{TextInput, TextInputState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = TextInput::from_state(TextInputState::with_value("Charm"))
        .placeholder("What is your favorite TUI toolkit?")
        .focused(true);
    input.insert('!');
    input.move_left();

    let backend = TestBackend::new(48, 1);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        frame.render_widget(&input, Rect::new(0, 0, 48, 1));
    })?;

    Ok(())
}
