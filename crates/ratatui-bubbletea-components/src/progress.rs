use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui_bubbletea_theme::BubbleTheme;

/// Symbols used to render a [`Progress`] bar.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ProgressSymbols {
    /// Filled segment symbol.
    pub filled: &'static str,
    /// Empty segment symbol.
    pub empty: &'static str,
}

impl Default for ProgressSymbols {
    fn default() -> Self {
        Self {
            filled: "█",
            empty: "░",
        }
    }
}

/// A themed progress bar widget.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Progress {
    percent: u16,
    width: Option<u16>,
    label: Option<String>,
    show_percentage: bool,
    symbols: ProgressSymbols,
    theme: BubbleTheme,
}

impl Progress {
    /// Creates a progress bar from a `0.0..=1.0` ratio.
    #[must_use]
    pub fn from_ratio(ratio: f64) -> Self {
        Self::from_percent(ratio_to_percent(ratio))
    }

    /// Creates a progress bar from a `0..=100` percentage.
    #[must_use]
    pub fn from_percent(percent: u16) -> Self {
        Self {
            percent: clamp_percent(percent),
            width: None,
            label: None,
            show_percentage: true,
            symbols: ProgressSymbols {
                filled: "█",
                empty: "░",
            },
            theme: BubbleTheme::default(),
        }
    }

    /// Sets the bar width. When unset, the render area width is used.
    #[must_use]
    pub const fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets a custom label rendered after the bar.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Enables or disables the automatic percentage label.
    #[must_use]
    pub const fn show_percentage(mut self, show_percentage: bool) -> Self {
        self.show_percentage = show_percentage;
        self
    }

    /// Sets the symbols used for filled and empty segments.
    #[must_use]
    pub const fn symbols(mut self, symbols: ProgressSymbols) -> Self {
        self.symbols = symbols;
        self
    }

    /// Sets the theme.
    #[must_use]
    pub const fn theme(mut self, theme: BubbleTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Returns the clamped percentage.
    #[must_use]
    pub const fn percent(&self) -> u16 {
        self.percent
    }

    /// Returns the ratio as `0.0..=1.0`.
    #[must_use]
    pub fn ratio(&self) -> f64 {
        f64::from(self.percent) / 100.0
    }

    fn label_text(&self) -> Option<String> {
        match (&self.label, self.show_percentage) {
            (Some(label), true) => Some(format!("{label} {}%", self.percent)),
            (Some(label), false) => Some(label.clone()),
            (None, true) => Some(format!("{}%", self.percent)),
            (None, false) => None,
        }
    }

    fn bar_width(&self, area_width: u16, label: Option<&str>) -> u16 {
        if let Some(width) = self.width {
            return width.min(area_width);
        }

        let label_width = label.map_or(0, |label| label.len().saturating_add(1));
        let label_width = u16::try_from(label_width).unwrap_or(u16::MAX);

        area_width.saturating_sub(label_width)
    }

    fn filled_width(&self, bar_width: u16) -> u16 {
        let filled = (u32::from(bar_width) * u32::from(self.percent) + 50) / 100;
        u16::try_from(filled).unwrap_or(bar_width)
    }

    fn line(&self, area_width: u16) -> Line<'_> {
        let label = self.label_text();
        let bar_width = self.bar_width(area_width, label.as_deref());
        let filled_width = self.filled_width(bar_width);
        let empty_width = bar_width.saturating_sub(filled_width);

        let filled = self.symbols.filled.repeat(usize::from(filled_width));
        let empty = self.symbols.empty.repeat(usize::from(empty_width));

        let mut spans = vec![self.theme.accent(filled), self.theme.muted(empty)];

        if let Some(label) = label {
            spans.push(self.theme.span(" "));
            spans.push(self.theme.span(label));
        }

        Line::from(spans)
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self::from_percent(0)
    }
}

impl Widget for &Progress {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() {
            return;
        }

        buf.set_line(area.x, area.y, &self.line(area.width), area.width);
    }
}

const fn clamp_percent(percent: u16) -> u16 {
    if percent > 100 { 100 } else { percent }
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
fn ratio_to_percent(ratio: f64) -> u16 {
    if !ratio.is_finite() || ratio <= 0.0 {
        0
    } else if ratio >= 1.0 {
        100
    } else {
        (ratio * 100.0).round() as u16
    }
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::layout::Rect;

    use super::{Progress, ProgressSymbols};

    #[test]
    fn percent_is_clamped() {
        assert_eq!(Progress::from_percent(0).percent(), 0);
        assert_eq!(Progress::from_percent(50).percent(), 50);
        assert_eq!(Progress::from_percent(101).percent(), 100);
    }

    #[test]
    fn ratio_is_clamped_and_rounded() {
        assert_eq!(Progress::from_ratio(-1.0).percent(), 0);
        assert_eq!(Progress::from_ratio(0.424).percent(), 42);
        assert_eq!(Progress::from_ratio(0.425).percent(), 43);
        assert_eq!(Progress::from_ratio(2.0).percent(), 100);
        assert_eq!(Progress::from_ratio(f64::NAN).percent(), 0);
    }

    #[test]
    fn progress_renders_bar_and_percentage() -> Result<(), Box<dyn std::error::Error>> {
        let progress = Progress::from_percent(50).width(10);
        let backend = TestBackend::new(14, 1);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|frame| {
            frame.render_widget(&progress, Rect::new(0, 0, 14, 1));
        })?;

        let buffer = terminal.backend().buffer();
        let rendered = (0..14).map(|x| buffer[(x, 0)].symbol()).collect::<String>();

        assert_eq!(rendered, "█████░░░░░ 50%");
        assert_eq!(buffer[(0, 0)].fg, progress.theme.palette.accent);
        assert_eq!(buffer[(5, 0)].fg, progress.theme.palette.muted);

        Ok(())
    }

    #[test]
    fn progress_renders_custom_label_without_percentage() -> Result<(), Box<dyn std::error::Error>>
    {
        let progress = Progress::from_percent(25)
            .width(4)
            .label("Loading")
            .show_percentage(false)
            .symbols(ProgressSymbols {
                filled: "=",
                empty: "-",
            });
        let backend = TestBackend::new(12, 1);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|frame| {
            frame.render_widget(&progress, Rect::new(0, 0, 12, 1));
        })?;

        let buffer = terminal.backend().buffer();
        let rendered = (0..12).map(|x| buffer[(x, 0)].symbol()).collect::<String>();

        assert_eq!(rendered, "=--- Loading");

        Ok(())
    }
}
