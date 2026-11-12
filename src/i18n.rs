//! Localisation.
//!
//! Every language is one file, `locales/<code>.txt`, embedded at build time and
//! registered in [`LOCALES`]. The first locale is the default: it lives at the
//! site root (`/`, `/examples`), every other one under its code (`/uk`,
//! `/uk/examples`). A key missing from a translation falls back to the default
//! locale, so a new language can start with only part of the text translated.
//!
//! File format, one message per line:
//!
//! ```text
//! # comment
//! home.title = A 3D game engine in Rust, grown from real games.
//! code.run.note = // first line\nsecond line
//! ```
//!
//! Keys are free-form dotted names. In values `\n` is a line break and `\\` a
//! backslash; everything else is literal text. Leading and trailing spaces are
//! trimmed.
//!
//! To add a language: copy `locales/en.txt` to `locales/<code>.txt`, translate
//! it and add one entry to [`LOCALES`]. Routes, the language switch, page
//! titles and the static pages for GitHub Pages pick it up from there (see the
//! README).

use std::collections::HashMap;
use std::sync::OnceLock;

pub struct Locale {
    /// BCP 47 code, used in the URL (`/uk/…`) and the `lang` attribute.
    pub code: &'static str,
    /// Short label in the language switch.
    pub label: &'static str,
    /// The language's own name, for screen readers and link titles.
    pub name: &'static str,
    source: &'static str,
}

/// All languages of the site. The first one is the default.
pub const LOCALES: &[Locale] = &[
    Locale {
        code: "en",
        label: "EN",
        name: "English",
        source: include_str!("../locales/en.txt"),
    },
    Locale {
        code: "uk",
        label: "УК",
        name: "Українська",
        source: include_str!("../locales/uk.txt"),
    },
];

/// A language of the site: an index into [`LOCALES`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lang(usize);

impl Lang {
    pub const DEFAULT: Lang = Lang(0);

    pub fn all() -> impl Iterator<Item = Lang> {
        (0..LOCALES.len()).map(Lang)
    }

    pub fn from_code(code: &str) -> Option<Lang> {
        LOCALES
            .iter()
            .position(|l| l.code.eq_ignore_ascii_case(code))
            .map(Lang)
    }

    fn locale(self) -> &'static Locale {
        &LOCALES[self.0]
    }

    pub fn code(self) -> &'static str {
        self.locale().code
    }

    pub fn label(self) -> &'static str {
        self.locale().label
    }

    pub fn name(self) -> &'static str {
        self.locale().name
    }

    pub fn is_default(self) -> bool {
        self == Lang::DEFAULT
    }

    /// The text for `key`, else the default locale's, else the key itself.
    pub fn t(self, key: &str) -> &'static str {
        let tables = tables();
        tables[self.0]
            .get(key)
            .or_else(|| tables[Lang::DEFAULT.0].get(key))
            .copied()
            .unwrap_or_else(|| {
                web_sys::console::warn_1(&format!("missing text: {key}").into());
                Box::leak(key.to_owned().into_boxed_str())
            })
    }
}

fn tables() -> &'static [HashMap<&'static str, &'static str>] {
    static TABLES: OnceLock<Vec<HashMap<&'static str, &'static str>>> = OnceLock::new();
    TABLES.get_or_init(|| LOCALES.iter().map(|l| parse(l.source)).collect())
}

fn parse(source: &'static str) -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let value = value.trim();
        let value: &'static str = if value.contains('\\') {
            Box::leak(unescape(value).into_boxed_str())
        } else {
            value
        };
        map.insert(key.trim(), value);
    }
    map
}

fn unescape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_locale_has_every_default_key() {
        let tables = tables();
        for (locale, table) in LOCALES.iter().zip(tables).skip(1) {
            let missing: Vec<_> = tables[0]
                .keys()
                .filter(|k| !table.contains_key(*k))
                .collect();
            assert!(missing.is_empty(), "{} is missing {missing:?}", locale.code);
        }
    }

    #[test]
    fn unescapes_line_breaks() {
        assert_eq!(unescape(r"a\nb\\c"), "a\nb\\c");
    }
}
