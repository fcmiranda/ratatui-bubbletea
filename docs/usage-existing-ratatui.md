# Using ratatui-bubbletea in an existing ratatui app

Use this path when you already have a ratatui app and want the Charm/Bubble Tea look without changing your app architecture.

The important point: **keep your current ratatui loop**. Add the theme and components where you already render widgets and handle events.

## 1. Add dependencies

While developing locally in this workspace:

```toml
[dependencies]
ratatui = { path = "../ratatui/ratatui", default-features = false }
ratatui-bubbletea-theme = { path = "../ratatui-bubbletea/crates/ratatui-bubbletea-theme" }
ratatui-bubbletea-components = { path = "../ratatui-bubbletea/crates/ratatui-bubbletea-components" }
```

If your app already depends on ratatui, keep your existing ratatui dependency and add only the two `ratatui-bubbletea-*` crates. Once published, replace the path dependencies with version numbers.

## Compatibility note

`ratatui-bubbletea` currently targets `ratatui 0.30.x`. Ratatui's `Style`, `Color`, `Block`, `Line`, and `Span` types must come from the same ratatui version across your app and these crates. If your app is still on `ratatui 0.29`, upgrade ratatui first.

If your app uses crossterm events/backend through ratatui, prefer ratatui's re-export:

```rust
use ratatui::crossterm;
```

instead of adding a separate `crossterm` dependency with a potentially different version.

## 2. Create a theme once

Store the theme in your app state or create it in your render function.

```rust
use ratatui_bubbletea_theme::BubbleTheme;

const ACCENT: ratatui::style::Color = ratatui_bubbletea_theme::Palette::CHARM.accent;

struct App {
    theme: BubbleTheme,
}

impl App {
    fn new() -> Self {
        Self {
            theme: BubbleTheme::default(),
        }
    }
}
```

## 3. Style existing ratatui widgets

You can keep using normal ratatui widgets. The theme just gives you ready-made style choices.

```rust
use ratatui::layout::Margin;

fn render(app: &App, frame: &mut ratatui::Frame<'_>) {
    let area = frame.area();
    let inner = area.inner(Margin::new(2, 1));

    frame.render_widget(
        app.theme.titled_block("Dashboard"),
        area,
    );

    frame.render_widget(
        app.theme.paragraph("Existing ratatui loop, Charm-like style"),
        inner,
    );
}
```

For modal overlays, use the focused/accent border preset:

```rust
frame.render_widget(app.theme.titled_modal_block("Edit account"), modal_area);
```

## 4. Add components incrementally

Components are normal ratatui widgets. Update their state from your current event/tick loop and render them with `frame.render_widget`.

### Spinner

```rust
use ratatui_bubbletea_components::{Spinner, SpinnerFrames};

struct App {
    spinner: Spinner,
}

impl App {
    fn new() -> Self {
        Self {
            spinner: Spinner::new()
                .frames(SpinnerFrames::DOTS)
                .label("Loading"),
        }
    }

    fn on_tick(&mut self) {
        self.spinner.tick();
    }

    fn render(&self, frame: &mut ratatui::Frame<'_>) {
        frame.render_widget(&self.spinner, frame.area());
    }
}
```

### Help footer

```rust
use ratatui_bubbletea_components::{Help, KeyBinding};

let help = Help::new([
    KeyBinding::with_keys(["q", "esc", "ctrl+c"], "quit"),
    KeyBinding::with_keys(["j", "down"], "down"),
    KeyBinding::with_keys(["k", "up"], "up"),
    KeyBinding::new("?", "help"),
]);

frame.render_widget(&help, footer_area);
```

### Selectable list

```rust
use ratatui_bubbletea_components::{ListItem, SelectList};

let mut list = SelectList::new([
    ListItem::new("Theme").description("done"),
    ListItem::new("Components").description("active"),
    ListItem::new("Docs").description("next"),
]);

list.next();
frame.render_widget(&list, list_area);
```

### Progress

```rust
use ratatui_bubbletea_components::Progress;

let progress = Progress::from_ratio(0.42)
    .width(24)
    .label("Downloading");

frame.render_widget(&progress, area);
```

### Text input

```rust
use ratatui_bubbletea_components::TextInput;

let mut input = TextInput::new()
    .placeholder("Search")
    .focused(true);

input.insert('r');
input.insert('s');
input.move_left();

frame.render_widget(&input, input_area);

// If your app manages terminal cursor position:
let cursor = input.cursor_position(input_area);
```

### Viewport

```rust
use ratatui_bubbletea_components::Viewport;

let mut viewport = Viewport::new([
    "one",
    "two",
    "three",
    "four",
]);

viewport.scroll_down(1, 2);
frame.render_widget(&viewport, area);
```

### Table

```rust
use ratatui::layout::Constraint;
use ratatui::widgets::Row;
use ratatui_bubbletea_components::ThemedTable;

let table = ThemedTable::new(
    [
        Row::new(["Theme", "done"]),
        Row::new(["List", "done"]),
        Row::new(["Docs", "active"]),
    ],
    [Constraint::Length(16), Constraint::Length(12)],
)
.title("Milestones")
.header(Row::new(["Component", "State"]))
.selected(Some(2));

frame.render_widget(&table, area);
```

## 5. Keep event handling in your app

This crate does not force a key/event abstraction on existing apps. For example:

```rust
match event {
    Event::Key(key) if key.code == KeyCode::Char('j') => app.list.next(),
    Event::Key(key) if key.code == KeyCode::Char('k') => app.list.previous(),
    Event::Key(key) if key.code == KeyCode::Char('?') => app.help_expanded = !app.help_expanded,
    _ => {}
}
```

## Recommended migration order

1. Add `BubbleTheme` and style your main blocks/paragraphs.
2. Add `Help` so the UI starts feeling Charm-like.
3. Replace ad-hoc loading indicators with `Spinner` or `Progress`.
4. Replace simple menus with `SelectList`.
5. Add `TextInput`, `Viewport`, and `ThemedTable` where useful.

At no point do you need to adopt `ratatui-tea` unless you want the optional app-loop model.
