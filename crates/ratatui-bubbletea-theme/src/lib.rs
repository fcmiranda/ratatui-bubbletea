//! Charm/Bubble Tea-inspired theme helpers for ratatui.
//!
//! This crate is intentionally usable from a normal ratatui event loop.

/// Returns the crate name.
///
/// This placeholder keeps Milestone 0 testable before the theme API lands.
#[must_use]
pub const fn crate_name() -> &'static str {
    "ratatui-bubbletea-theme"
}

#[cfg(test)]
mod tests {
    use super::crate_name;

    #[test]
    fn exposes_crate_name() {
        assert_eq!(crate_name(), "ratatui-bubbletea-theme");
    }
}
