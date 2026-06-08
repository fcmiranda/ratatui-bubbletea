use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui_bubbletea_components::TextInput;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = TextInput::new().placeholder("Search").focused(true);
    for ch in "ratatui".chars() {
        input.insert(ch);
    }

    let backend = TestBackend::new(24, 1);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        frame.render_widget(&input, Rect::new(0, 0, 24, 1));
    })?;

    Ok(())
}
