# Bubble Tea feel on top of ratatui: research and implementation plan

## Executive summary

Yes, this is feasible without touching ratatui's source code.

The cleanest path is a separate companion workspace that uses ratatui as the renderer and provides the pieces that create the Bubble Tea/Charm experience around it.

The primary product should be theme and components that normal ratatui apps can adopt incrementally:

1. a Charm-like theme/style layer that maps to ratatui `Style`, `Block`, `Line`, `Span`, and `Layout`,
2. styled constructors/helpers for existing ratatui widgets,
3. a small component kit inspired by Bubbles: spinner, progress, list, table, text input, viewport, help/key bindings,
4. examples and golden/snapshot tests that make the desired look and feel concrete.

An optional second layer can provide the Bubble Tea app-loop model for users who want it:

1. an Elm/Bubble Tea-style `Model` / `Msg` / `Cmd` / `Program` shell,
2. event-loop, tick, command, and task helpers,
3. adapters that work with the same theme/components.

Ratatui already exposes the right extension points: widgets, stateful widgets, `Frame::render_widget`, `Buffer`, `Style`, layout, text primitives, and terminal draw loops. It does not need a plugin system or upstream patch for this. The companion crates should stay outside ratatui and depend on ratatui's public API, preferably `ratatui-core` for reusable components and optionally `ratatui` for application/runtime conveniences.

The important design rule: users should not need to adopt a new app loop just to get the look and feel. Existing ratatui apps should be able to use the theme/components directly.

## Repositories inspected

- `/mnt/data/Projects/ratatui`
- `/mnt/data/Projects/bubbletea`
- `/mnt/data/Projects/bubbletea.rs` as an extra nearby Rust reference

`/home/akitaonrails/Projects/...` and `/mnt/data/Projects/...` appear mirrored for these projects.

## What ratatui gives us

Relevant findings:

- Ratatui is split into reusable crates. The workspace contains `ratatui`, `ratatui-core`, backend crates, widgets, macros, and examples (`/mnt/data/Projects/ratatui/Cargo.toml`).
- The project architecture explicitly supports third-party widget libraries depending on `ratatui-core` (`ARCHITECTURE.md`; `ratatui-core/src/lib.rs`; `ratatui/src/lib.rs`).
- Ratatui is immediate-mode: the whole UI is redrawn each frame, then diffed against the previous buffer (`ratatui/src/lib.rs`; `ratatui-core/src/terminal/render.rs`).
- The normal render path is `Terminal::draw(|frame| { ... })`, then `Frame::render_widget` / `Frame::render_stateful_widget` (`ratatui-core/src/terminal/frame.rs`).
- Custom widgets are stable and straightforward: implement `Widget::render(self, Rect, &mut Buffer)` or `StatefulWidget::render(..., &mut State)` (`ratatui-core/src/widgets/widget.rs`; `stateful_widget.rs`).
- Ratatui has strong primitives for the visual layer: `Style`, `Color`, `Modifier`, `Text`, `Line`, `Span`, `Layout`, `Block`, palettes, and direct buffer access for custom rendering.
- Existing examples show the patterns we want to reuse:
  - single root widget: `examples/apps/demo2/src/app.rs`,
  - app-local theme structs: `examples/apps/demo2/src/theme.rs`,
  - custom themed widget: `examples/apps/custom-widget/src/main.rs`,
  - async/tick event loops: `examples/apps/async-github/src/main.rs`.

Important constraints:

- Ratatui is not retained-mode. Widgets should be cheap render descriptions over state owned by the app/component.
- Ratatui should keep owning drawing and buffer diffing. We should not emit ANSI strings directly from the companion crate as the main render path.
- Backend escape hatches exist, but using them would make terminal state harder to keep correct. Avoid unless a feature cannot be represented in ratatui.
- `WidgetRef` / `StatefulWidgetRef` are useful for heterogeneous widget containers but currently unstable. The first version should not require them.

## What Bubble Tea actually contributes

Bubble Tea core is less about visual widgets and more about the application architecture:

- `Model` has `Init`, `Update`, and `View` (`bubbletea/tea.go`).
- all state transitions happen via messages (`Msg`),
- side effects are commands (`Cmd`) that eventually produce messages,
- commands can be batched or sequenced,
- timers are one-shot messages via `Tick` / `Every`,
- the `Program` owns terminal lifecycle, event translation, update loop, command execution, rendering, and shutdown.

The recognizable Charm look mostly comes from the ecosystem around Bubble Tea:

- Bubbles provides components such as spinner, progress, list, table, help, text input, and key bindings.
- Lip Gloss provides style composition, colors, borders, spacing, alignment, and the default polished terminal aesthetic.
- Bubble Tea examples combine those libraries with conventions like compact help footers, `j/k` navigation, muted gray hints, accent colors, rounded borders, bullets, checkmarks, spinners, progress animations, and predictable `q`/`esc`/`ctrl+c` exits.

For ratatui, that means the companion crates should not try to clone Bubble Tea's string renderer. They should port the component behavior and aesthetic defaults into ratatui-native widgets and styles, with the Bubble Tea app architecture available only as an optional layer.

## Existing `bubbletea.rs` reference

The local `/mnt/data/Projects/bubbletea.rs` repo is a direct Rust port of Bubble Tea concepts.

Useful ideas:

- `Model`, dynamic `Msg`, `Cmd`, `batch`, `sequence`, `tick`, and `every` semantics.
- `Program` lifecycle concepts: send, quit, kill, wait, filters, panic cleanup.
- Explicit terminal/input message taxonomy.
- Good practice of documenting parity gaps and testing terminal output.

Not suitable as the primary base for this project:

- Its `View` is string/ANSI content plus terminal flags.
- Its renderer emits ANSI/OSC directly.
- It owns low-level terminal handling via `libc` and `windows-sys`.
- It does not integrate with ratatui.

Conclusion: use it as conceptual prior art for the runtime, not as the rendering architecture.

## Design options considered

### Option A: Theme and widgets/components first

Build a ratatui component/theme crate with Bubble Tea/Bubbles/Lip Gloss-inspired components, but no app-loop runtime requirement.

Pros:

- smallest scope,
- easiest to maintain,
- purely ratatui-native,
- no terminal lifecycle conflicts.

Cons:

- does not provide Bubble Tea's `Msg`, `Cmd`, composable update loops, commands, ticks, and subscriptions by itself,
- every app can keep writing its own ratatui loop.

This should be the primary v1 direction because it is the easiest for existing ratatui apps to adopt.

### Option B: Optional ratatui-native TEA app shell

Build an optional crate that wraps a ratatui draw loop with Bubble Tea-style `Model` / `Msg` / `Cmd`, while reusing the same theme/components from Option A.

Pros:

- captures the Bubble Tea programming model for users who want it,
- keeps ratatui as the renderer,
- no upstream ratatui changes,
- maintainable separation between runtime, styles, and components,
- works naturally with ratatui immediate-mode rendering.

Cons:

- larger scope than theme/components,
- needs careful async/task design,
- must avoid over-abstracting ratatui.

Recommended as an optional later layer, not as a prerequisite for the theme/components.

### Option C: Adapt or fork `bubbletea.rs`

Add a ratatui renderer to the existing direct port.

Pros:

- reuses prior work on commands and program lifecycle.

Cons:

- current rendering abstraction is string/ANSI-first,
- terminal ownership overlaps with ratatui/crossterm,
- 1:1 Go parity goals may conflict with Rust and ratatui ergonomics,
- higher risk of carrying unrelated complexity.

Not recommended as the primary implementation path.

### Option D: ratatui plugin/extension upstream

Modify ratatui or propose a plugin layer.

Pros:

- could make some APIs official if they were missing.

Cons:

- unnecessary: ratatui already exposes public widget/render/event-loop extension points,
- slower and harder to maintain,
- violates the goal of not touching ratatui source.

Not recommended.

## Recommended architecture

Start as a new standalone Rust workspace in this repo.

Suggested crates/modules:

```text
ratatui-bubbletea/
  crates/
    ratatui-bubbletea-theme/       # Charm/Lip Gloss-inspired theme tokens and widget helpers
    ratatui-bubbletea-components/  # Bubbles-inspired ratatui widgets/components
    ratatui-tea/                   # optional Model/Program/Msg/Cmd app-loop shell
  examples/
    themed-widgets/
    spinner/
    list/
    table/
    textinput/
    dashboard/
    tea-counter/
  docs/
```

The exact names can change, but the layering should stay clear:

1. theme helpers are the base,
2. components build on the theme and ratatui,
3. `ratatui-tea` is optional and depends on neither app code nor component internals.

Existing ratatui apps should be able to depend only on the theme/components crates and keep their current event loops.

### Dependency shape and duplication control

The optional `ratatui-tea` layer can stay clean if dependencies flow in one direction only:

```text
ratatui / ratatui-core
        ↑
ratatui-bubbletea-theme
        ↑
ratatui-bubbletea-components
        ↑
ratatui-tea  optional app-loop shell
```

Rules:

- `theme` owns visual tokens, style presets, border presets, text helpers, and common widget styling helpers.
- `components` owns reusable UI state and ratatui widget implementations: spinner, help, list, progress, text input, viewport, etc.
- `ratatui-tea` owns only app-loop concerns: `Model`, `Msg`, `Cmd`, `Program`, event translation, ticks, command execution, terminal setup/restore.
- `ratatui-tea` may depend on `theme` and `components` for examples/adapters, but theme/components must not depend on `ratatui-tea`.
- Shared concepts such as key bindings should live in `components`, not in `ratatui-tea`, because plain ratatui apps need them too.
- Shared timers/animation state should live with the component when it is visual state, and in `ratatui-tea` only when it is command/event-loop scheduling.

This avoids duplication: a spinner is implemented once as a ratatui component. Plain ratatui users call `spinner.tick()` from their own loop; `ratatui-tea` users receive a tick message and call the same component method from `update()`.

### Theme layer

Provide a default Charm-inspired theme that maps to ratatui types:

```rust
pub struct Theme {
    pub accent: Style,
    pub muted: Style,
    pub success: Style,
    pub warning: Style,
    pub error: Style,
    pub border: Style,
    pub title: Style,
    pub help_key: Style,
    pub help_desc: Style,
}
```

Use ratatui-native `Style`, `Color`, `Modifier`, `Line`, and `Span`. Do not port Lip Gloss wholesale. We only need the useful concepts:

- style tokens,
- border presets,
- spacing helpers,
- alignment helpers,
- text decorators,
- consistent component defaults.

The ergonomic goal is easy adoption in normal ratatui code:

```rust
let theme = BubbleTheme::default();

frame.render_widget(
    theme.paragraph("Hello")
        .title("Demo")
        .bordered(),
    area,
);
```

or, where direct wrappers are cleaner:

```rust
frame.render_widget(
    BubbleParagraph::new("Hello")
        .theme(&theme)
        .title("Demo"),
    area,
);
```

The first version should wrap/style existing ratatui widgets before inventing new widgets.

### Component layer

Components should be ratatui-native. They can expose state/update helpers, but rendering should use stable `Widget` / `StatefulWidget` APIs.

Possible component shape:

```rust
pub struct Spinner {
    state: SpinnerState,
    style: SpinnerStyle,
}

impl Spinner {
    pub fn tick(&mut self);
}

impl Widget for &Spinner {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // render with ratatui primitives
    }
}
```

First component candidates:

1. key bindings + help footer,
2. spinner,
3. progress bar,
4. list/select menu,
5. table styling helpers,
6. text input,
7. viewport/pager.

The components should work in plain ratatui loops:

```rust
loop {
    terminal.draw(|frame| {
        frame.render_widget(&app.spinner, area);
    })?;

    if tick_due {
        app.spinner.tick();
    }
}
```

Avoid requiring unstable `WidgetRef` in v0.

### Optional `ratatui-tea` app-loop layer

This layer is for users who want Bubble Tea's architecture, not for users who only want the visual style. It should be optional and should reuse the same theme/components.

Core API sketch:

```rust
pub trait Model: Sized + 'static {
    type Msg: Send + 'static;

    fn init(&mut self) -> Cmd<Self::Msg> {
        Cmd::none()
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg>;

    fn view(&self, frame: &mut ratatui::Frame<'_>);
}
```

Runtime responsibilities:

- initialize/restore terminal,
- translate crossterm events into messages,
- handle resize, tick, focus, paste, mouse, and keyboard messages,
- run commands and send their returned messages back into the loop,
- support `batch`, `sequence`, `tick`, `every`, and external `ProgramHandle::send`,
- call `terminal.draw(|frame| model.view(frame))`,
- provide clean quit/error/panic cleanup behavior.

Keep this layer small. It should not know about spinner/list/table internals.

### Terminal mode declarations

Bubble Tea v2 moves terminal modes into the declarative `View`. Ratatui uses different rendering primitives, so we can model this as separate program state instead of bundling it into widget output.

Possible API:

```rust
pub trait TerminalPolicy {
    fn terminal_options(&self) -> TerminalOptions;
}

pub struct TerminalOptions {
    pub alternate_screen: bool,
    pub mouse: MouseMode,
    pub focus_reporting: bool,
    pub paste_mode: bool,
    pub cursor_visible: bool,
    pub title: Option<String>,
}
```

Apply changes at the program boundary, not during widget rendering. That keeps widgets pure and avoids backend state surprises.

### Message design

Prefer strongly typed app messages by default:

```rust
enum Msg {
    Key(KeyEvent),
    Resize { width: u16, height: u16 },
    Tick,
    Loaded(Result<Data, Error>),
}
```

Provide an optional dynamic message wrapper later if the ergonomics require it. Rust users generally expect typed enums; starting dynamic would make simple apps harder to reason about.

### Command design

Initial command API:

```rust
pub struct Cmd<M>(/* boxed future/task or sync function */);

impl<M> Cmd<M> {
    pub fn none() -> Self;
    pub fn once(f: impl FnOnce() -> M + Send + 'static) -> Self;
    pub fn batch(cmds: impl IntoIterator<Item = Cmd<M>>) -> Self;
    pub fn sequence(cmds: impl IntoIterator<Item = Cmd<M>>) -> Self;
    pub fn tick(duration: Duration, f: impl FnOnce(Instant) -> M + Send + 'static) -> Self;
}
```

Executor strategy:

- MVP: thread/channel-based executor, no mandatory async runtime.
- Optional feature: `tokio` executor and stream/subscription support.
- All commands return messages to the same central queue.

## What to avoid

- Do not modify ratatui source.
- Do not bypass ratatui with ANSI string rendering for normal widgets.
- Do not require `ratatui-tea` to use the theme/components.
- Do not make the optional app-loop layer depend on component internals.
- Do not make all messages dynamic in the first version.
- Do not require a specific async runtime in the optional app-loop crate.
- Do not depend on unstable ratatui features for the MVP.
- Do not chase byte-for-byte Bubble Tea parity; chase the programming model and feel.

## Implementation phases

### Phase 0: repository skeleton

- Create a Rust workspace.
- Add `ratatui-bubbletea-theme` crate.
- Add `ratatui-bubbletea-components` crate if component code is ready to split; otherwise start with a `components` module and split later.
- Add basic CI commands: `cargo fmt`, `cargo clippy`, `cargo test`.
- Add examples directory.

Deliverable: empty but buildable workspace.

### Phase 1: theme primitives and styled ratatui helpers

- Define `BubbleTheme` / `Theme` with Charm-inspired default tokens.
- Add helpers for text, muted text, accent text, titles, borders, selected/focused state, help text, bullets, and status colors.
- Add styled constructors or extension traits for common ratatui widgets: `Block`, `Paragraph`, `List`, `Table`, `Gauge`/progress-like widgets.
- Add a `themed-widgets` example that uses a normal ratatui event loop.

Deliverable: existing ratatui apps can opt into the look by using theme helpers, without changing their architecture.

### Phase 2: first Bubbles-inspired components

- Key bindings/help footer.
- Spinner.
- Progress bar.
- Selectable list.
- Each component should expose plain state/update methods and render through ratatui `Widget` / `StatefulWidget`.

Deliverable: component examples comparable to common Bubble Tea/Bubbles demos, still runnable from plain ratatui loops.

### Phase 3: richer components and ergonomics

- Table styling helpers or a table wrapper.
- Text input.
- Viewport/pager.
- Optional mouse-friendly component helpers.
- Builder APIs and extension traits based on what felt best in examples.

Deliverable: enough themed primitives to build real TUIs with a Charm-like feel.

### Phase 4: optional `ratatui-tea` MVP

- Add separate `ratatui-tea` crate.
- Define `Model`, `Cmd`, `Program`, and `ProgramHandle`.
- Integrate ratatui terminal init/draw/restore.
- Map crossterm keyboard and resize events into messages.
- Implement quit handling.
- Add `tea-counter` example using the same theme/components.

Deliverable: users who want Bubble Tea's app architecture can opt in, but theme/component users are unaffected.

### Phase 5: optional commands, ticks, and subscriptions

- Implement `Cmd::none`, `Cmd::once`, `Cmd::batch`, `Cmd::sequence`, `Cmd::tick`.
- Add message channel and external `send` handle.
- Add spinner/timer example using `ratatui-tea`.
- Consider optional `tokio` feature for async commands/streams.

Deliverable: animations and async-ish effects in the optional app-loop model.

### Phase 6: testing and polish

- Snapshot tests for widget buffers.
- Integration tests for component state transitions.
- Integration tests for optional runtime state transitions when `ratatui-tea` exists.
- Panic/restore tests where practical.
- Docs showing both adoption paths:
  - plain ratatui + theme/components,
  - optional `ratatui-tea` app shell.

Deliverable: maintainable, documented crate ready for broader use.

## First implementation target

The first code we should write should prove that an existing ratatui app can adopt the style without changing its event loop:

```rust
let theme = BubbleTheme::default();

terminal.draw(|frame| {
    let area = frame.area();

    frame.render_widget(
        theme
            .block()
            .title("ratatui-bubbletea")
            .rounded()
            .focused(true),
        area,
    );

    frame.render_widget(
        theme.paragraph("A normal ratatui app with Charm-like defaults"),
        area.inner(Margin::new(2, 1)),
    );
})?;
```

If this feels ergonomic, expand to spinner/help/list components. Only after the theme/components feel right should we add the optional `ratatui-tea` loop.

## Final recommendation

Create a ratatui-native companion library, not a ratatui fork and not a direct `bubbletea.rs` renderer port.

The maintainable split is:

- theme: Charm/Lip Gloss-inspired styling mapped to ratatui primitives,
- components: Bubbles-inspired widgets implemented as ratatui widgets/stateful widgets,
- optional app shell: Bubble Tea-style program loop over ratatui for users who want `Model` / `Msg` / `Cmd`.

This achieves the desired look and feel while preserving ratatui's strengths and keeping the integration clean.
