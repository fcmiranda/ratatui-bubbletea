//! Bubbles-inspired components implemented with ratatui primitives.
//!
//! Components in this crate should work from a plain ratatui event loop and
//! should not depend on the optional `ratatui-tea` app-loop crate.

/// Returns the crate name.
///
/// This placeholder keeps Milestone 0 testable before component APIs land.
#[must_use]
pub const fn crate_name() -> &'static str {
    "ratatui-bubbletea-components"
}

#[cfg(test)]
mod tests {
    use super::crate_name;

    #[test]
    fn exposes_crate_name() {
        assert_eq!(crate_name(), "ratatui-bubbletea-components");
    }
}
