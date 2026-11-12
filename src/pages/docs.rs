use yew::prelude::*;

use super::{with_code, PageProps};
use crate::components::badge::{Badge, Tone};
use crate::components::code_block::{CodeBlock, Language};
use crate::components::header::{DOCS_URL, REPO_URL};
use crate::components::icon::Icon;
use crate::components::nav_link::NavLink;
use crate::route::Page;

const GETTING_STARTED_URL: &str = "https://docs.aphyn.org/docs/getting-started";
const API_REFERENCE_URL: &str = "https://docs.aphyn.org/api-reference/aphyn/index.html";
const README_GUIDE_URL: &str = "https://github.com/are-you-fruits-studio/aphyn#build-a-game";

/// An on-site introduction to the documentation at docs.aphyn.org: what is
/// there, what is not yet, and where else to learn from.
#[function_component(Docs)]
pub fn docs(props: &PageProps) -> Html {
    let lang = props.lang;
    let t = |key| lang.t(key);

    // A card that leaves the site: the whole card is the link.
    let out_card = |href: &'static str,
                    icon: &'static str,
                    host: &'static str,
                    title: &'static str,
                    body: &'static str| {
        html! {
            <a class="dest" {href}>
                <span class="card-icon" aria-hidden="true"><Icon name={icon} size={17} /></span>
                <span class="dest-host">{ host }<Icon name="external-link" size={12} /></span>
                <span class="dest-title">{ title }</span>
                <span class="dest-body">{ body }</span>
            </a>
        }
    };

    html! {
        <>
            <section class="page-hero">
                <div class="wrap split center">
                    <div>
                        <div class="eyebrow">{ t("docs.eyebrow") }</div>
                        <h1 class="h1 h1-page">{ t("docs.title") }</h1>
                        <p class="lead">{ t("docs.lead") }</p>
                        <div class="btn-row">
                            <a class="btn btn-primary" href={DOCS_URL}>
                                { t("docs.open") }
                                <Icon name="external-link" />
                            </a>
                            <NavLink class="btn btn-secondary" to={Page::Examples.route(lang)}>
                                <Icon name="code" />
                                { t("docs.examples_cta") }
                            </NavLink>
                        </div>
                    </div>
                    // A small "map" of the docs site, drawn from its real sections.
                    <div class="docs-map" aria-hidden="true">
                        <div class="docs-map-bar">
                            <span class="dot"></span><span class="dot"></span><span class="dot"></span>
                            <span class="docs-map-url">{ "docs.aphyn.org" }</span>
                        </div>
                        <div class="docs-map-body">
                            <div class="docs-map-side">
                                <span class="on">{ "Getting Started" }</span>
                                <span>{ "API Reference" }</span>
                            </div>
                            <div class="docs-map-page">
                                <span class="l1"></span>
                                <span class="l2"></span>
                                <span class="l2 short"></span>
                                <span class="code-line-ph"></span>
                                <span class="l2"></span>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <section class="wrap section" aria-labelledby="docs-sections" style="padding-top: 0;">
                <h2 id="docs-sections" class="h3">{ t("docs.sections") }</h2>
                <div class="grid" style="--col: 260px;">
                    { out_card(GETTING_STARTED_URL, "book-open", "docs.aphyn.org", t("docs.guide.title"), t("docs.guide.body")) }
                    { out_card(API_REFERENCE_URL, "file-code", "docs.aphyn.org", t("docs.api.title"), t("docs.api.body")) }
                    { out_card(README_GUIDE_URL, "package", "github.com", t("docs.readme.title"), t("docs.readme.body")) }
                    <NavLink class="dest" to={Page::Examples.route(lang)}>
                        <span class="card-icon" aria-hidden="true"><Icon name="code" size={17} /></span>
                        <span class="dest-host">{ "aphyn.org" }</span>
                        <span class="dest-title">{ t("docs.examples.title") }</span>
                        <span class="dest-body">{ t("docs.examples.body") }</span>
                    </NavLink>
                </div>
            </section>

            <section class="band" aria-labelledby="docs-offline">
                <div class="wrap section split">
                    <div>
                        <div class="eyebrow">{ t("docs.offline.eyebrow") }</div>
                        <h2 id="docs-offline" class="h2 h2-sm">{ t("docs.offline.title") }</h2>
                        <p class="body">{ with_code(t("docs.offline.body.before"), "cargo doc", t("docs.offline.body.after")) }</p>
                        <div class="status-note">
                            <Badge tone={Tone::Warning} dot=true>{ t("docs.status.badge") }</Badge>
                            <p>{ t("docs.status.body") }</p>
                        </div>
                    </div>
                    <div class="code-col">
                        <CodeBlock code={t("docs.offline.code")} language={Language::Shell} filename="terminal" {lang} />
                        <p class="code-note">{ t("docs.offline.note") }</p>
                        <a class="more" href={REPO_URL}>
                            { t("home.hero.source") }
                            <Icon name="external-link" />
                        </a>
                    </div>
                </div>
            </section>
        </>
    }
}
