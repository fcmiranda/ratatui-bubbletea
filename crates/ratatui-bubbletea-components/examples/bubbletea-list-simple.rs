//! Port of Bubble Tea's `examples/list-simple` idea.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui_bubbletea_components::{ListItem, SelectList};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut list = SelectList::new([
        ListItem::new("Ratatui").description("renderer"),
        ListItem::new("Bubble Tea").description("update/view inspiration"),
        ListItem::new("Lip Gloss").description("theme inspiration"),
        ListItem::new("Bubbles").description("component inspiration"),
    ]);
    list.next();

    let backend = TestBackend::new(64, 4);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        frame.render_widget(&list, Rect::new(0, 0, 64, 4));
    })?;

    Ok(())
}
