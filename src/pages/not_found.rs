use yew::prelude::*;

use super::PageProps;
use crate::components::icon::Icon;
use crate::components::nav_link::NavLink;
use crate::route::Page;

#[function_component(NotFound)]
pub fn not_found(props: &PageProps) -> Html {
    let lang = props.lang;
    let t = |key| lang.t(key);
    html! {
        <section class="wrap section nf">
            <div>
                <div class="code">{ t("nf.code") }</div>
                <h1 class="h1 h1-page">{ t("nf.title") }</h1>
                <p class="lead">{ t("nf.body") }</p>
                <div class="btn-row">
                    <NavLink class="btn btn-primary" to={Page::Home.route(lang)}>{ t("nf.home") }</NavLink>
                    <NavLink class="btn btn-secondary" to={Page::Docs.route(lang)}>
                        <Icon name="book-open" />
                        { t("nav.docs") }
                    </NavLink>
                </div>
                <p class="comment" style="margin-top: 40px;">{ t("nf.comment") }</p>
            </div>
            // An empty outline where the berry would be: nothing grows here.
            <div class="nf-mark" aria-hidden="true">
                <svg width="160" height="160" viewBox="0 0 32 32"><circle cx="16" cy="16" r="14" /></svg>
            </div>
        </section>
    }
}
