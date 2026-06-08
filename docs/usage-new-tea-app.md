# Starting a new app with optional `ratatui-tea`

Use this path when you want a new app organized around Bubble Tea-style concepts:

```text
Msg -> update state -> Cmd -> draw with ratatui
```

`ratatui-tea` is intentionally small. It is an app-loop shell over ratatui, not a replacement renderer. Your `view` method still renders normal ratatui widgets.

## Current scope

Implemented today:

- `Model`,
- `Cmd`,
- `Program`,
- `ProgramHandle` channel helper,
- `Cmd::none`,
- `Cmd::message`,
- `Cmd::once`,
- `Cmd::batch`,
- `Cmd::sequence`,
- `Cmd::tick`,
- `Program::draw` using ratatui `Terminal::draw`.

Not implemented yet:

- full terminal lifecycle management,
- crossterm event translation,
- non-blocking async/tick executor.

For now, pair `ratatui-tea` with your own terminal setup and event loop, or use it as a clean model/update/view organizer while the app shell matures.

## 1. Add dependencies

While developing locally:

```toml
[dependencies]
ratatui = { path = "../ratatui/ratatui", default-features = false }
ratatui-bubbletea-theme = { path = "../ratatui-bubbletea/crates/ratatui-bubbletea-theme" }
ratatui-bubbletea-components = { path = "../ratatui-bubbletea/crates/ratatui-bubbletea-components" }
ratatui-tea = { path = "../ratatui-bubbletea/crates/ratatui-tea" }
```

Compatibility: these crates currently target `ratatui 0.30.x`. Keep your app's ratatui dependency on the same 0.30 line. If you use crossterm through ratatui, prefer `ratatui::crossterm` over a separate `crossterm` dependency.

## 2. Define model and messages

```rust
use ratatui_bubbletea_components::{Help, KeyBinding, Spinner, SpinnerFrames};
use ratatui_bubbletea_theme::BubbleTheme;
use ratatui_tea::{Cmd, Model};

struct App {
    theme: BubbleTheme,
    spinner: Spinner,
    help: Help,
    count: i32,
}

enum Msg {
    Increment,
    Decrement,
    Tick,
}

impl App {
    fn new() -> Self {
        Self {
            theme: BubbleTheme::default(),
            spinner: Spinner::new()
                .frames(SpinnerFrames::DOTS)
                .label("Working"),
            help: Help::new([
                KeyBinding::with_keys(["q", "esc"], "quit"),
                KeyBinding::new("+", "increment"),
                KeyBinding::new("-", "decrement"),
            ]),
            count: 0,
        }
    }
}
```

## 3. Implement `Model`

```rust
impl Model for App {
    type Msg = Msg;

    fn init(&mut self) -> Cmd<Msg> {
        Cmd::none()
    }

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
            Msg::Tick => self.spinner.tick(),
        }

        Cmd::none()
    }

    fn view(&self, frame: &mut ratatui::Frame<'_>) {
        let area = frame.area();

        frame.render_widget(
            self.theme.titled_block("Counter"),
            area,
        );

        frame.render_widget(
            self.theme.paragraph(format!("count: {}", self.count)),
            area,
        );
    }
}
```

## 4. Create and drive a program

```rust
use ratatui_tea::Program;

let mut program = Program::new(App::new());

program.init();
program.send(Msg::Increment);
program.send(Msg::Tick);
```

To draw:

```rust
program.draw(&mut terminal)?;
```

## 5. Use commands for follow-up messages

`Cmd` lets `update` produce follow-up messages.

```rust
fn update(&mut self, msg: Msg) -> Cmd<Msg> {
    match msg {
        Msg::Increment => {
            self.count += 1;
            Cmd::message(Msg::Tick)
        }
        Msg::Tick => {
            self.spinner.tick();
            Cmd::none()
        }
        Msg::Decrement => {
            self.count -= 1;
            Cmd::none()
        }
    }
}
```

Batch and sequence are available:

```rust
Cmd::batch([
    Cmd::message(Msg::Increment),
    Cmd::once(|| Msg::Tick),
])
```

One-shot tick is also available:

```rust
Cmd::tick(std::time::Duration::from_millis(100), |_| Msg::Tick)
```

Current caveat: `Cmd::tick` sleeps synchronously in the MVP executor. This is fine for tests and simple prototypes, but a later milestone should move ticks/tasks to a non-blocking executor.

## 6. Recommended new-app structure

```text
src/
  main.rs       # terminal setup and event loop
  app.rs        # App state and Model implementation
  msg.rs        # Msg enum
  ui.rs         # ratatui layout/render helpers if desired
```

The app shell should stay thin:

- terminal setup/restore,
- input polling,
- map input to `Msg`,
- `program.send(msg)`,
- `program.draw(&mut terminal)`.

Your model owns state. Components like `Spinner`, `SelectList`, `TextInput`, and `Viewport` live inside the model and update from `Msg` in `update`.

## When to use `ratatui-tea`

Use it when:

- you want a clear `Model` / `Msg` / `Cmd` architecture,
- you are starting a new app,
- you want to keep side effects and state transitions organized,
- you like Bubble Tea's mental model.

Skip it when:

- you already have a ratatui app loop you like,
- you only need the visual style/components,
- you need a full production terminal lifecycle/event system today.
