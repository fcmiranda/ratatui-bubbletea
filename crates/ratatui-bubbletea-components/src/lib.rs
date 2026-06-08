//! Bubbles-inspired components implemented with ratatui primitives.
//!
//! Components in this crate work from a plain ratatui event loop and do not
//! depend on the optional `ratatui-tea` app-loop crate.

mod help;
mod key;
mod list;
mod progress;
mod spinner;
mod table;
mod text_input;
mod viewport;

pub use help::{Help, HelpMode};
pub use key::{KeyBinding, KeyMap};
pub use list::{ListItem, ListState, SelectList};
pub use progress::{Progress, ProgressSymbols};
pub use spinner::{Spinner, SpinnerFrames, SpinnerState};
pub use table::ThemedTable;
pub use text_input::{TextInput, TextInputState};
pub use viewport::{Viewport, ViewportState};
