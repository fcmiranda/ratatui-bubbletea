//! Interactive showcase for all current `ratatui-bubbletea-components` widgets.

use std::io;
use std::time::{Duration, Instant};

use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Row};
use ratatui::{Frame, Terminal};
use ratatui_bubbletea_components::{
    Help, KeyBinding, KeyMap, ListItem, Progress, SelectList, Spinner, SpinnerFrames, TextInput,
    TextInputState, ThemedTable, Viewport,
};
use ratatui_bubbletea_theme::{BubbleTheme, Palette};

const TICK_RATE: Duration = Duration::from_millis(100);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = setup_terminal()?;
    let mut app = Showcase::new();

    let result = run(&mut terminal, &mut app);
    restore_terminal(terminal)?;
    result
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut Showcase,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|frame| app.render(frame))?;

        if !event::poll(TICK_RATE)? {
            app.tick();
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };

        if key.kind == KeyEventKind::Press && !app.handle_key(key) {
            break;
        }
    }

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

#[derive(Debug, Clone)]
struct Showcase {
    theme: BubbleTheme,
    keymap: KeyMap,
    spinner: Spinner,
    line_spinner: bool,
    progress: u16,
    list: SelectList,
    input: TextInput,
    viewport: Viewport,
    expanded_help: bool,
    status: String,
    viewport_height: usize,
    last_tick: Instant,
}

impl Showcase {
    fn new() -> Self {
        let theme = BubbleTheme::default();
        let keymap = KeyMap::new([
            KeyBinding::with_keys(["q", "esc", "ctrl+c"], "quit"),
            KeyBinding::with_keys(["j", "down"], "list down"),
            KeyBinding::with_keys(["k", "up"], "list up"),
            KeyBinding::with_keys(["pgdn", "space"], "viewport down"),
            KeyBinding::new("pgup", "viewport up"),
            KeyBinding::with_keys(["+", "="], "progress up"),
            KeyBinding::new("-", "progress down"),
            KeyBinding::new("s", "spin once"),
            KeyBinding::new("t", "toggle spinner"),
            KeyBinding::new("h", "toggle help"),
        ]);
        let list = SelectList::new([
            ListItem::new("Theme").description("Palette::CHARM + helpers"),
            ListItem::new("Spinner").description("animated frame state"),
            ListItem::new("Progress").description("themed bar"),
            ListItem::new("List").description("selection state"),
            ListItem::new("Table").description("ratatui table wrapper"),
            ListItem::new("TextInput").description("unicode-safe cursor"),
            ListItem::new("Viewport").description("scrollable content"),
            ListItem::new("Help").description("compact/expanded keymap"),
        ]);
        let input = TextInput::from_state(TextInputState::with_value("edit me"))
            .placeholder("type here")
            .focused(true);
        let viewport = Viewport::new(viewport_lines());

        Self {
            theme,
            keymap,
            spinner: Spinner::new().label("Charm-style spinner"),
            line_spinner: false,
            progress: 42,
            list,
            input,
            viewport,
            expanded_help: false,
            status: "Welcome. Press h for expanded help.".to_owned(),
            viewport_height: 1,
            last_tick: Instant::now(),
        }
    }

    fn tick(&mut self) {
        self.spinner.tick();
        self.progress = self.progress.saturating_add(1) % 101;
        self.last_tick = Instant::now();
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        let label = key_label(key);
        self.update_status(&label);

        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return false,
            KeyCode::Char('q') | KeyCode::Esc => return false,
            KeyCode::Char('h') => self.expanded_help = !self.expanded_help,
            KeyCode::Char('j') | KeyCode::Down => self.list.next(),
            KeyCode::Char('k') | KeyCode::Up => self.list.previous(),
            KeyCode::PageDown | KeyCode::Char(' ') => self.viewport.page_down(self.viewport_height),
            KeyCode::PageUp => self.viewport.page_up(self.viewport_height),
            KeyCode::Char('+' | '=') => self.progress = self.progress.saturating_add(5).min(100),
            KeyCode::Char('-') => self.progress = self.progress.saturating_sub(5),
            KeyCode::Char('s') => self.spinner.tick(),
            KeyCode::Char('t') => self.toggle_spinner(),
            KeyCode::Backspace => self.input.backspace(),
            KeyCode::Delete => self.input.delete(),
            KeyCode::Left => self.input.move_left(),
            KeyCode::Right => self.input.move_right(),
            KeyCode::Char(ch) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.input.insert(ch);
            }
            _ => {}
        }

        true
    }

    fn render(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        let help_height = if self.expanded_help { 6 } else { 1 };
        let [header, body, footer] = Layout::vertical([
            Constraint::Length(4),
            Constraint::Min(12),
            Constraint::Length(help_height),
        ])
        .areas(area);
        let [left, middle, right] = Layout::horizontal([
            Constraint::Percentage(36),
            Constraint::Percentage(32),
            Constraint::Percentage(32),
        ])
        .areas(body);
        let [list_area, input_area, progress_area] = Layout::vertical([
            Constraint::Min(8),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .areas(left);
        let [spinner_area, palette_area, table_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(4),
            Constraint::Min(7),
        ])
        .areas(middle);

        self.viewport_height = usize::from(right.height.saturating_sub(2));

        self.render_header(frame, header);
        self.render_list(frame, list_area);
        self.render_input(frame, input_area);
        self.render_progress(frame, progress_area);
        self.render_spinner(frame, spinner_area);
        self.render_palette(frame, palette_area);
        self.render_table(frame, table_area);
        self.render_viewport(frame, right);
        self.render_help(frame, footer);
    }

    fn update_status(&mut self, label: &str) {
        self.status = self.keymap.find(label).map_or_else(
            || format!("key: {label}"),
            |binding| format!("{} → {}", binding.label(), binding.description()),
        );
    }

    fn toggle_spinner(&mut self) {
        self.line_spinner = !self.line_spinner;
        self.spinner = if self.line_spinner {
            Spinner::new()
                .frames(SpinnerFrames::LINE)
                .label("ASCII line spinner")
        } else {
            Spinner::new()
                .frames(SpinnerFrames::DOTS)
                .label("Charm-style spinner")
        };
    }

    fn render_header(&self, frame: &mut Frame<'_>, area: Rect) {
        let text = vec![
            Line::from(vec![
                self.theme.accent("ratatui-bubbletea showcase"),
                self.theme.muted("  all current components in one TUI"),
            ]),
            Line::from(vec![
                self.theme.muted("status: "),
                self.theme.span(self.status.as_str()),
            ]),
        ];
        frame.render_widget(
            Paragraph::new(text).block(self.theme.titled_block("Bubble Tea components")),
            area,
        );
    }

    fn render_list(&self, frame: &mut Frame<'_>, area: Rect) {
        self.render_in_block(frame, area, "SelectList", |frame, inner| {
            frame.render_widget(&self.list, inner);
        });
    }

    fn render_input(&self, frame: &mut Frame<'_>, area: Rect) {
        self.render_in_block(frame, area, "TextInput", |frame, inner| {
            frame.render_widget(&self.input, inner);
        });
    }

    fn render_progress(&self, frame: &mut Frame<'_>, area: Rect) {
        let progress = Progress::from_percent(self.progress).label("Progress");
        self.render_in_block(frame, area, "Progress", |frame, inner| {
            frame.render_widget(&progress, inner);
        });
    }

    fn render_spinner(&self, frame: &mut Frame<'_>, area: Rect) {
        self.render_in_block(frame, area, "Spinner", |frame, inner| {
            frame.render_widget(&self.spinner, inner);
        });
    }

    fn render_palette(&self, frame: &mut Frame<'_>, area: Rect) {
        let palette = Palette::CHARM;
        let line = Line::from(vec![
            Span::styled(" foreground ", self.theme.text.bg(palette.foreground)),
            Span::styled(" accent ", self.theme.text.bg(palette.accent)),
            Span::styled(" success ", self.theme.text.bg(palette.success)),
        ]);
        frame.render_widget(
            Paragraph::new(vec![
                self.theme
                    .help_line([("Palette::CHARM", "shared color tokens")]),
                line,
            ])
            .block(self.theme.titled_block("Theme")),
            area,
        );
    }

    fn render_table(&self, frame: &mut Frame<'_>, area: Rect) {
        let selected = self.list.selected().unwrap_or(0).min(2);
        let table = ThemedTable::new(
            [
                Row::new(["theme", "semantic styles"]),
                Row::new(["components", "ratatui widgets"]),
                Row::new(["tea", "optional loop"]),
            ],
            [Constraint::Length(12), Constraint::Min(12)],
        )
        .title("ThemedTable")
        .header(Row::new(["Crate", "Role"]))
        .selected(Some(selected));

        frame.render_widget(&table, area);
    }

    fn render_viewport(&self, frame: &mut Frame<'_>, area: Rect) {
        self.render_in_block(frame, area, "Viewport", |frame, inner| {
            frame.render_widget(&self.viewport, inner);
        });
    }

    fn render_help(&self, frame: &mut Frame<'_>, area: Rect) {
        let help = Help::new(self.keymap.bindings().iter().cloned())
            .mode(if self.expanded_help {
                ratatui_bubbletea_components::HelpMode::Expanded
            } else {
                ratatui_bubbletea_components::HelpMode::Compact
            })
            .theme(self.theme);

        frame.render_widget(&help, area);
    }

    fn render_in_block(
        &self,
        frame: &mut Frame<'_>,
        area: Rect,
        title: &'static str,
        render_inner: impl FnOnce(&mut Frame<'_>, Rect),
    ) {
        let block = self.theme.titled_block(title);
        let inner = block.inner(area);
        frame.render_widget(block, area);
        render_inner(frame, inner);
    }
}

fn key_label(key: KeyEvent) -> String {
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => "ctrl+c".to_owned(),
        KeyCode::Char(' ') => "space".to_owned(),
        KeyCode::Char(ch) => ch.to_string(),
        KeyCode::Esc => "esc".to_owned(),
        KeyCode::Enter => "enter".to_owned(),
        KeyCode::Up => "up".to_owned(),
        KeyCode::Down => "down".to_owned(),
        KeyCode::Left => "left".to_owned(),
        KeyCode::Right => "right".to_owned(),
        KeyCode::PageUp => "pgup".to_owned(),
        KeyCode::PageDown => "pgdn".to_owned(),
        KeyCode::Backspace => "backspace".to_owned(),
        KeyCode::Delete => "delete".to_owned(),
        _ => "unbound".to_owned(),
    }
}

fn viewport_lines() -> [&'static str; 18] {
    [
        "This viewport is a line-oriented pager component.",
        "",
        "It intentionally renders through ratatui, not ANSI strings.",
        "Use PageDown or Space to scroll down.",
        "Use PageUp to scroll back up.",
        "",
        "The left column shows SelectList, TextInput, and Progress.",
        "The middle column shows Spinner, Theme tokens, and ThemedTable.",
        "The footer is the Help component backed by a KeyMap.",
        "",
        "Typing regular characters edits the TextInput.",
        "j/k or arrow keys move the selected list item.",
        "+/- adjust progress.",
        "s advances the spinner once.",
        "t toggles spinner frame sets.",
        "h toggles compact/expanded help.",
        "",
        "q, Esc, or Ctrl+C quits.",
    ]
}
