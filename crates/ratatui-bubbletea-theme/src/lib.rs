//! Charm/Bubble Tea-inspired theme helpers for ratatui.
//!
//! This crate is intentionally usable from a normal ratatui event loop. It
//! does not own rendering, terminal setup, or event handling.

use std::borrow::Cow;

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

/// Default theme type for Charm/Bubble Tea-inspired ratatui apps.
pub type BubbleTheme = Theme;

/// Small set of symbols used by Charm-like terminal UIs.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Symbols {
    /// Prefix for ordinary list items.
    pub bullet: &'static str,
    /// Prefix for selected list items.
    pub selected: &'static str,
    /// Success marker.
    pub check: &'static str,
    /// Error/failure marker.
    pub cross: &'static str,
    /// Separator between compact help bindings.
    pub help_separator: &'static str,
}

impl Default for Symbols {
    fn default() -> Self {
        Self {
            bullet: "•",
            selected: "▸",
            check: "✓",
            cross: "✗",
            help_separator: " • ",
        }
    }
}

/// Color palette behind [`Theme`]'s semantic styles.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Palette {
    /// Main foreground color.
    pub foreground: Color,
    /// Muted text color for hints and secondary copy.
    pub muted: Color,
    /// Primary accent color.
    pub accent: Color,
    /// Success color.
    pub success: Color,
    /// Warning color.
    pub warning: Color,
    /// Error color.
    pub error: Color,
    /// Normal border color.
    pub border: Color,
    /// Focused border color.
    pub focused_border: Color,
    /// Selected item background color.
    pub selected_background: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            foreground: Color::Rgb(230, 230, 230),
            muted: Color::Rgb(122, 122, 122),
            accent: Color::Rgb(255, 117, 191),
            success: Color::Rgb(4, 211, 97),
            warning: Color::Rgb(255, 193, 7),
            error: Color::Rgb(255, 83, 112),
            border: Color::Rgb(92, 92, 92),
            focused_border: Color::Rgb(123, 201, 255),
            selected_background: Color::Rgb(48, 48, 48),
        }
    }
}

/// Semantic style tokens and widget helpers for a Charm-like ratatui UI.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Theme {
    /// Source palette used to build semantic styles.
    pub palette: Palette,
    /// Shared symbols used by helper methods.
    pub symbols: Symbols,
    /// Main text style.
    pub text: Style,
    /// Muted secondary text style.
    pub muted: Style,
    /// Primary accent style.
    pub accent: Style,
    /// Success style.
    pub success: Style,
    /// Warning style.
    pub warning: Style,
    /// Error style.
    pub error: Style,
    /// Normal border style.
    pub border: Style,
    /// Focused border style.
    pub focused_border: Style,
    /// Block title style.
    pub title: Style,
    /// Selected row/item style.
    pub selected: Style,
    /// Help key style.
    pub help_key: Style,
    /// Help description style.
    pub help_desc: Style,
}

impl Theme {
    /// Builds a theme from a palette and symbol set.
    #[must_use]
    pub fn new(palette: Palette, symbols: Symbols) -> Self {
        Self {
            palette,
            symbols,
            text: Style::new().fg(palette.foreground),
            muted: Style::new().fg(palette.muted),
            accent: Style::new().fg(palette.accent),
            success: Style::new().fg(palette.success),
            warning: Style::new().fg(palette.warning),
            error: Style::new().fg(palette.error),
            border: Style::new().fg(palette.border),
            focused_border: Style::new().fg(palette.focused_border),
            title: Style::new().fg(palette.accent).add_modifier(Modifier::BOLD),
            selected: Style::new()
                .fg(palette.foreground)
                .bg(palette.selected_background)
                .add_modifier(Modifier::BOLD),
            help_key: Style::new().fg(palette.accent).add_modifier(Modifier::BOLD),
            help_desc: Style::new().fg(palette.muted),
        }
    }

    /// Creates a rounded bordered block using the theme's default border and title styles.
    #[must_use]
    pub fn block<'a>(&self) -> Block<'a> {
        self.block_with_focus(false)
    }

    /// Creates a rounded bordered block with focus-aware border styling.
    #[must_use]
    pub fn block_with_focus<'a>(&self, focused: bool) -> Block<'a> {
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(if focused {
                self.focused_border
            } else {
                self.border
            })
            .title_style(self.title)
    }

    /// Creates a paragraph using the theme's normal text style.
    #[must_use]
    pub fn paragraph<'a, T>(&self, text: T) -> Paragraph<'a>
    where
        T: Into<Text<'a>>,
    {
        Paragraph::new(text).style(self.text)
    }

    /// Creates a paragraph with a themed block attached.
    #[must_use]
    pub fn paragraph_in_block<'a, T, L>(&self, text: T, title: L) -> Paragraph<'a>
    where
        T: Into<Text<'a>>,
        L: Into<Line<'a>>,
    {
        Paragraph::new(text)
            .style(self.text)
            .block(self.block().title(title))
    }

    /// Creates a normal text span.
    #[must_use]
    pub fn span<'a, T>(&self, content: T) -> Span<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Span::styled(content, self.text)
    }

    /// Creates a muted text span.
    #[must_use]
    pub fn muted<'a, T>(&self, content: T) -> Span<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Span::styled(content, self.muted)
    }

    /// Creates an accent text span.
    #[must_use]
    pub fn accent<'a, T>(&self, content: T) -> Span<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Span::styled(content, self.accent)
    }

    /// Creates a success text span.
    #[must_use]
    pub fn success<'a, T>(&self, content: T) -> Span<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Span::styled(content, self.success)
    }

    /// Creates a warning text span.
    #[must_use]
    pub fn warning<'a, T>(&self, content: T) -> Span<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Span::styled(content, self.warning)
    }

    /// Creates an error text span.
    #[must_use]
    pub fn error<'a, T>(&self, content: T) -> Span<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Span::styled(content, self.error)
    }

    /// Creates a title line.
    #[must_use]
    pub fn title<'a, T>(&self, content: T) -> Line<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Line::styled(content, self.title)
    }

    /// Creates a line prefixed with the default bullet symbol.
    #[must_use]
    pub fn bullet<'a, T>(&self, content: T) -> Line<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Line::from(vec![
            self.muted(self.symbols.bullet),
            self.span(" "),
            self.span(content),
        ])
    }

    /// Creates a success line prefixed with the default check symbol.
    #[must_use]
    pub fn checked<'a, T>(&self, content: T) -> Line<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Line::from(vec![
            self.success(self.symbols.check),
            self.span(" "),
            self.span(content),
        ])
    }

    /// Creates an error line prefixed with the default cross symbol.
    #[must_use]
    pub fn crossed<'a, T>(&self, content: T) -> Line<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Line::from(vec![
            self.error(self.symbols.cross),
            self.span(" "),
            self.span(content),
        ])
    }

    /// Creates a compact help line from `(key, description)` pairs.
    #[must_use]
    pub fn help_line<'a, I, K, D>(&self, bindings: I) -> Line<'a>
    where
        I: IntoIterator<Item = (K, D)>,
        K: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>,
    {
        let mut spans = Vec::new();

        for (index, (key, description)) in bindings.into_iter().enumerate() {
            if index > 0 {
                spans.push(self.muted(self.symbols.help_separator));
            }

            spans.push(Span::styled(key, self.help_key));
            spans.push(self.muted(" "));
            spans.push(Span::styled(description, self.help_desc));
        }

        Line::from(spans)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new(Palette::default(), Symbols::default())
    }
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::layout::Rect;
    use ratatui::style::{Color, Modifier, Style};

    use super::{BubbleTheme, Palette, Symbols, Theme};

    #[test]
    fn default_theme_maps_palette_to_semantic_styles() {
        let theme = Theme::default();
        let palette = Palette::default();

        assert_eq!(theme.text, Style::new().fg(palette.foreground));
        assert_eq!(theme.muted, Style::new().fg(palette.muted));
        assert_eq!(theme.accent, Style::new().fg(palette.accent));
        assert_eq!(theme.border, Style::new().fg(palette.border));
        assert_eq!(
            theme.focused_border,
            Style::new().fg(palette.focused_border)
        );
        assert_eq!(theme.title.add_modifier, Modifier::BOLD);
        assert_eq!(theme.selected.bg, Some(palette.selected_background));
    }

    #[test]
    fn custom_palette_is_reflected_in_styles() {
        let palette = Palette {
            foreground: Color::White,
            muted: Color::DarkGray,
            accent: Color::Cyan,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            border: Color::Gray,
            focused_border: Color::Blue,
            selected_background: Color::Black,
        };
        let theme = Theme::new(palette, Symbols::default());

        assert_eq!(theme.accent.fg, Some(Color::Cyan));
        assert_eq!(theme.focused_border.fg, Some(Color::Blue));
        assert_eq!(theme.selected.bg, Some(Color::Black));
    }

    #[test]
    fn helper_lines_include_expected_symbols_and_text() {
        let theme = BubbleTheme::default();

        assert_eq!(theme.bullet("Item").to_string(), "• Item");
        assert_eq!(theme.checked("Done").to_string(), "✓ Done");
        assert_eq!(theme.crossed("Failed").to_string(), "✗ Failed");
    }

    #[test]
    fn help_line_renders_compact_bindings() {
        let theme = Theme::default();
        let line = theme.help_line([("q", "quit"), ("?", "help")]);

        assert_eq!(line.to_string(), "q quit • ? help");
        assert_eq!(line.spans[0].style, theme.help_key);
        assert_eq!(line.spans[2].style, theme.help_desc);
    }

    #[test]
    fn themed_block_renders_rounded_border_and_title() -> Result<(), Box<dyn std::error::Error>> {
        let theme = Theme::default();
        let backend = TestBackend::new(12, 3);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|frame| {
            frame.render_widget(theme.block().title("Demo"), frame.area());
        })?;

        let buffer = terminal.backend().buffer();
        assert_eq!(buffer[(0, 0)].symbol(), "╭");
        assert_eq!(buffer[(1, 0)].symbol(), "D");
        assert_eq!(buffer[(4, 0)].symbol(), "o");
        assert_eq!(buffer[(11, 0)].symbol(), "╮");
        assert_eq!(buffer[(0, 2)].symbol(), "╰");
        assert_eq!(buffer[(11, 2)].symbol(), "╯");

        assert_eq!(buffer[(0, 0)].fg, theme.palette.border);
        assert_eq!(buffer[(1, 0)].fg, theme.palette.accent);
        assert!(buffer[(1, 0)].modifier.contains(Modifier::BOLD));

        Ok(())
    }

    #[test]
    fn focused_block_uses_focused_border_style() -> Result<(), Box<dyn std::error::Error>> {
        let theme = Theme::default();
        let backend = TestBackend::new(4, 3);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|frame| {
            frame.render_widget(theme.block_with_focus(true), frame.area());
        })?;

        assert_eq!(
            terminal.backend().buffer()[(0, 0)].fg,
            theme.palette.focused_border
        );

        Ok(())
    }

    #[test]
    fn paragraph_helper_applies_text_style() -> Result<(), Box<dyn std::error::Error>> {
        let theme = Theme::default();
        let backend = TestBackend::new(5, 1);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|frame| {
            frame.render_widget(theme.paragraph("Hello"), Rect::new(0, 0, 5, 1));
        })?;

        let buffer = terminal.backend().buffer();
        assert_eq!(buffer[(0, 0)].symbol(), "H");
        assert_eq!(buffer[(4, 0)].symbol(), "o");
        assert_eq!(buffer[(0, 0)].fg, theme.palette.foreground);

        Ok(())
    }
}
