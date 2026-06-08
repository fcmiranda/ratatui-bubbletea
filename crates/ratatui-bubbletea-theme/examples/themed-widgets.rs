use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Margin;
use ratatui_bubbletea_theme::BubbleTheme;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme = BubbleTheme::default();
    let backend = TestBackend::new(48, 8);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        let area = frame.area();
        let inner = area.inner(Margin::new(2, 1));

        frame.render_widget(
            theme
                .block_with_focus(true)
                .title(theme.title("ratatui-bubbletea")),
            area,
        );

        frame.render_widget(
            theme.paragraph(vec![
                theme.bullet("Plain ratatui loop"),
                theme.checked("Charm-like defaults"),
                theme.help_line([("q", "quit"), ("?", "help")]),
            ]),
            inner,
        );
    })?;

    Ok(())
}
