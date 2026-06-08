use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui_bubbletea_theme::BubbleTheme;

/// Scroll state for a [`Viewport`].
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ViewportState {
    offset: usize,
}

impl ViewportState {
    /// Creates a viewport state at the top.
    #[must_use]
    pub const fn new() -> Self {
        Self { offset: 0 }
    }

    /// Returns the top visible line offset.
    #[must_use]
    pub const fn offset(self) -> usize {
        self.offset
    }

    /// Sets the offset, clamped to content length and viewport height.
    pub fn set_offset(&mut self, offset: usize, content_len: usize, visible_height: usize) {
        self.offset = offset.min(max_offset(content_len, visible_height));
    }

    /// Scrolls down by `amount` lines.
    pub fn scroll_down(&mut self, amount: usize, content_len: usize, visible_height: usize) {
        self.set_offset(
            self.offset.saturating_add(amount),
            content_len,
            visible_height,
        );
    }

    /// Scrolls up by `amount` lines.
    pub fn scroll_up(&mut self, amount: usize, content_len: usize, visible_height: usize) {
        self.set_offset(
            self.offset.saturating_sub(amount),
            content_len,
            visible_height,
        );
    }

    /// Scrolls down by one page.
    pub fn page_down(&mut self, content_len: usize, visible_height: usize) {
        self.scroll_down(visible_height.max(1), content_len, visible_height);
    }

    /// Scrolls up by one page.
    pub fn page_up(&mut self, content_len: usize, visible_height: usize) {
        self.scroll_up(visible_height.max(1), content_len, visible_height);
    }

    /// Moves to the top.
    pub const fn top(&mut self) {
        self.offset = 0;
    }

    /// Moves to the bottom.
    pub fn bottom(&mut self, content_len: usize, visible_height: usize) {
        self.offset = max_offset(content_len, visible_height);
    }
}

/// A simple scrollable viewport for line-oriented content.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Viewport {
    lines: Vec<String>,
    state: ViewportState,
    show_position: bool,
    theme: BubbleTheme,
}

impl Viewport {
    /// Creates a viewport from lines.
    #[must_use]
    pub fn new<L, S>(lines: L) -> Self
    where
        L: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            lines: lines.into_iter().map(Into::into).collect(),
            state: ViewportState::new(),
            show_position: true,
            theme: BubbleTheme::default(),
        }
    }

    /// Sets the theme.
    #[must_use]
    pub const fn theme(mut self, theme: BubbleTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Enables or disables the bottom-right position indicator.
    #[must_use]
    pub const fn show_position(mut self, show_position: bool) -> Self {
        self.show_position = show_position;
        self
    }

    /// Returns the state.
    #[must_use]
    pub const fn state(&self) -> ViewportState {
        self.state
    }

    /// Returns mutable state.
    #[must_use]
    pub const fn state_mut(&mut self) -> &mut ViewportState {
        &mut self.state
    }

    /// Returns the number of content lines.
    #[must_use]
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    /// Returns whether the viewport has no content.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Sets the scroll offset for a given visible height.
    pub fn set_offset(&mut self, offset: usize, visible_height: usize) {
        self.state
            .set_offset(offset, self.lines.len(), visible_height);
    }

    /// Scrolls down by `amount` lines.
    pub fn scroll_down(&mut self, amount: usize, visible_height: usize) {
        self.state
            .scroll_down(amount, self.lines.len(), visible_height);
    }

    /// Scrolls up by `amount` lines.
    pub fn scroll_up(&mut self, amount: usize, visible_height: usize) {
        self.state
            .scroll_up(amount, self.lines.len(), visible_height);
    }

    /// Scrolls down by one page.
    pub fn page_down(&mut self, visible_height: usize) {
        self.state.page_down(self.lines.len(), visible_height);
    }

    /// Scrolls up by one page.
    pub fn page_up(&mut self, visible_height: usize) {
        self.state.page_up(self.lines.len(), visible_height);
    }

    /// Moves to the top.
    pub const fn top(&mut self) {
        self.state.top();
    }

    /// Moves to the bottom for a given visible height.
    pub fn bottom(&mut self, visible_height: usize) {
        self.state.bottom(self.lines.len(), visible_height);
    }

    fn position_label(&self, visible_height: usize) -> String {
        if self.lines.is_empty() {
            return "0/0".to_owned();
        }

        let current = self.state.offset().saturating_add(1);
        let visible_end = self
            .state
            .offset()
            .saturating_add(visible_height)
            .min(self.lines.len());
        format!("{current}-{visible_end}/{}", self.lines.len())
    }
}

impl Widget for &Viewport {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() {
            return;
        }

        for (offset, line) in (0..area.height).zip(self.lines.iter().skip(self.state.offset())) {
            let line = Line::from(vec![self.theme.span(line.as_str())]);
            buf.set_line(area.x, area.y + offset, &line, area.width);
        }

        if self.show_position {
            let label = self.position_label(usize::from(area.height));
            let label_width = u16::try_from(label.len()).unwrap_or(u16::MAX);
            let x = area.right().saturating_sub(label_width);
            let y = area.bottom().saturating_sub(1);
            let line = Line::from(vec![self.theme.muted(label)]);
            buf.set_line(x, y, &line, label_width.min(area.width));
        }
    }
}

const fn max_offset(content_len: usize, visible_height: usize) -> usize {
    content_len.saturating_sub(visible_height)
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::layout::Rect;

    use super::{Viewport, ViewportState};

    #[test]
    fn state_scrolls_with_bounds() {
        let mut state = ViewportState::new();

        state.scroll_down(10, 5, 2);
        assert_eq!(state.offset(), 3);
        state.scroll_up(2, 5, 2);
        assert_eq!(state.offset(), 1);
        state.page_down(5, 2);
        assert_eq!(state.offset(), 3);
        state.page_up(5, 2);
        assert_eq!(state.offset(), 1);
        state.bottom(5, 2);
        assert_eq!(state.offset(), 3);
        state.top();
        assert_eq!(state.offset(), 0);
    }

    #[test]
    fn viewport_scroll_methods_use_content_len() {
        let mut viewport = Viewport::new(["one", "two", "three", "four"]);

        viewport.bottom(2);
        assert_eq!(viewport.state().offset(), 2);
        viewport.scroll_up(1, 2);
        assert_eq!(viewport.state().offset(), 1);
        viewport.set_offset(99, 2);
        assert_eq!(viewport.state().offset(), 2);
    }

    #[test]
    fn viewport_renders_visible_lines_and_position() -> Result<(), Box<dyn std::error::Error>> {
        let mut viewport = Viewport::new(["one", "two", "three", "four"]);
        viewport.scroll_down(1, 2);
        let backend = TestBackend::new(12, 2);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|frame| {
            frame.render_widget(&viewport, Rect::new(0, 0, 12, 2));
        })?;

        let buffer = terminal.backend().buffer();
        let first = (0..3).map(|x| buffer[(x, 0)].symbol()).collect::<String>();
        let second = (0..5).map(|x| buffer[(x, 1)].symbol()).collect::<String>();
        let label = (7..12).map(|x| buffer[(x, 1)].symbol()).collect::<String>();

        assert_eq!(first, "two");
        assert_eq!(second, "three");
        assert_eq!(label, "2-3/4");
        assert_eq!(buffer[(7, 1)].fg, viewport.theme.palette.muted);

        Ok(())
    }

    #[test]
    fn viewport_can_hide_position() -> Result<(), Box<dyn std::error::Error>> {
        let viewport = Viewport::new(["one"]).show_position(false);
        let backend = TestBackend::new(6, 1);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|frame| {
            frame.render_widget(&viewport, Rect::new(0, 0, 6, 1));
        })?;

        let buffer = terminal.backend().buffer();
        let rendered = (0..6).map(|x| buffer[(x, 0)].symbol()).collect::<String>();

        assert_eq!(rendered, "one   ");

        Ok(())
    }
}
