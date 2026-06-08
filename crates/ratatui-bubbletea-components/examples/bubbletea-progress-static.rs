//! Port of Bubble Tea's `examples/progress-static` idea.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui_bubbletea_components::Progress;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let steps = [10, 33, 66, 100];
    let bars = steps.map(|percent| {
        Progress::from_percent(percent)
            .width(24)
            .label(format!("step {percent}"))
    });

    let backend = TestBackend::new(48, 4);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        for (row, progress) in (0..4).zip(bars.iter()) {
            frame.render_widget(progress, Rect::new(0, row, 48, 1));
        }
    })?;

    Ok(())
}
