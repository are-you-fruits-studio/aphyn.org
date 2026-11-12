//! Engine releases, pulled from the engine repository.
//!
//! The deploy workflow asks the GitHub API for the latest releases of
//! are-you-fruits-studio/aphyn, writes them to `/releases.json` next to the
//! site and passes the newest tag to the build as `APHYN_VERSION`. The site
//! shows that version from the first paint, then reads `/releases.json` from
//! its own domain, so visitors never call the GitHub API themselves.

use js_sys::{Array, Date, Object, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;

use crate::i18n::Lang;

/// The version baked in at build time; the last known release otherwise.
pub const BUILD_VERSION: &str = match option_env!("APHYN_VERSION") {
    Some(v) => v,
    None => "v0.21.0",
};

#[derive(Clone, Debug, PartialEq)]
pub struct Release {
    pub tag: String,
    pub name: String,
    /// ISO 8601 publication time.
    pub date: String,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Releases {
    /// Newest first. Empty until `/releases.json` is loaded, or when it is missing.
    pub list: Vec<Release>,
}

impl Releases {
    pub fn latest_tag(&self) -> String {
        self.list
            .first()
            .map(|r| r.tag.clone())
            .unwrap_or_else(|| BUILD_VERSION.to_owned())
    }
}

/// Provides [`Releases`] to the whole site and loads them once.
#[derive(Properties, PartialEq)]
pub struct ReleasesProviderProps {
    pub children: Html,
}

#[function_component(ReleasesProvider)]
pub fn releases_provider(props: &ReleasesProviderProps) -> Html {
    let releases = use_state(|| Releases { list: Vec::new() });
    {
        let releases = releases.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(list) = load().await {
                    releases.set(Releases { list });
                }
            });
        });
    }
    html! {
        <ContextProvider<Releases> context={(*releases).clone()}>
            { props.children.clone() }
        </ContextProvider<Releases>>
    }
}

async fn load() -> Option<Vec<Release>> {
    let window = web_sys::window()?;
    let response: web_sys::Response = JsFuture::from(window.fetch_with_str("/releases.json"))
        .await
        .ok()?
        .dyn_into()
        .ok()?;
    if !response.ok() {
        return None;
    }
    let json = JsFuture::from(response.json().ok()?).await.ok()?;
    let items: Array = json.dyn_into().ok()?;
    let field = |item: &JsValue, key: &str| {
        Reflect::get(item, &JsValue::from_str(key))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default()
    };
    let list: Vec<Release> = items
        .iter()
        .map(|item| Release {
            tag: field(&item, "tag"),
            name: field(&item, "name"),
            date: field(&item, "date"),
            url: field(&item, "url"),
        })
        .filter(|r| !r.tag.is_empty())
        .collect();
    (!list.is_empty()).then_some(list)
}

/// "12 Nov 2026" in the page's language.
pub fn format_date(iso: &str, lang: Lang) -> String {
    let date = Date::new(&JsValue::from_str(iso));
    if date.get_time().is_nan() {
        return String::new();
    }
    let options = Object::new();
    let _ = Reflect::set(&options, &"year".into(), &"numeric".into());
    let _ = Reflect::set(&options, &"month".into(), &"short".into());
    let _ = Reflect::set(&options, &"day".into(), &"numeric".into());
    date.to_locale_date_string(lang.code(), &options).into()
}
