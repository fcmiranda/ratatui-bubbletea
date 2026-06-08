# Milestone plan

This project should be implemented as small, working, tested milestones. Each milestone is complete only when:

- the milestone's public API is documented enough to use from examples,
- unit tests cover the important behavior and rendering expectations,
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` pass,
- the milestone is committed to git by itself.

## Architecture rule

Keep the dependency graph one-way to avoid duplication:

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

- Existing ratatui apps must be able to use `theme` and `components` without adopting `ratatui-tea`.
- Visual concepts live in `theme` or `components`, never in `ratatui-tea`.
- App-loop concepts live in `ratatui-tea`, never in `theme` or `components`.
- A component is implemented once. Plain ratatui users update it from their own loop; `ratatui-tea` users update it from `Model::update`.
- Prefer stable ratatui APIs. Do not require `unstable-widget-ref` for v0.

## Milestone 0: workspace skeleton

Goal: make the repository buildable and testable.

Deliverables:

- initialize git,
- root `Cargo.toml` workspace,
- crates:
  - `crates/ratatui-bubbletea-theme`,
  - `crates/ratatui-bubbletea-components`,
- root `README.md`,
- minimal crate docs,
- tests proving the workspace is wired correctly.

Validation:

- `cargo fmt --check`,
- `cargo clippy --workspace --all-targets -- -D warnings`,
- `cargo test --workspace`.

Commit message:

- `chore: create workspace skeleton`

## Milestone 1: theme primitives and styled ratatui helpers

Goal: an existing ratatui app can adopt a Charm-like look without changing its event loop.

Deliverables:

- `BubbleTheme` / `Theme` type,
- default palette and semantic styles:
  - text,
  - muted,
  - accent,
  - success,
  - warning,
  - error,
  - border,
  - focused border,
  - title,
  - selected,
  - help key,
  - help description,
- helpers for:
  - `Block` styling,
  - `Paragraph` styling,
  - common `Line` / `Span` styles,
  - bullets/checkmarks/crosses,
  - compact help text,
- `examples/themed-widgets` using a plain ratatui loop,
- unit tests for defaults and helper output.

Validation:

- theme unit tests,
- example compiles,
- workspace fmt/clippy/test pass.

Commit message:

- `feat: add charm-inspired theme primitives`

## Milestone 2: key bindings and help footer

Goal: reproduce the common Bubble Tea/Bubbles help footer feel.

Deliverables:

- `KeyBinding` with key aliases and human label,
- `KeyMap` / grouped bindings,
- `Help` widget with compact and expanded modes,
- theme integration for muted descriptions and highlighted keys,
- examples showing `q/esc/ctrl+c`, `j/k`, arrows, and `?` help expansion,
- unit tests for key matching and rendered help text.

Validation:

- key matching tests,
- buffer/render snapshot-style tests,
- workspace fmt/clippy/test pass.

Commit message:

- `feat: add key bindings and help footer`

## Milestone 3: spinner component

Goal: add a Bubbles-like spinner that works in plain ratatui loops.

Deliverables:

- spinner frame sets,
- `SpinnerState` / `Spinner`,
- `tick()` state update,
- configurable label/style,
- `Widget` implementation,
- `examples/spinner`,
- unit tests for frame cycling and rendering.

Validation:

- spinner state tests,
- buffer/render tests,
- workspace fmt/clippy/test pass.

Commit message:

- `feat: add spinner component`

## Milestone 4: progress component

Goal: add a styled progress bar with Charm-like defaults.

Deliverables:

- `Progress` / `ProgressState`,
- percentage and ratio constructors,
- configurable width, label, filled/empty symbols,
- theme integration,
- `examples/progress`,
- unit tests for clamping, labels, and rendering.

Validation:

- progress logic tests,
- buffer/render tests,
- workspace fmt/clippy/test pass.

Commit message:

- `feat: add progress component`

## Milestone 5: selectable list component

Goal: add a polished list/select menu abstraction.

Deliverables:

- `ListState` with selected index,
- navigation helpers: next, previous, first, last,
- optional filtering-friendly item model,
- styled selected item indicator,
- optional help integration,
- `examples/list`,
- unit tests for navigation boundaries and rendering.

Validation:

- state transition tests,
- buffer/render tests,
- workspace fmt/clippy/test pass.

Commit message:

- `feat: add selectable list component`

## Milestone 6: table styling helpers

Goal: make ratatui tables look consistent with the theme.

Deliverables:

- table style helpers or lightweight wrapper,
- header/row/selected row styles,
- border/title helpers,
- `examples/table`,
- unit tests for style construction and selected-row rendering.

Validation:

- style helper tests,
- buffer/render tests where practical,
- workspace fmt/clippy/test pass.

Commit message:

- `feat: add themed table helpers`

## Milestone 7: text input component

Goal: add a simple themed text input suitable for forms/search bars.

Deliverables:

- `TextInputState`,
- insertion/deletion/cursor movement,
- placeholder support,
- focused/unfocused styles,
- optional cursor position helper for apps,
- `examples/textinput`,
- unit tests for editing behavior and rendering.

Validation:

- editing tests,
- buffer/render tests,
- workspace fmt/clippy/test pass.

Commit message:

- `feat: add text input component`

## Milestone 8: viewport/pager component

Goal: add a scrollable viewport for long content.

Deliverables:

- `ViewportState`,
- vertical scrolling helpers,
- line wrapping policy if feasible,
- themed scrollbar/position indicator if feasible,
- `examples/viewport`,
- unit tests for scrolling bounds and rendering.

Validation:

- scroll state tests,
- buffer/render tests,
- workspace fmt/clippy/test pass.

Commit message:

- `feat: add viewport component`

## Milestone 9: optional `ratatui-tea` MVP

Goal: provide Bubble Tea-style app-loop ergonomics without making it required.

Deliverables:

- new `crates/ratatui-tea`,
- `Model`, `Cmd`, `Program`, `ProgramHandle`,
- keyboard/resize event translation,
- quit handling,
- terminal setup/restore,
- `examples/tea-counter` using the existing theme/components,
- unit tests for command and update-loop behavior where possible.

Validation:

- app-loop tests,
- example compiles,
- workspace fmt/clippy/test pass.

Commit message:

- `feat: add optional tea app loop`

## Milestone 10: optional commands, ticks, and async hooks

Goal: round out the optional app-loop model.

Deliverables:

- `Cmd::none`, `once`, `batch`, `sequence`, `tick`,
- external `ProgramHandle::send`,
- optional `tokio` feature if needed,
- timer/spinner example using `ratatui-tea`,
- unit tests for command scheduling semantics.

Validation:

- command tests,
- example compiles,
- workspace fmt/clippy/test pass.

Commit message:

- `feat: add tea commands and ticks`

## Milestone 11: documentation and stabilization pass

Goal: make the project usable by others and reduce regression risk.

Deliverables:

- complete README examples,
- docs comparing plain ratatui vs optional `ratatui-tea`,
- API docs for all public types,
- snapshot/render test cleanup,
- release checklist.

Validation:

- docs build if enabled,
- workspace fmt/clippy/test pass.

Commit message:

- `docs: document usage paths and release checklist`
