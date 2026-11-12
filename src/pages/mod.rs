pub mod docs;
pub mod examples;
pub mod home;
pub mod not_found;
pub mod releases;
pub mod roadmap;

use yew::prelude::*;

use crate::i18n::Lang;

#[derive(Properties, PartialEq)]
pub struct PageProps {
    pub lang: Lang,
}

/// A sentence with inline code in the middle: `before code after`. Locale
/// values are trimmed, so the spaces are added here; none goes before `after`
/// when it starts with punctuation.
pub fn with_code(before: &str, code: &'static str, after: &str) -> Html {
    let after = match after.chars().next() {
        Some(c) if c.is_alphanumeric() => format!(" {after}"),
        _ => after.to_owned(),
    };
    html! {
        <>
            { before.to_owned() }
            if !before.is_empty() { { " " } }
            <code class="inline-code">{ code }</code>
            { after }
        </>
    }
}
