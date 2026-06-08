# ratatui-bubbletea

Charm/Bubble Tea-inspired themes and components for ratatui.

The first goal is not to replace ratatui's rendering or app loop. Existing ratatui apps should be able to adopt the look incrementally by using theme helpers and ratatui-native components.

Planned layers:

- `ratatui-bubbletea-theme`: semantic theme tokens and styled helpers for existing ratatui widgets.
- `ratatui-bubbletea-components`: Bubbles-inspired components implemented as ratatui widgets/stateful widgets.
- `ratatui-tea`: optional future app-loop shell for users who want Bubble Tea-style `Model` / `Msg` / `Cmd` ergonomics.

See [`docs/milestones.md`](docs/milestones.md) for the implementation plan.
