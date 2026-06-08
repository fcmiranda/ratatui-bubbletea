use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui_bubbletea_components::Viewport;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut viewport = Viewport::new([
        "ratatui-bubbletea",
        "theme helpers",
        "help footer",
        "spinner",
        "progress",
        "list",
        "table",
        "text input",
        "viewport",
    ]);
    viewport.bottom(5);

    let backend = TestBackend::new(32, 5);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        frame.render_widget(&viewport, Rect::new(0, 0, 32, 5));
    })?;

    Ok(())
}
