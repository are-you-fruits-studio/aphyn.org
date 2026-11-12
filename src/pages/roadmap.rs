use yew::prelude::*;

use super::{with_code, PageProps};
use crate::components::badge::{Badge, Tone};
use crate::components::callout::Callout;
use crate::components::icon::Icon;
use crate::components::nav_link::NavLink;
use crate::releases::{format_date, Releases};

/// How many releases the page lists.
const RECENT: usize = 4;

#[function_component(Roadmap)]
pub fn roadmap(props: &PageProps) -> Html {
    let lang = props.lang;
    let t = |key| lang.t(key);
    let releases = use_context::<Releases>().expect("releases context");

    let item = |icon: &'static str, class: &'static str, text: Html| {
        html! { <li><span class={class}><Icon name={icon} /></span><span>{ text }</span></li> }
    };
    let works = ["ecs", "render", "ui", "audio", "collision", "prefabs"];
    let growing = ["physics", "rendering", "reload"];
    let not_yet = [
        "animation",
        "shadows",
        "skybox",
        "ssr",
        "profiler",
        "docking",
        "platforms",
    ];

    html! {
        <>
            <section class="wrap" style="padding-block: clamp(48px, 11vw, 88px) clamp(40px, 7vw, 56px);">
                <div class="eyebrow">{ t("roadmap.eyebrow") }</div>
                <h1 class="h1 h1-page">{ t("roadmap.title") }</h1>
                <p class="lead" style="margin-bottom: 28px;">{ t("roadmap.lead") }</p>
                <Callout title={t("roadmap.callout.title")}>{ t("roadmap.callout.body") }</Callout>
            </section>

            <section class="wrap" style="padding-bottom: clamp(48px, 9vw, 72px);">
                <div class="grid" style="--col: 340px; align-items: start;">
                    <div class="panel">
                        <div class="panel-head"><Badge tone={Tone::Success} dot=true>{ t("status.works") }</Badge></div>
                        <ul class="list icons">
                            { for works.iter().map(|k| item("check", "ok", html! { lang.t(&format!("roadmap.works.{k}")) })) }
                            { item("check", "ok", with_code(t("roadmap.works.build"), "aphyn build", "")) }
                            { item("check", "ok", html! { t("roadmap.works.platforms") }) }
                        </ul>
                    </div>
                    <div class="panel">
                        <div class="panel-head"><Badge tone={Tone::Info} dot=true>{ t("status.growing") }</Badge></div>
                        <ul class="list icons">
                            { item("refresh-cw", "grow", html! { t("roadmap.growing.physics") }) }
                            { item("refresh-cw", "grow", with_code(t("roadmap.growing.editor.before"), "aphyn", t("roadmap.growing.editor.after"))) }
                            { for growing.iter().skip(1).map(|k| item("refresh-cw", "grow", html! { lang.t(&format!("roadmap.growing.{k}")) })) }
                        </ul>
                    </div>
                    <div class="panel">
                        <div class="panel-head"><Badge>{ t("status.not_yet") }</Badge></div>
                        <ul class="list icons muted">
                            { for not_yet.iter().map(|k| item("circle", "no", html! { lang.t(&format!("roadmap.not_yet.{k}")) })) }
                        </ul>
                        <p class="comment" style="margin-top: 20px;">{ t("roadmap.not_yet.comment") }</p>
                    </div>
                </div>
            </section>

            <section class="band" aria-labelledby="releases">
                <div class="wrap split" style="padding-block: clamp(48px, 9vw, 72px); gap: 48px;">
                    <div>
                        <h2 id="releases" class="h2 h2-sm" style="margin-bottom: 12px;">{ t("roadmap.releases.title") }</h2>
                        <p class="body" style="margin-bottom: 20px;">
                            { with_code(t("roadmap.releases.body.before"), "todos.md", t("roadmap.releases.body.after")) }
                        </p>
                        <NavLink class="more" to={crate::route::Page::Releases.route(lang)}>
                            { t("roadmap.releases.all") }
                            <Icon name="arrow-right" />
                        </NavLink>
                    </div>
                    if releases.list.is_empty() {
                        <p class="body">{ t("roadmap.releases.empty") }</p>
                    } else {
                        <ol class="releases highlight-first">
                            { for releases.list.iter().take(RECENT).map(|r| html! {
                                <li>
                                    <a class="ver" href={r.url.clone()}>{ r.tag.clone() }</a>
                                    <span class="what">
                                        { if r.name.is_empty() || r.name == r.tag { r.tag.clone() } else { r.name.clone() } }
                                        <span class="date">{ format_date(&r.date, lang) }</span>
                                    </span>
                                </li>
                            }) }
                        </ol>
                    }
                </div>
            </section>
        </>
    }
}
