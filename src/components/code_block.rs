use gloo_timers::callback::Timeout;
use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use yew::prelude::*;

use super::icon::Icon;
use crate::i18n::Lang;

#[derive(Clone, Copy, PartialEq)]
pub enum Language {
    Rust,
    Toml,
    Shell,
}

impl Language {
    fn name(self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::Toml => "toml",
            Language::Shell => "shell",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct CodeBlockProps {
    pub code: AttrValue,
    pub language: Language,
    pub filename: AttrValue,
    pub lang: Lang,
    #[prop_or(false)]
    pub line_numbers: bool,
}

/// Syntax-highlighted code with a copy button. Always dark.
#[function_component(CodeBlock)]
pub fn code_block(props: &CodeBlockProps) -> Html {
    let copied = use_state(|| false);
    let timer = use_mut_ref(|| None::<Timeout>);

    let on_copy = {
        let code = props.code.clone();
        let copied = copied.clone();
        Callback::from(move |_| {
            copy_to_clipboard(&code);
            copied.set(true);
            let copied = copied.clone();
            *timer.borrow_mut() = Some(Timeout::new(1400, move || copied.set(false)));
        })
    };

    let code = props.code.trim_end_matches('\n');
    html! {
        <div class="code-block">
            <div class="code-head">
                <Icon name="file-code" size={13} />
                <span class="code-file">{ props.filename.clone() }</span>
                <span class="code-lang">{ props.language.name() }</span>
                <button type="button" class={classes!("code-copy", copied.then_some("copied"))} aria-label={props.lang.t("code.copy")} onclick={on_copy}>
                    <Icon name={if *copied { "check" } else { "copy" }} size={13} />
                    if *copied { { props.lang.t("code.copied") } }
                </button>
            </div>
            <pre><code>
                { for code.split('\n').enumerate().map(|(i, line)| html! {
                    <div class="code-line">
                        if props.line_numbers { <span class="ln" aria-hidden="true">{ i + 1 }</span> }
                        <span class="src">
                            { for highlight(line, props.language).into_iter().map(|(kind, text)| match kind {
                                Tok::Plain => html! { { text } },
                                kind => html! { <span class={kind.class()}>{ text }</span> },
                            }) }
                            if line.is_empty() { { " " } }
                        </span>
                    </div>
                }) }
            </code></pre>
        </div>
    }
}

fn copy_to_clipboard(text: &str) {
    // navigator.clipboard.writeText(text), without needing web-sys' unstable Clipboard API.
    let Some(window) = web_sys::window() else {
        return;
    };
    let navigator: JsValue = window.navigator().into();
    let Ok(clipboard) = Reflect::get(&navigator, &"clipboard".into()) else {
        return;
    };
    if clipboard.is_undefined() {
        return;
    }
    if let Ok(write) = Reflect::get(&clipboard, &"writeText".into()) {
        if let Some(write) = write.dyn_ref::<Function>() {
            let _ = write.call1(&clipboard, &JsValue::from_str(text));
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tok {
    Plain,
    Comment,
    Str,
    Keyword,
    Number,
    Type,
    Func,
    Prompt,
}

impl Tok {
    fn class(self) -> &'static str {
        match self {
            Tok::Plain => "",
            Tok::Comment => "tok-comment",
            Tok::Str => "tok-string",
            Tok::Keyword => "tok-keyword",
            Tok::Number => "tok-number",
            Tok::Type => "tok-type",
            Tok::Func => "tok-fn",
            Tok::Prompt => "tok-prompt",
        }
    }
}

const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "dyn",
];

/// The design system's small highlighter for Rust, TOML and shell: comments
/// (`//`, and `#` outside Rust or for Rust attributes), strings, keywords,
/// numbers, capitalised types, calls and a `$ ` prompt.
fn highlight(line: &str, language: Language) -> Vec<(Tok, &str)> {
    let bytes = line.as_bytes();
    // (kind, start, end) byte ranges; neighbouring plain runs are merged.
    let mut spans: Vec<(Tok, usize, usize)> = Vec::new();
    let push =
        |spans: &mut Vec<(Tok, usize, usize)>, kind: Tok, start: usize, end: usize| match spans
            .last_mut()
        {
            Some(last) if last.0 == Tok::Plain && kind == Tok::Plain && last.2 == start => {
                last.2 = end
            }
            _ => spans.push((kind, start, end)),
        };
    let is_ident = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        let prev_is_word = i > 0 && is_ident(bytes[i - 1]);
        let rest = &line[i..];
        // A comment starts a line or follows a space, so `https://` stays a string.
        let at_word_start = i == 0 || bytes[i - 1].is_ascii_whitespace();
        if (rest.starts_with("//") && at_word_start)
            || (b == b'#' && (language != Language::Rust || bytes.get(i + 1) == Some(&b'[')))
        {
            push(&mut spans, Tok::Comment, i, bytes.len());
            break;
        }
        if b == b'"' {
            let mut j = i + 1;
            while j < bytes.len() && bytes[j] != b'"' {
                j += if bytes[j] == b'\\' { 2 } else { 1 };
            }
            let end = (j + 1).min(bytes.len());
            push(&mut spans, Tok::Str, i, end);
            i = end;
            continue;
        }
        if rest.starts_with("$ ") {
            push(&mut spans, Tok::Prompt, i, i + 2);
            i += 2;
            continue;
        }
        if !prev_is_word && b.is_ascii_digit() {
            let mut j = i;
            while j < bytes.len() && (bytes[j].is_ascii_digit() || bytes[j] == b'_') {
                j += 1;
            }
            if j + 1 < bytes.len() && bytes[j] == b'.' && bytes[j + 1].is_ascii_digit() {
                j += 1;
                while j < bytes.len() && bytes[j].is_ascii_digit() {
                    j += 1;
                }
            }
            if j < bytes.len() && is_ident(bytes[j]) {
                // Part of a longer word such as `0x` or `2d`: leave it plain.
                while j < bytes.len() && is_ident(bytes[j]) {
                    j += 1;
                }
                push(&mut spans, Tok::Plain, i, j);
            } else {
                push(&mut spans, Tok::Number, i, j);
            }
            i = j;
            continue;
        }
        if !prev_is_word && (b.is_ascii_alphabetic() || b == b'_') {
            let mut j = i;
            while j < bytes.len() && is_ident(bytes[j]) {
                j += 1;
            }
            let word = &line[i..j];
            let next = line[j..].trim_start().bytes().next();
            let kind = if KEYWORDS.contains(&word) {
                Tok::Keyword
            } else if b.is_ascii_uppercase() {
                Tok::Type
            } else if word.bytes().all(|c| !c.is_ascii_uppercase())
                && matches!(next, Some(b'(' | b'!'))
            {
                Tok::Func
            } else {
                Tok::Plain
            };
            push(&mut spans, kind, i, j);
            i = j;
            continue;
        }
        let len = rest.chars().next().map(char::len_utf8).unwrap_or(1);
        push(&mut spans, Tok::Plain, i, i + len);
        i += len;
    }
    spans
        .into_iter()
        .map(|(kind, start, end)| (kind, &line[start..end]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlights_rust() {
        let toks = highlight("    let mut app = App::new(); // hi", Language::Rust);
        assert!(toks.contains(&(Tok::Keyword, "let")));
        assert!(toks.contains(&(Tok::Type, "App")));
        assert!(toks.contains(&(Tok::Func, "new")));
        assert!(toks.contains(&(Tok::Comment, "// hi")));
    }

    #[test]
    fn hash_is_a_comment_outside_rust() {
        let toks = highlight("# zip it", Language::Shell);
        assert_eq!(toks, vec![(Tok::Comment, "# zip it")]);
    }

    #[test]
    fn a_url_is_not_a_comment() {
        let toks = highlight("git clone https://github.com/x.git", Language::Shell);
        assert!(toks.iter().all(|t| t.0 != Tok::Comment));
    }

    #[test]
    fn keeps_every_character() {
        let line = "aphyn = { git = \"https://x\", tag = \"v0.21.0\" } # ok";
        let joined: String = highlight(line, Language::Toml)
            .into_iter()
            .map(|t| t.1)
            .collect();
        assert_eq!(joined, line);
    }
}
