use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui_bubbletea_components::{ListItem, SelectList};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut list = SelectList::new([
        ListItem::new("Clone repositories").description("done"),
        ListItem::new("Build theme").description("in progress"),
        ListItem::new("Add components").description("next"),
    ]);
    list.next();

    let backend = TestBackend::new(48, 3);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        frame.render_widget(&list, Rect::new(0, 0, 48, 3));
    })?;

    Ok(())
}
