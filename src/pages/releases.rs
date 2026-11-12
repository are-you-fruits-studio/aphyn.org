use yew::prelude::*;

use super::PageProps;
use crate::components::badge::{Badge, Tone};
use crate::components::callout::Callout;
use crate::components::code_block::{CodeBlock, Language};
use crate::components::header::RELEASES_URL;
use crate::components::icon::Icon;
use crate::releases::{format_date, Release, Releases as ReleaseData, BUILD_VERSION};

/// An on-site page for the engine's releases: the latest one with how to pin
/// it, then the earlier ones. Data comes from `/releases.json` (see releases.rs).
#[function_component(Releases)]
pub fn releases(props: &PageProps) -> Html {
    let lang = props.lang;
    let t = |key| lang.t(key);
    let data = use_context::<ReleaseData>().expect("releases context");
    let latest_tag = data.latest_tag();
    let dependency = format!(
        "[dependencies]\naphyn = {{ git = \"https://github.com/are-you-fruits-studio/aphyn\", tag = \"{latest_tag}\" }}"
    );

    let title_of = |r: &Release| {
        if r.name.is_empty() || r.name == r.tag {
            r.tag.clone()
        } else {
            r.name.clone()
        }
    };

    let latest = data.list.first().cloned();

    html! {
        <>
            <section class="page-hero">
                <div class="wrap">
                    <div class="eyebrow">{ t("releases.eyebrow") }</div>
                    <h1 class="h1 h1-page">{ t("releases.title") }</h1>
                    <p class="lead">{ t("releases.lead") }</p>
                </div>
            </section>

            <section class="wrap" aria-labelledby="latest-release" style="padding-bottom: clamp(40px, 8vw, 64px);">
                <div class="latest">
                    <div class="latest-main">
                        <Badge tone={Tone::Accent} dot=true>{ t("releases.latest") }</Badge>
                        <h2 id="latest-release" class="latest-tag">{ latest_tag.clone() }</h2>
                        if let Some(r) = &latest {
                            <p class="latest-name">{ title_of(r) }</p>
                            <p class="latest-date">{ format_date(&r.date, lang) }</p>
                        }
                        <div class="btn-row" style="margin-top: 24px;">
                            <a class="btn btn-primary" href={latest.as_ref().map(|r| r.url.clone()).unwrap_or_else(|| format!("{RELEASES_URL}/tag/{BUILD_VERSION}"))}>
                                { t("releases.notes") }
                                <Icon name="external-link" />
                            </a>
                        </div>
                        <p class="latest-hint">{ t("releases.download") }</p>
                    </div>
                    <div class="latest-code">
                        <CodeBlock code={dependency} language={Language::Toml} filename="Cargo.toml" {lang} />
                        <p class="code-note">{ t("releases.pin_note") }</p>
                    </div>
                </div>
            </section>

            <section class="wrap" aria-labelledby="earlier-releases" style="padding-bottom: clamp(56px, 12vw, 96px);">
                <div class="split" style="gap: 48px;">
                    <div>
                        <h2 id="earlier-releases" class="h3">{ t("releases.earlier") }</h2>
                        if data.list.len() > 1 {
                            <ol class="releases">
                                { for data.list.iter().skip(1).map(|r| html! {
                                    <li>
                                        <a class="ver" href={r.url.clone()}>{ r.tag.clone() }</a>
                                        <span class="what">
                                            { title_of(r) }
                                            <span class="date">{ format_date(&r.date, lang) }</span>
                                        </span>
                                    </li>
                                }) }
                            </ol>
                        } else {
                            <p class="body">{ t("roadmap.releases.empty") }</p>
                        }
                        <div style="margin-top: 20px;">
                            <a class="more" href={RELEASES_URL}>
                                { t("releases.all") }
                                <Icon name="external-link" />
                            </a>
                        </div>
                    </div>
                    <div>
                        <Callout title={t("roadmap.callout.title")}>{ t("releases.callout") }</Callout>
                    </div>
                </div>
            </section>
        </>
    }
}
