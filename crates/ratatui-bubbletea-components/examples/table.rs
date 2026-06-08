use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::Row;
use ratatui_bubbletea_components::ThemedTable;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let table = ThemedTable::new(
        [
            Row::new(["Theme", "done"]),
            Row::new(["Help", "done"]),
            Row::new(["Table", "active"]),
        ],
        [Constraint::Length(16), Constraint::Length(12)],
    )
    .title("Milestones")
    .header(Row::new(["Component", "State"]))
    .selected(Some(2));

    let backend = TestBackend::new(40, 6);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        frame.render_widget(&table, Rect::new(0, 0, 40, 6));
    })?;

    Ok(())
}
