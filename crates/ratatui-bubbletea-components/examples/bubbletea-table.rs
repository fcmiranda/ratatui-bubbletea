//! Port of Bubble Tea's `examples/table` idea.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::Row;
use ratatui_bubbletea_components::ThemedTable;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rows = [
        Row::new(["Charm", "done", "theme"]),
        Row::new(["Bubbles", "doing", "components"]),
        Row::new(["Tea", "next", "runtime"]),
    ];
    let table = ThemedTable::new(
        rows,
        [
            Constraint::Length(12),
            Constraint::Length(10),
            Constraint::Length(14),
        ],
    )
    .title("Tasks")
    .header(Row::new(["Project", "State", "Area"]))
    .selected(Some(1));

    let backend = TestBackend::new(48, 6);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        frame.render_widget(&table, Rect::new(0, 0, 48, 6));
    })?;

    Ok(())
}
