# Implementation status

This file records the current state so future sessions can resume after context compaction.

## Completed milestones

- Milestone 0: workspace skeleton.
- Milestone 1: Charm-inspired theme primitives and ratatui widget helpers.
- Milestone 2: key bindings, key map, and help footer.
- Milestone 3: spinner component.
- Milestone 4: progress component.
- Milestone 5: selectable list component.
- Milestone 6: themed table wrapper/helpers.
- Milestone 7: text input component.
- Milestone 8: viewport/pager component.
- Milestone 9: optional `ratatui-tea` MVP with `Model`, `Cmd`, `Program`, `ProgramHandle` channel, and ratatui draw integration.
- Milestone 10: `ratatui-tea` command helpers: `once`, `batch`, `sequence`, and one-shot `tick`.

## Current crate layout

```text
crates/
  ratatui-bubbletea-theme/
  ratatui-bubbletea-components/
  ratatui-tea/
```

## Implemented components

`ratatui-bubbletea-components` currently exports:

- `KeyBinding`, `KeyMap`,
- `Help`, `HelpMode`,
- `Spinner`, `SpinnerFrames`, `SpinnerState`,
- `Progress`, `ProgressSymbols`,
- `ListItem`, `ListState`, `SelectList`,
- `ThemedTable`,
- `TextInput`, `TextInputState`,
- `Viewport`, `ViewportState`.

## Validation command

Run this before committing changes:

```bash
cargo fmt --check
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

## Important design constraints

- Do not make `ratatui-bubbletea-theme` or `ratatui-bubbletea-components` depend on `ratatui-tea`.
- Keep components usable from plain ratatui event loops.
- Keep `ratatui-tea` optional and adapter-like.
- Do not bypass ratatui with ANSI string rendering.
- Prefer stable ratatui APIs.

## Known follow-ups

- Decide whether `ratatui-tea::Cmd::tick` should move from blocking synchronous sleep to an async/threaded executor.
- Add real terminal lifecycle/event-loop helpers behind an optional crossterm feature if desired.
- Add more screenshots/golden render docs once visual defaults settle.
- Consider publishing strategy: current workspace uses local path dependency to `../ratatui/ratatui` for development against the inspected repo.
