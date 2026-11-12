use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::i18n::Lang;
use crate::pages::{
    docs::Docs, examples::Examples, home::Home, not_found::NotFound, releases::Releases,
    roadmap::Roadmap,
};
use crate::releases::ReleasesProvider;
use crate::route::{Page, Route};
use crate::theme::Theme;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ReleasesProvider>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ReleasesProvider>
    }
}

fn switch(route: Route) -> Html {
    let (page, lang) = route.resolve();
    html! { <Layout {page} {lang} /> }
}

#[derive(Properties, PartialEq)]
struct LayoutProps {
    page: Page,
    lang: Lang,
}

#[function_component(Layout)]
fn layout(props: &LayoutProps) -> Html {
    let LayoutProps { page, lang } = *props;
    let theme = use_state(Theme::current);

    // Title, `lang` attribute and scroll position follow the page.
    use_effect_with((page, lang), |&(page, lang)| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                document.set_title(title(page, lang));
                if let Some(root) = document.document_element() {
                    let _ = root.set_attribute("lang", lang.code());
                }
            }
            let has_hash = window
                .location()
                .hash()
                .map(|h| !h.is_empty())
                .unwrap_or(false);
            if !has_hash {
                window.scroll_to_with_x_and_y(0.0, 0.0);
            }
        }
    });

    let on_toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let next = theme.toggled();
            next.apply();
            theme.set(next);
        })
    };

    let content = match page {
        Page::Home => html! { <Home {lang} /> },
        Page::Examples => html! { <Examples {lang} /> },
        Page::Docs => html! { <Docs {lang} /> },
        Page::Roadmap => html! { <Roadmap {lang} /> },
        Page::Releases => html! { <Releases {lang} /> },
        Page::NotFound => html! { <NotFound {lang} /> },
    };

    html! {
        <div class="site">
            <Header {page} {lang} theme={*theme} {on_toggle_theme} />
            <main id="content">{ content }</main>
            if page != Page::NotFound {
                <Footer {lang} full={page == Page::Home} />
            }
        </div>
    }
}

fn title(page: Page, lang: Lang) -> &'static str {
    lang.t(match page {
        Page::Home => "title.home",
        Page::Examples => "title.examples",
        Page::Docs => "title.docs",
        Page::Roadmap => "title.roadmap",
        Page::Releases => "title.releases",
        Page::NotFound => "title.not_found",
    })
}
