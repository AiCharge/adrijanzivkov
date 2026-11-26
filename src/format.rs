//! Formatting the Vue pages did in the browser (dayjs, `Intl.NumberFormat`), now done on the server.

use chrono::{Datelike, NaiveDateTime};
use chrono_tz::Europe::Skopje;

const MACEDONIAN_MONTHS: [&str; 12] = [
    "јануари",
    "февруари",
    "март",
    "април",
    "мај",
    "јуни",
    "јули",
    "август",
    "септември",
    "октомври",
    "ноември",
    "декември",
];

/// `2026-11-26 19:49:57` (stored in UTC) → `26 ноември 2026` (Skopje time), as dayjs's
/// `D MMMM YYYY` showed it.
pub fn date_mk(utc: NaiveDateTime) -> String {
    let local = utc.and_utc().with_timezone(&Skopje);
    // `chrono` counts months from 1, arrays from 0. `as usize` converts the number's type:
    // Rust never converts between number types on its own.
    let month = MACEDONIAN_MONTHS[local.month0() as usize];

    format!("{} {month} {}", local.day(), local.year())
}

/// A URL slug from a title, e.g. `Одржана хуманитарна акција` → `odrzana-xumanitarna-akcija`.
/// Cyrillic letters are spelled the way Laravel's `Str::slug` spelled them, so new posts get the same
/// kind of addresses as old ones.
pub fn slug(title: &str) -> String {
    let mut slug = String::with_capacity(title.len());

    for character in title.chars().flat_map(char::to_lowercase) {
        let latin = match character {
            'а' => "a",
            'б' => "b",
            'в' => "v",
            'г' => "g",
            'д' => "d",
            'ѓ' => "gj",
            'е' => "e",
            'ж' => "z",
            'з' => "z",
            'ѕ' => "dz",
            'и' => "i",
            'ј' => "j",
            'к' => "k",
            'л' => "l",
            'љ' => "lj",
            'м' => "m",
            'н' => "n",
            'њ' => "nj",
            'о' => "o",
            'п' => "p",
            'р' => "r",
            'с' => "s",
            'т' => "t",
            'ќ' => "kj",
            'у' => "u",
            'ф' => "f",
            'х' => "x",
            'ц' => "c",
            'ч' => "c",
            'џ' => "dz",
            'ш' => "s",
            'a'..='z' | '0'..='9' => {
                slug.push(character);
                continue;
            }
            _ => "-",
        };
        slug.push_str(latin);
    }

    // Collapse runs of dashes and trim them from the ends.
    slug.split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// `2026-11-26 19:49:57` (UTC) → `26.11.2026 20:49` (Skopje time), for the admin's tables.
pub fn date_time(utc: NaiveDateTime) -> String {
    utc.and_utc()
        .with_timezone(&Skopje)
        .format("%d.%m.%Y %H:%M")
        .to_string()
}

/// `120000` → `120.000`, the Macedonian thousands separator.
pub fn amount(value: i64) -> String {
    let digits = value.unsigned_abs().to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3);

    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push('.');
        }
        grouped.push(digit);
    }

    if value < 0 {
        format!("-{grouped}")
    } else {
        grouped
    }
}

/// The text of some HTML, for card previews: tags removed, common entities decoded and
/// whitespace collapsed.
pub fn strip_html(html: &str) -> String {
    let mut text = String::with_capacity(html.len());
    let mut inside_tag = false;

    for character in html.chars() {
        match character {
            '<' => {
                inside_tag = true;
                text.push(' ');
            }
            '>' => inside_tag = false,
            _ if !inside_tag => text.push(character),
            _ => {}
        }
    }

    let decoded = text
        .replace("&nbsp;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&");

    decoded.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Cuts text to `width` characters, ending in `...` when it was longer, like PHP's
/// `mb_strimwidth($text, 0, 160, '...')`.
pub fn excerpt(text: &str, width: usize) -> String {
    // Counting `chars()`, not bytes: Cyrillic letters take two bytes each in UTF-8.
    if text.chars().count() <= width {
        return text.to_string();
    }

    let kept: String = text.chars().take(width.saturating_sub(3)).collect();
    format!("{kept}...")
}

// Unit tests live next to the code they test. `cargo test` runs them.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_thousands_with_dots() {
        assert_eq!(amount(0), "0");
        assert_eq!(amount(999), "999");
        assert_eq!(amount(1000), "1.000");
        assert_eq!(amount(120000), "120.000");
        assert_eq!(amount(1234567), "1.234.567");
    }

    #[test]
    fn formats_dates_in_macedonian_skopje_time() {
        let utc =
            NaiveDateTime::parse_from_str("2026-11-26 23:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
        // 23:30 UTC is already the 27th in Skopje (UTC+1 in winter).
        assert_eq!(date_mk(utc), "27 ноември 2026");
    }

    #[test]
    fn makes_slugs_like_laravel() {
        assert_eq!(
            slug("Одржана хуманитарна акција во Кочани"),
            "odrzana-xumanitarna-akcija-vo-kocani"
        );
        assert_eq!(slug("  Нова акција!! 2026 "), "nova-akcija-2026");
        assert_eq!(slug("Џеб, Ѓорѓи и Љубица"), "dzeb-gjorgji-i-ljubica");
    }

    #[test]
    fn strips_tags_and_entities() {
        assert_eq!(
            strip_html("<p>Hello&nbsp;<strong>world</strong></p><p>again</p>"),
            "Hello world again"
        );
    }

    #[test]
    fn cuts_long_text_with_an_ellipsis() {
        assert_eq!(excerpt("кратко", 160), "кратко");
        assert_eq!(excerpt("абвгдѓежзс", 8), "абвгд...");
    }
}
