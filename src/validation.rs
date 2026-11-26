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

/// Laravel's `email` rule, simply: one `@`, something on both sides, no spaces.
pub fn is_email(value: &str) -> bool {
    match value.split_once('@') {
        Some((local, domain)) => {
            !local.is_empty()
                && !domain.is_empty()
                && !domain.contains('@')
                && !value.chars().any(char::is_whitespace)
        }
        None => false,
    }
}

/// The password rule used in production (Laravel's `Password::defaults()` there): at least 12
/// characters, with lower and upper case letters, a digit and a symbol.
pub fn is_strong_password(value: &str) -> bool {
    value.chars().count() >= 12
        && value.chars().any(char::is_lowercase)
        && value.chars().any(char::is_uppercase)
        && value.chars().any(|c| c.is_ascii_digit())
        && value
            .chars()
            .any(|c| !c.is_alphanumeric() && !c.is_whitespace())
}

/// Laravel's `integer|min:1|max:4294967295` on an amount typed into a form.
pub fn parse_amount(value: &str) -> Result<i64, &'static str> {
    if value.is_empty() {
        return Err("Потребниот износ е задолжителен.");
    }
    let amount: i64 = value
        .parse()
        .map_err(|_| "Потребниот износ мора да биде цел број.")?;
    if amount < 1 {
        return Err("Потребниот износ мора да биде најмалку 1 МКД.");
    }
    if amount > 4_294_967_295 {
        return Err("Потребниот износ е преголем.");
    }
    Ok(amount)
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
    fn checks_emails_passwords_and_amounts() {
        assert!(is_email("admin@gmail.com"));
        assert!(!is_email("admin"));
        assert!(!is_email("a b@c.com"));
        assert!(is_strong_password("Тајна-Лозинка-2026"));
        assert!(!is_strong_password("short1A!"));
        assert!(!is_strong_password("alllowercase-2026"));
        assert_eq!(parse_amount("120000"), Ok(120000));
        assert!(parse_amount("10.5").is_err());
        assert!(parse_amount("0").is_err());
        assert!(parse_amount("много").is_err());
    }

    #[test]
    fn accepts_checkbox_values() {
        assert!(is_accepted(Some("1")));
        assert!(is_accepted(Some("on")));
        assert!(!is_accepted(Some("0")));
        assert!(!is_accepted(None));
    }
}
