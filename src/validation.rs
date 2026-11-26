//! Checking submitted forms, like Laravel's Form Requests with their custom messages.
//!
//! Each form struct gets a `validate()` method that fills an `Errors` with one message per field.

use std::collections::BTreeMap;

/// The errors of one submitted form, by field name, in the order fields were checked.
#[derive(Debug, Default)]
pub struct Errors {
    messages: BTreeMap<&'static str, String>,
}

impl Errors {
    /// The message for a field, for templates: `{% if let Some(error) = errors.get("name") %}`.
    pub fn get(&self, field: &str) -> Option<&str> {
        self.messages.get(field).map(String::as_str)
    }

    pub fn has(&self, field: &str) -> bool {
        self.messages.contains_key(field)
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Adds a message unless the field already has one, so each field shows its first problem only.
    pub fn add(&mut self, field: &'static str, message: impl Into<String>) {
        self.messages.entry(field).or_insert_with(|| message.into());
    }

    /// `required`: not empty once trimmed.
    pub fn required(&mut self, field: &'static str, value: &str, message: &str) {
        if value.trim().is_empty() {
            self.add(field, message);
        }
    }

    /// `max:N` on text: at most `max` characters (not bytes: `ш` is one character, two bytes).
    pub fn max_chars(&mut self, field: &'static str, value: &str, max: usize, message: &str) {
        if value.chars().count() > max {
            self.add(field, message);
        }
    }
}

/// Laravel's `accepted` rule: a ticked checkbox sends `1`, `on`, `yes` or `true`.
pub fn is_accepted(value: Option<&str>) -> bool {
    matches!(value, Some("1" | "on" | "yes" | "true"))
}

/// Laravel trims every input and turns empty strings into `null`. This does the trimming; callers
/// decide what empty means for each field.
pub fn trimmed(value: &str) -> String {
    value.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_the_first_message_per_field() {
        let mut errors = Errors::default();
        errors.required("name", "  ", "Името е задолжително.");
        errors.max_chars("name", "  ", 0, "too long");

        assert_eq!(errors.get("name"), Some("Името е задолжително."));
        assert!(!errors.has("phone"));
    }

    #[test]
    fn counts_characters_not_bytes() {
        let mut errors = Errors::default();
        errors.max_chars("name", "шшш", 3, "too long");
        assert!(errors.is_empty());
    }

    #[test]
    fn accepts_checkbox_values() {
        assert!(is_accepted(Some("1")));
        assert!(is_accepted(Some("on")));
        assert!(!is_accepted(Some("0")));
        assert!(!is_accepted(None));
    }
}
