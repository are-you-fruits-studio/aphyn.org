use yew_router::prelude::*;

use crate::i18n::Lang;

/// Pages of the default language live at the root, every other language
/// under its code: `/examples` and `/uk/examples`.
///
/// Adding a page here also means adding it to `pages` in
/// `scripts/static-routes.sh`, so GitHub Pages serves it with a 200.
#[derive(Clone, Debug, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/docs")]
    Docs,
    #[at("/examples")]
    Examples,
    #[at("/roadmap")]
    Roadmap,
    #[at("/releases")]
    Releases,
    #[at("/:lang")]
    LocalHome { lang: String },
    #[at("/:lang/docs")]
    LocalDocs { lang: String },
    #[at("/:lang/examples")]
    LocalExamples { lang: String },
    #[at("/:lang/roadmap")]
    LocalRoadmap { lang: String },
    #[at("/:lang/releases")]
    LocalReleases { lang: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Home,
    Docs,
    Examples,
    Roadmap,
    Releases,
    NotFound,
}

impl Route {
    /// The page and language this route shows. An unknown language code is a
    /// missing page.
    pub fn resolve(&self) -> (Page, Lang) {
        let local = |page, code: &str| match Lang::from_code(code) {
            Some(lang) => (page, lang),
            None => (Page::NotFound, Lang::DEFAULT),
        };
        match self {
            Route::Home => (Page::Home, Lang::DEFAULT),
            Route::Docs => (Page::Docs, Lang::DEFAULT),
            Route::Examples => (Page::Examples, Lang::DEFAULT),
            Route::Roadmap => (Page::Roadmap, Lang::DEFAULT),
            Route::Releases => (Page::Releases, Lang::DEFAULT),
            Route::LocalHome { lang } => local(Page::Home, lang),
            Route::LocalDocs { lang } => local(Page::Docs, lang),
            Route::LocalExamples { lang } => local(Page::Examples, lang),
            Route::LocalRoadmap { lang } => local(Page::Roadmap, lang),
            Route::LocalReleases { lang } => local(Page::Releases, lang),
            Route::NotFound => (Page::NotFound, lang_from_location()),
        }
    }
}

impl Page {
    /// The route of this page in the given language. A missing page has no
    /// route of its own, so it points at that language's home page.
    pub fn route(self, lang: Lang) -> Route {
        if lang.is_default() {
            return match self {
                Page::Home | Page::NotFound => Route::Home,
                Page::Docs => Route::Docs,
                Page::Examples => Route::Examples,
                Page::Roadmap => Route::Roadmap,
                Page::Releases => Route::Releases,
            };
        }
        let lang = lang.code().to_owned();
        match self {
            Page::Home | Page::NotFound => Route::LocalHome { lang },
            Page::Docs => Route::LocalDocs { lang },
            Page::Examples => Route::LocalExamples { lang },
            Page::Roadmap => Route::LocalRoadmap { lang },
            Page::Releases => Route::LocalReleases { lang },
        }
    }
}

/// A missing page under `/<code>/…` answers in that language.
fn lang_from_location() -> Lang {
    web_sys::window()
        .and_then(|w| w.location().pathname().ok())
        .and_then(|path| path.split('/').nth(1).and_then(Lang::from_code))
        .unwrap_or(Lang::DEFAULT)
}
