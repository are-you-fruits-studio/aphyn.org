use yew::prelude::*;

use super::icon::Icon;
use super::logo::Logo;
use super::nav_link::NavLink;
use crate::i18n::Lang;
use crate::route::Page;
use crate::theme::Theme;

pub const DOCS_URL: &str = "https://docs.aphyn.org";
pub const REPO_URL: &str = "https://github.com/are-you-fruits-studio/aphyn";
pub const RELEASES_URL: &str = "https://github.com/are-you-fruits-studio/aphyn/releases";

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub page: Page,
    pub lang: Lang,
    pub theme: Theme,
    pub on_toggle_theme: Callback<()>,
}

/// Top bar. Under 760px the navigation folds into a menu button.
#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let HeaderProps {
        page, lang, theme, ..
    } = *props;
    let menu_open = use_state(|| false);

    {
        let menu_open = menu_open.clone();
        use_effect_with((page, lang), move |_| menu_open.set(false));
    }

    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };
    let close_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |()| menu_open.set(false))
    };
    let toggle_theme = {
        let cb = props.on_toggle_theme.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let (theme_icon, theme_label) = match theme {
        Theme::Dark => ("sun", lang.t("theme.to_light")),
        Theme::Light => ("moon", lang.t("theme.to_dark")),
    };

    // Every header link stays on this site. Docs and Releases are pages of
    // their own that introduce the external docs and GitHub releases.
    let nav_links = |onnavigate: Option<Callback<()>>| {
        let link = |target: Page, key: &'static str| {
            html! {
                <NavLink to={target.route(lang)} current={page == target} onnavigate={onnavigate.clone()}>
                    { lang.t(key) }
                </NavLink>
            }
        };
        html! {
            <>
                { link(Page::Docs, "nav.docs") }
                { link(Page::Examples, "nav.examples") }
                { link(Page::Roadmap, "nav.roadmap") }
                { link(Page::Releases, "nav.releases") }
            </>
        }
    };

    html! {
        <header class="header">
            <a class="sr-only" href="#content">{ lang.t("nav.skip") }</a>
            <div class="wrap header-bar">
                <NavLink class="header-logo" to={Page::Home.route(lang)}>
                    <Logo size={26} />
                    <span class="sr-only">{ lang.t("nav.home") }</span>
                </NavLink>
                <nav class="nav-main" aria-label={lang.t("nav.label")}>
                    { nav_links(None) }
                </nav>
                <div class="header-spacer"></div>
                <div class="header-tools">
                    <LangSwitch {page} {lang} />
                    <button type="button" class="icon-btn" aria-label={theme_label} title={theme_label} onclick={toggle_theme}>
                        <Icon name={theme_icon} size={18} />
                    </button>
                    <a class="icon-btn gh-btn" href={REPO_URL} aria-label={lang.t("nav.github")} title={lang.t("nav.github")}>
                        <Icon name="github" size={18} />
                    </a>
                    <button
                        type="button"
                        class="icon-btn menu-btn"
                        aria-label={lang.t("nav.menu")}
                        aria-expanded={menu_open.to_string()}
                        aria-controls="mobile-menu"
                        onclick={toggle_menu}
                    >
                        <Icon name="menu" size={20} />
                    </button>
                </div>
            </div>
            <nav id="mobile-menu" class={classes!("mobile-menu", menu_open.then_some("open"))} aria-label={lang.t("nav.label")}>
                { nav_links(Some(close_menu.clone())) }
                <div class="mobile-menu-foot">
                    <LangSwitch {page} {lang} onnavigate={close_menu} />
                    <a class="mono-link" href={REPO_URL}>
                        <Icon name="github" size={16} />
                        { "GitHub" }
                    </a>
                </div>
            </nav>
        </header>
    }
}

#[derive(Properties, PartialEq)]
struct LangSwitchProps {
    page: Page,
    lang: Lang,
    #[prop_or_default]
    onnavigate: Option<Callback<()>>,
}

/// One entry per language in the registry; the current one is not a link.
#[function_component(LangSwitch)]
fn lang_switch(props: &LangSwitchProps) -> Html {
    let LangSwitchProps { page, lang, .. } = *props;
    html! {
        <nav class="lang-switch" aria-label={lang.t("nav.language")}>
            { for Lang::all().map(|l| if l == lang {
                html! { <span aria-current="true" title={l.name()} lang={l.code()}>{ l.label() }</span> }
            } else {
                html! {
                    <NavLink to={page.route(l)} hreflang={Some(l.code())} onnavigate={props.onnavigate.clone()}>
                        <span class="sr-only">{ l.name() }</span>
                        <span aria-hidden="true">{ l.label() }</span>
                    </NavLink>
                }
            }) }
        </nav>
    }
}
