# ratatui-bubbletea

Charm/Bubble Tea-inspired themes and components for ratatui.

This project does **not** replace ratatui's renderer. Existing ratatui apps can adopt the look incrementally by using theme helpers and ratatui-native components. The optional `ratatui-tea` crate is only for users who also want Bubble Tea-style app-loop ergonomics.

## Crates

- `ratatui-bubbletea-theme`: semantic theme tokens and styled helpers for existing ratatui widgets.
- `ratatui-bubbletea-components`: Bubbles-inspired components implemented as ratatui widgets/stateful widgets.
- `ratatui-tea`: optional app-loop shell for `Model` / `Msg` / `Cmd` style apps.

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

See [`docs/milestones.md`](docs/milestones.md) and [`docs/status.md`](docs/status.md) for the implementation plan and current status.
