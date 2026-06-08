//! Port of Bubble Tea's `examples/tabs` idea using ratatui primitives plus the theme.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui_bubbletea_theme::BubbleTheme;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme = BubbleTheme::default();
    let tabs = Line::from(vec![
        Span::styled(" Home ", theme.selected),
        theme.muted(" Settings "),
        theme.muted(" Logs "),
    ]);
    let body = theme.paragraph_in_block("Welcome to the themed tab view.", "Home");

    let backend = TestBackend::new(48, 5);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        frame.render_widget(Paragraph::new(tabs), Rect::new(0, 0, 48, 1));
        frame.render_widget(body, Rect::new(0, 2, 48, 3));
    })?;

    Ok(())
}
