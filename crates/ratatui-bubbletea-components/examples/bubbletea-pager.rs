//! Port of Bubble Tea's `examples/pager` idea.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui_bubbletea_components::Viewport;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut viewport = Viewport::new([
        "# ratatui-bubbletea",
        "",
        "A Charm-inspired component set for ratatui.",
        "",
        "Use the theme crate from any ratatui app.",
        "Adopt components one at a time.",
        "Use ratatui-tea only when you want an Elm-style loop.",
        "",
        "Press j/k in a real app to scroll.",
    ]);
    viewport.scroll_down(3, 5);

    let backend = TestBackend::new(64, 5);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        frame.render_widget(&viewport, Rect::new(0, 0, 64, 5));
    })?;

    Ok(())
}
