/// A key binding with one or more aliases and a human-readable description.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct KeyBinding {
    keys: Vec<String>,
    description: String,
}

impl KeyBinding {
    /// Creates a binding with a single key label.
    #[must_use]
    pub fn new(key: impl Into<String>, description: impl Into<String>) -> Self {
        Self::with_keys([key], description)
    }

    /// Creates a binding with multiple key aliases.
    #[must_use]
    pub fn with_keys<K, I>(keys: I, description: impl Into<String>) -> Self
    where
        K: Into<String>,
        I: IntoIterator<Item = K>,
    {
        Self {
            keys: keys.into_iter().map(Into::into).collect(),
            description: description.into(),
        }
    }

    /// Returns all configured key aliases.
    #[must_use]
    pub fn keys(&self) -> &[String] {
        &self.keys
    }

    /// Returns the human-readable description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the compact display label, for example `q/esc`.
    #[must_use]
    pub fn label(&self) -> String {
        self.keys.join("/")
    }

    /// Returns whether the provided key label matches any configured alias.
    #[must_use]
    pub fn matches(&self, input: &str) -> bool {
        let input = normalize_key(input);
        self.keys.iter().any(|key| normalize_key(key) == input)
    }
}

/// A collection of related key bindings.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct KeyMap {
    bindings: Vec<KeyBinding>,
}

impl KeyMap {
    /// Creates a key map from bindings.
    #[must_use]
    pub fn new(bindings: impl IntoIterator<Item = KeyBinding>) -> Self {
        Self {
            bindings: bindings.into_iter().collect(),
        }
    }

    /// Returns the bindings in display order.
    #[must_use]
    pub fn bindings(&self) -> &[KeyBinding] {
        &self.bindings
    }

    /// Adds a binding to the end of the key map.
    pub fn push(&mut self, binding: KeyBinding) {
        self.bindings.push(binding);
    }

    /// Finds the first binding matching the provided key label.
    #[must_use]
    pub fn find(&self, input: &str) -> Option<&KeyBinding> {
        self.bindings.iter().find(|binding| binding.matches(input))
    }

    /// Returns true when any binding matches the provided key label.
    #[must_use]
    pub fn matches(&self, input: &str) -> bool {
        self.find(input).is_some()
    }
}

fn normalize_key(input: &str) -> String {
    match input.trim().to_ascii_lowercase().replace('-', "+").as_str() {
        "escape" => "esc".to_owned(),
        "return" => "enter".to_owned(),
        "arrow+up" => "up".to_owned(),
        "arrow+down" => "down".to_owned(),
        "arrow+left" => "left".to_owned(),
        "arrow+right" => "right".to_owned(),
        "c+c" => "ctrl+c".to_owned(),
        other => other.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::{KeyBinding, KeyMap};

    #[test]
    fn binding_matches_aliases_and_normalized_forms() {
        let binding = KeyBinding::with_keys(["q", "esc", "ctrl+c"], "quit");

        assert!(binding.matches("q"));
        assert!(binding.matches("ESCAPE"));
        assert!(binding.matches("ctrl-c"));
        assert!(binding.matches("c-c"));
        assert!(!binding.matches("j"));
    }

    #[test]
    fn binding_label_joins_aliases() {
        let binding = KeyBinding::with_keys(["q", "esc"], "quit");

        assert_eq!(binding.label(), "q/esc");
        assert_eq!(binding.description(), "quit");
    }

    #[test]
    fn keymap_finds_first_matching_binding() {
        let keymap = KeyMap::new([
            KeyBinding::with_keys(["j", "down"], "down"),
            KeyBinding::with_keys(["k", "up"], "up"),
        ]);

        assert_eq!(
            keymap.find("arrow-down").map(KeyBinding::description),
            Some("down")
        );
        assert!(keymap.matches("k"));
        assert!(!keymap.matches("q"));
    }
}
