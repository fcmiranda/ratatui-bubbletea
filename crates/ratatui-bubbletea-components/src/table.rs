use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::{HighlightSpacing, Row, StatefulWidget, Table, TableState, Widget};
use ratatui_bubbletea_theme::BubbleTheme;

/// A thin Charm-like styling wrapper around ratatui's table widget.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ThemedTable<'a> {
    rows: Vec<Row<'a>>,
    widths: Vec<Constraint>,
    header: Option<Row<'a>>,
    title: Option<String>,
    selected: Option<usize>,
    column_spacing: u16,
    theme: BubbleTheme,
}

impl<'a> ThemedTable<'a> {
    /// Creates a table from rows and column widths.
    #[must_use]
    pub fn new<R, C>(rows: R, widths: C) -> Self
    where
        R: IntoIterator,
        R::Item: Into<Row<'a>>,
        C: IntoIterator,
        C::Item: Into<Constraint>,
    {
        Self {
            rows: rows.into_iter().map(Into::into).collect(),
            widths: widths.into_iter().map(Into::into).collect(),
            header: None,
            title: None,
            selected: None,
            column_spacing: 1,
            theme: BubbleTheme::default(),
        }
    }

    /// Sets the header row.
    #[must_use]
    pub fn header(mut self, header: Row<'a>) -> Self {
        self.header = Some(header.style(self.theme.title));
        self
    }

    /// Sets a themed block title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the selected row.
    #[must_use]
    pub const fn selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    /// Sets column spacing.
    #[must_use]
    pub const fn column_spacing(mut self, spacing: u16) -> Self {
        self.column_spacing = spacing;
        self
    }

    /// Sets the theme.
    #[must_use]
    pub const fn theme(mut self, theme: BubbleTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Returns the selected row.
    #[must_use]
    pub const fn selected_row(&self) -> Option<usize> {
        self.selected
    }

    /// Returns row count.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    fn table(&self) -> Table<'a> {
        let rows = self
            .rows
            .iter()
            .cloned()
            .map(|row| row.style(self.theme.text));
        let mut table = Table::new(rows, self.widths.clone())
            .style(self.theme.text)
            .row_highlight_style(self.theme.selected)
            .highlight_symbol(self.theme.symbols.selected)
            .highlight_spacing(HighlightSpacing::Always)
            .column_spacing(self.column_spacing);

        if let Some(header) = self.header.clone() {
            table = table.header(header.style(self.theme.title));
        }

        if let Some(title) = &self.title {
            table = table.block(self.theme.block().title(title.clone()));
        }

        table
    }
}

impl Widget for &ThemedTable<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() {
            return;
        }

        let mut state = TableState::default().with_selected(self.selected);
        StatefulWidget::render(self.table(), area, buf, &mut state);
    }
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::layout::{Constraint, Rect};
    use ratatui::widgets::Row;

    use super::ThemedTable;

    #[test]
    fn table_tracks_selected_row_and_count() {
        let table = ThemedTable::new(
            [Row::new(["Alpha"]), Row::new(["Beta"])],
            [Constraint::Length(8)],
        )
        .selected(Some(1));

        assert_eq!(table.row_count(), 2);
        assert_eq!(table.selected_row(), Some(1));
    }

    #[test]
    fn table_renders_title_header_and_selected_row() -> Result<(), Box<dyn std::error::Error>> {
        let table = ThemedTable::new(
            [Row::new(["Alpha", "ready"]), Row::new(["Beta", "busy"])],
            [Constraint::Length(8), Constraint::Length(8)],
        )
        .title("Jobs")
        .header(Row::new(["Name", "State"]))
        .selected(Some(1));
        let backend = TestBackend::new(24, 5);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|frame| {
            frame.render_widget(&table, Rect::new(0, 0, 24, 5));
        })?;

        let buffer = terminal.backend().buffer();
        assert_eq!(buffer[(1, 0)].symbol(), "J");
        assert_eq!(buffer[(2, 1)].symbol(), "N");
        assert_eq!(buffer[(1, 3)].symbol(), "▸");
        assert_eq!(buffer[(2, 3)].symbol(), "B");
        assert_eq!(buffer[(2, 3)].bg, table.theme.palette.selected_background);

        Ok(())
    }
}
