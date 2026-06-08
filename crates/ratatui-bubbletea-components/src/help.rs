use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use ratatui_bubbletea_theme::BubbleTheme;

use crate::key::KeyBinding;

/// Display mode for the [`Help`] widget.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum HelpMode {
    /// One compact line: `q quit • ? help`.
    #[default]
    Compact,
    /// One binding per line.
    Expanded,
}

/// A compact or expanded help footer for key bindings.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Help {
    bindings: Vec<KeyBinding>,
    mode: HelpMode,
    theme: BubbleTheme,
}

impl Help {
    /// Creates a compact help widget from key bindings.
    #[must_use]
    pub fn new(bindings: impl IntoIterator<Item = KeyBinding>) -> Self {
        Self {
            bindings: bindings.into_iter().collect(),
            mode: HelpMode::Compact,
            theme: BubbleTheme::default(),
        }
    }

    /// Sets the display mode.
    #[must_use]
    pub const fn mode(mut self, mode: HelpMode) -> Self {
        self.mode = mode;
        self
    }

    /// Uses expanded display mode.
    #[must_use]
    pub const fn expanded(self) -> Self {
        self.mode(HelpMode::Expanded)
    }

    /// Uses compact display mode.
    #[must_use]
    pub const fn compact(self) -> Self {
        self.mode(HelpMode::Compact)
    }

    /// Sets the theme.
    #[must_use]
    pub const fn theme(mut self, theme: BubbleTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Returns the configured bindings.
    #[must_use]
    pub fn bindings(&self) -> &[KeyBinding] {
        &self.bindings
    }

    /// Returns the current display mode.
    #[must_use]
    pub const fn display_mode(&self) -> HelpMode {
        self.mode
    }

    fn render_compact(&self, area: Rect, buf: &mut Buffer) {
        let line = self.theme.help_line(
            self.bindings
                .iter()
                .map(|binding| (binding.label(), binding.description().to_owned())),
        );
        buf.set_line(area.x, area.y, &line, area.width);
    }

    fn render_expanded(&self, area: Rect, buf: &mut Buffer) {
        for (offset, binding) in (0..area.height).zip(self.bindings.iter()) {
            let line = self
                .theme
                .help_line([(binding.label(), binding.description().to_owned())]);
            buf.set_line(area.x, area.y + offset, &line, area.width);
        }
    }
}

impl Widget for &Help {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() {
            return;
        }

        match self.mode {
            HelpMode::Compact => self.render_compact(area, buf),
            HelpMode::Expanded => self.render_expanded(area, buf),
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::layout::Rect;

    use super::{Help, HelpMode};
    use crate::KeyBinding;

    #[test]
    fn help_defaults_to_compact_mode() {
        let help = Help::new([KeyBinding::new("q", "quit")]);

        assert_eq!(help.display_mode(), HelpMode::Compact);
        assert_eq!(help.bindings().len(), 1);
    }

    #[test]
    fn compact_help_renders_single_line() -> Result<(), Box<dyn std::error::Error>> {
        let help = Help::new([
            KeyBinding::with_keys(["q", "esc"], "quit"),
            KeyBinding::new("?", "help"),
        ]);
        let backend = TestBackend::new(24, 1);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|frame| {
            frame.render_widget(&help, Rect::new(0, 0, 24, 1));
        })?;

        let buffer = terminal.backend().buffer();
        let rendered = (0..18).map(|x| buffer[(x, 0)].symbol()).collect::<String>();
        assert_eq!(rendered, "q/esc quit • ? hel");
        assert_eq!(buffer[(0, 0)].fg, help.theme.palette.accent);
        assert_eq!(buffer[(6, 0)].fg, help.theme.palette.muted);

        Ok(())
    }

    #[test]
    fn expanded_help_renders_one_binding_per_line() -> Result<(), Box<dyn std::error::Error>> {
        let help = Help::new([KeyBinding::new("j", "down"), KeyBinding::new("k", "up")]).expanded();
        let backend = TestBackend::new(10, 2);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|frame| {
            frame.render_widget(&help, Rect::new(0, 0, 10, 2));
        })?;

        let buffer = terminal.backend().buffer();
        let first = (0..6).map(|x| buffer[(x, 0)].symbol()).collect::<String>();
        let second = (0..4).map(|x| buffer[(x, 1)].symbol()).collect::<String>();

        assert_eq!(first, "j down");
        assert_eq!(second, "k up");

        Ok(())
    }
}
