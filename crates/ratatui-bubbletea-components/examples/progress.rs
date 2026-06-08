use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui_bubbletea_components::Progress;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let progress = Progress::from_ratio(0.42).width(24).label("Downloading");
    let backend = TestBackend::new(48, 1);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        frame.render_widget(&progress, Rect::new(0, 0, 48, 1));
    })?;

    Ok(())
}
