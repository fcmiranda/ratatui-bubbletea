# ratatui-bubbletea

Charm/Bubble Tea-inspired themes and components for ratatui.

This project does **not** replace ratatui's renderer. Existing ratatui apps can adopt the look incrementally by using theme helpers and ratatui-native components. The optional `ratatui-tea` crate is only for users who also want Bubble Tea-style app-loop ergonomics.

## Crates

- `ratatui-bubbletea-theme`: semantic theme tokens and styled helpers for existing ratatui widgets.
- `ratatui-bubbletea-components`: Bubbles-inspired components implemented as ratatui widgets/stateful widgets.
- `ratatui-tea`: optional app-loop shell for `Model` / `Msg` / `Cmd` style apps.

## Usage documentation

- [Use in an existing ratatui app](docs/usage-existing-ratatui.md): keep your current event loop and adopt the theme/components incrementally.
- [Start a new app with optional `ratatui-tea`](docs/usage-new-tea-app.md): organize a new app around `Model` / `Msg` / `Cmd` while still rendering with ratatui.
- [Implementation status](docs/status.md): what is implemented and what follow-ups remain.
- [Milestone plan](docs/milestones.md): milestone-by-milestone development checklist.

## Plain ratatui usage

```rust
use ratatui_bubbletea_theme::BubbleTheme;

let theme = BubbleTheme::default();

terminal.draw(|frame| {
    frame.render_widget(
        theme.block_with_focus(true).title("Demo"),
        frame.area(),
    );
})?;
```

Components work the same way:

```rust
use ratatui_bubbletea_components::{Spinner, SpinnerFrames};

let mut spinner = Spinner::new()
    .frames(SpinnerFrames::DOTS)
    .label("Loading");

spinner.tick();

terminal.draw(|frame| {
    frame.render_widget(&spinner, frame.area());
})?;
```

## Optional `ratatui-tea` usage

```rust
use ratatui_tea::{Cmd, Model, Program};

struct App { count: i32 }
enum Msg { Increment }

impl Model for App {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::Increment => self.count += 1,
        }
        Cmd::none()
    }

    fn view(&self, frame: &mut ratatui::Frame<'_>) {
        // normal ratatui rendering
    }
}

let mut program = Program::new(App { count: 0 });
program.send(Msg::Increment);
```

For more detail, start with [the existing-app guide](docs/usage-existing-ratatui.md) or [the new `ratatui-tea` app guide](docs/usage-new-tea-app.md).
