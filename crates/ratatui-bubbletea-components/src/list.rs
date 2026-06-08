use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui_bubbletea_theme::BubbleTheme;

/// One item in a [`SelectList`].
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ListItem {
    label: String,
    description: Option<String>,
}

impl ListItem {
    /// Creates an item with only a label.
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: None,
        }
    }

    /// Adds secondary description text.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Returns the label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description_text(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl From<&str> for ListItem {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ListItem {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Selection state for a [`SelectList`].
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ListState {
    selected: Option<usize>,
}

impl ListState {
    /// Creates an empty selection state.
    #[must_use]
    pub const fn new() -> Self {
        Self { selected: None }
    }

    /// Returns the selected index.
    #[must_use]
    pub const fn selected(self) -> Option<usize> {
        self.selected
    }

    /// Selects a specific item, clamped to the available item count.
    pub fn select(&mut self, index: Option<usize>, len: usize) {
        self.selected = index.and_then(|index| (index < len).then_some(index));
    }

    /// Selects the first item if the list is not empty.
    pub fn first(&mut self, len: usize) {
        self.selected = (len > 0).then_some(0);
    }

    /// Selects the last item if the list is not empty.
    pub fn last(&mut self, len: usize) {
        self.selected = len.checked_sub(1);
    }

    /// Moves selection down by one item, clamped at the end.
    pub fn next(&mut self, len: usize) {
        match (self.selected, len) {
            (_, 0) => self.selected = None,
            (None, _) => self.selected = Some(0),
            (Some(index), _) => self.selected = Some(index.saturating_add(1).min(len - 1)),
        }
    }

    /// Moves selection up by one item, clamped at the beginning.
    pub fn previous(&mut self, len: usize) {
        match (self.selected, len) {
            (_, 0) => self.selected = None,
            (None, _) => self.selected = Some(0),
            (Some(index), _) => self.selected = Some(index.saturating_sub(1)),
        }
    }
}

/// A themed selectable list widget.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SelectList {
    items: Vec<ListItem>,
    state: ListState,
    theme: BubbleTheme,
}

impl SelectList {
    /// Creates a selectable list.
    #[must_use]
    pub fn new<I, T>(items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<ListItem>,
    {
        let items = items.into_iter().map(Into::into).collect::<Vec<_>>();
        let mut state = ListState::new();
        state.first(items.len());

        Self {
            items,
            state,
            theme: BubbleTheme::default(),
        }
    }

    /// Sets the theme.
    #[must_use]
    pub const fn theme(mut self, theme: BubbleTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Returns all items.
    #[must_use]
    pub fn items(&self) -> &[ListItem] {
        &self.items
    }

    /// Returns current state.
    #[must_use]
    pub const fn state(&self) -> ListState {
        self.state
    }

    /// Returns the selected index.
    #[must_use]
    pub const fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    /// Returns the selected item.
    #[must_use]
    pub fn selected_item(&self) -> Option<&ListItem> {
        self.selected().and_then(|index| self.items.get(index))
    }

    /// Selects a specific item.
    pub fn select(&mut self, index: Option<usize>) {
        self.state.select(index, self.items.len());
    }

    /// Selects the first item.
    pub fn first(&mut self) {
        self.state.first(self.items.len());
    }

    /// Selects the last item.
    pub fn last(&mut self) {
        self.state.last(self.items.len());
    }

    /// Moves selection down by one item.
    pub fn next(&mut self) {
        self.state.next(self.items.len());
    }

    /// Moves selection up by one item.
    pub fn previous(&mut self) {
        self.state.previous(self.items.len());
    }

    fn item_line<'a>(&'a self, item: &'a ListItem, selected: bool) -> Line<'a> {
        let marker = if selected {
            self.theme.symbols.selected
        } else {
            self.theme.symbols.bullet
        };

        let marker = if selected {
            self.theme.accent(marker)
        } else {
            self.theme.muted(marker)
        };
        let label = if selected {
            ratatui::text::Span::styled(item.label.as_str(), self.theme.selected)
        } else {
            self.theme.span(item.label.as_str())
        };

        let mut spans = vec![marker, self.theme.span(" "), label];

        if let Some(description) = &item.description {
            spans.push(self.theme.muted(" — "));
            spans.push(self.theme.muted(description.as_str()));
        }

        Line::from(spans)
    }
}

impl Widget for &SelectList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() {
            return;
        }

        for (offset, item) in (0..area.height).zip(self.items.iter()) {
            let index = usize::from(offset);
            let selected = self.selected() == Some(index);
            let line = self.item_line(item, selected);
            buf.set_line(area.x, area.y + offset, &line, area.width);
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::layout::Rect;

    use super::{ListItem, ListState, SelectList};

    #[test]
    fn state_navigation_is_clamped() {
        let mut state = ListState::new();

        state.next(3);
        assert_eq!(state.selected(), Some(0));
        state.next(3);
        assert_eq!(state.selected(), Some(1));
        state.last(3);
        assert_eq!(state.selected(), Some(2));
        state.next(3);
        assert_eq!(state.selected(), Some(2));
        state.previous(3);
        assert_eq!(state.selected(), Some(1));
        state.first(3);
        assert_eq!(state.selected(), Some(0));
        state.previous(3);
        assert_eq!(state.selected(), Some(0));
        state.select(Some(99), 3);
        assert_eq!(state.selected(), None);
    }

    #[test]
    fn empty_state_navigation_stays_unselected() {
        let mut state = ListState::new();

        state.next(0);
        assert_eq!(state.selected(), None);
        state.previous(0);
        assert_eq!(state.selected(), None);
        state.first(0);
        assert_eq!(state.selected(), None);
        state.last(0);
        assert_eq!(state.selected(), None);
    }

    #[test]
    fn list_selects_first_item_by_default() {
        let list = SelectList::new(["Alpha", "Beta"]);

        assert_eq!(list.selected(), Some(0));
        assert_eq!(list.selected_item().map(ListItem::label), Some("Alpha"));
    }

    #[test]
    fn list_navigation_updates_selected_item() {
        let mut list = SelectList::new(["Alpha", "Beta", "Gamma"]);

        list.next();
        assert_eq!(list.selected_item().map(ListItem::label), Some("Beta"));
        list.last();
        assert_eq!(list.selected_item().map(ListItem::label), Some("Gamma"));
        list.previous();
        assert_eq!(list.selected_item().map(ListItem::label), Some("Beta"));
    }

    #[test]
    fn list_renders_selected_and_unselected_items() -> Result<(), Box<dyn std::error::Error>> {
        let mut list = SelectList::new([
            ListItem::new("Alpha").description("first"),
            ListItem::new("Beta").description("second"),
        ]);
        list.next();

        let backend = TestBackend::new(18, 2);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|frame| {
            frame.render_widget(&list, Rect::new(0, 0, 18, 2));
        })?;

        let buffer = terminal.backend().buffer();
        let first = (0..15).map(|x| buffer[(x, 0)].symbol()).collect::<String>();
        let second = (0..15).map(|x| buffer[(x, 1)].symbol()).collect::<String>();

        assert_eq!(first, "• Alpha — first");
        assert_eq!(second, "▸ Beta — second");
        assert_eq!(buffer[(0, 0)].fg, list.theme.palette.muted);
        assert_eq!(buffer[(0, 1)].fg, list.theme.palette.accent);
        assert_eq!(buffer[(2, 1)].bg, list.theme.palette.selected_background);

        Ok(())
    }
}
