use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;
use ratatui_bubbletea_theme::BubbleTheme;

/// Editable state for [`TextInput`].
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct TextInputState {
    value: String,
    cursor: usize,
}

impl TextInputState {
    /// Creates an empty input state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
        }
    }

    /// Creates state with an initial value and cursor at the end.
    #[must_use]
    pub fn with_value(value: impl Into<String>) -> Self {
        let value = value.into();
        let cursor = value.chars().count();
        Self { value, cursor }
    }

    /// Returns the input value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the cursor character offset.
    #[must_use]
    pub const fn cursor(&self) -> usize {
        self.cursor
    }

    /// Returns whether the value is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Inserts a character at the cursor.
    pub fn insert(&mut self, ch: char) {
        let byte_index = self.byte_index(self.cursor);
        self.value.insert(byte_index, ch);
        self.cursor = self.cursor.saturating_add(1);
    }

    /// Deletes the character before the cursor.
    pub fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }

        let start = self.byte_index(self.cursor - 1);
        let end = self.byte_index(self.cursor);
        self.value.replace_range(start..end, "");
        self.cursor -= 1;
    }

    /// Deletes the character at the cursor.
    pub fn delete(&mut self) {
        if self.cursor >= self.len_chars() {
            return;
        }

        let start = self.byte_index(self.cursor);
        let end = self.byte_index(self.cursor + 1);
        self.value.replace_range(start..end, "");
    }

    /// Moves the cursor left.
    pub fn move_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    /// Moves the cursor right.
    pub fn move_right(&mut self) {
        self.cursor = self.cursor.saturating_add(1).min(self.len_chars());
    }

    /// Moves the cursor to the beginning.
    pub const fn home(&mut self) {
        self.cursor = 0;
    }

    /// Moves the cursor to the end.
    pub fn end(&mut self) {
        self.cursor = self.len_chars();
    }

    fn len_chars(&self) -> usize {
        self.value.chars().count()
    }

    fn byte_index(&self, char_index: usize) -> usize {
        self.value
            .char_indices()
            .nth(char_index)
            .map_or(self.value.len(), |(index, _)| index)
    }
}

/// A themed text input widget.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TextInput {
    state: TextInputState,
    placeholder: String,
    focused: bool,
    theme: BubbleTheme,
}

impl TextInput {
    /// Creates an empty text input.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: TextInputState::new(),
            placeholder: String::new(),
            focused: false,
            theme: BubbleTheme::default(),
        }
    }

    /// Creates a text input from existing state.
    #[must_use]
    pub fn from_state(state: TextInputState) -> Self {
        Self {
            state,
            ..Self::new()
        }
    }

    /// Sets placeholder text.
    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Sets focus state.
    #[must_use]
    pub const fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Sets the theme.
    #[must_use]
    pub const fn theme(mut self, theme: BubbleTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Returns state.
    #[must_use]
    pub const fn state(&self) -> &TextInputState {
        &self.state
    }

    /// Returns mutable state.
    #[must_use]
    pub const fn state_mut(&mut self) -> &mut TextInputState {
        &mut self.state
    }

    /// Inserts a character at the cursor.
    pub fn insert(&mut self, ch: char) {
        self.state.insert(ch);
    }

    /// Deletes the character before the cursor.
    pub fn backspace(&mut self) {
        self.state.backspace();
    }

    /// Deletes the character at the cursor.
    pub fn delete(&mut self) {
        self.state.delete();
    }

    /// Moves the cursor left.
    pub fn move_left(&mut self) {
        self.state.move_left();
    }

    /// Moves the cursor right.
    pub fn move_right(&mut self) {
        self.state.move_right();
    }

    /// Returns the desired terminal cursor position for the given render area.
    #[must_use]
    pub fn cursor_position(&self, area: Rect) -> Position {
        let x_offset = u16::try_from(self.state.cursor()).unwrap_or(u16::MAX);
        Position::new(area.x.saturating_add(x_offset).min(area.right()), area.y)
    }

    fn line(&self) -> Line<'_> {
        if self.state.is_empty() {
            if self.focused {
                return Line::from(vec![cursor_span(" ", &self.theme)]);
            }

            return Line::from(vec![self.theme.muted(self.placeholder.as_str())]);
        }

        let mut spans = Vec::new();
        let chars = self.state.value().chars().collect::<Vec<_>>();
        let cursor = self.state.cursor();

        for (index, ch) in chars.iter().enumerate() {
            let text = ch.to_string();
            if self.focused && index == cursor {
                spans.push(cursor_span(text, &self.theme));
            } else {
                spans.push(self.theme.span(text));
            }
        }

        if self.focused && cursor == chars.len() {
            spans.push(cursor_span(" ", &self.theme));
        }

        Line::from(spans)
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &TextInput {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() {
            return;
        }

        buf.set_line(area.x, area.y, &self.line(), area.width);
    }
}

fn cursor_span<'a>(content: impl Into<std::borrow::Cow<'a, str>>, theme: &BubbleTheme) -> Span<'a> {
    Span::styled(
        content,
        theme
            .accent
            .bg(theme.palette.selected_background)
            .add_modifier(Modifier::REVERSED),
    )
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::layout::Rect;
    use ratatui::style::Modifier;

    use super::{TextInput, TextInputState};

    #[test]
    fn state_edits_ascii_text() {
        let mut state = TextInputState::new();

        state.insert('a');
        state.insert('c');
        state.move_left();
        state.insert('b');

        assert_eq!(state.value(), "abc");
        assert_eq!(state.cursor(), 2);

        state.delete();
        assert_eq!(state.value(), "ab");

        state.backspace();
        assert_eq!(state.value(), "a");
        assert_eq!(state.cursor(), 1);
    }

    #[test]
    fn state_handles_unicode_character_boundaries() {
        let mut state = TextInputState::new();

        state.insert('é');
        state.insert('好');
        state.move_left();
        state.backspace();

        assert_eq!(state.value(), "好");
        assert_eq!(state.cursor(), 0);
    }

    #[test]
    fn cursor_movement_is_clamped() {
        let mut state = TextInputState::with_value("abc");

        state.move_right();
        assert_eq!(state.cursor(), 3);
        state.home();
        state.move_left();
        assert_eq!(state.cursor(), 0);
        state.end();
        assert_eq!(state.cursor(), 3);
    }

    #[test]
    fn unfocused_empty_input_renders_placeholder() -> Result<(), Box<dyn std::error::Error>> {
        let input = TextInput::new().placeholder("Search");
        let backend = TestBackend::new(6, 1);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|frame| {
            frame.render_widget(&input, Rect::new(0, 0, 6, 1));
        })?;

        let buffer = terminal.backend().buffer();
        let rendered = (0..6).map(|x| buffer[(x, 0)].symbol()).collect::<String>();

        assert_eq!(rendered, "Search");
        assert_eq!(buffer[(0, 0)].fg, input.theme.palette.muted);

        Ok(())
    }

    #[test]
    fn focused_input_renders_cursor() -> Result<(), Box<dyn std::error::Error>> {
        let mut input = TextInput::from_state(TextInputState::with_value("abc")).focused(true);
        input.move_left();
        let backend = TestBackend::new(4, 1);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|frame| {
            frame.render_widget(&input, Rect::new(0, 0, 4, 1));
        })?;

        let buffer = terminal.backend().buffer();
        assert_eq!(buffer[(2, 0)].symbol(), "c");
        assert!(buffer[(2, 0)].modifier.contains(Modifier::REVERSED));
        assert_eq!(input.cursor_position(Rect::new(0, 0, 4, 1)).x, 2);

        Ok(())
    }
}
