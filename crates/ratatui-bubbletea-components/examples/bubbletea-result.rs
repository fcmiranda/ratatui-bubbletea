//! Port of Bubble Tea's `examples/result` idea.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui_bubbletea_components::{ListItem, SelectList};
use ratatui_bubbletea_theme::BubbleTheme;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme = BubbleTheme::default();
    let mut choices = SelectList::new([
        ListItem::new("Plant carrots"),
        ListItem::new("Water basil"),
        ListItem::new("Harvest tomatoes"),
    ]);
    choices.next();
    choices.next();

    let selected = choices
        .selected_item()
        .map_or("nothing selected", ListItem::label);
    let backend = TestBackend::new(48, 7);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        frame.render_widget(&choices, Rect::new(0, 0, 48, 3));
        frame.render_widget(
            theme.paragraph_in_block(format!("You chose: {selected}"), "Result"),
            Rect::new(0, 4, 48, 3),
        );
    })?;

    Ok(())
}
