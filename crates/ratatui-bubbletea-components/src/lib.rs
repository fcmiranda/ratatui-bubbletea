//! Bubbles-inspired components implemented with ratatui primitives.
//!
//! Components in this crate work from a plain ratatui event loop and do not
//! depend on the optional `ratatui-tea` app-loop crate.

mod help;
mod key;

pub use help::{Help, HelpMode};
pub use key::{KeyBinding, KeyMap};
