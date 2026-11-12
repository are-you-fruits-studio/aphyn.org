use yew::prelude::*;

use super::{with_code, PageProps};
use crate::components::badge::{Badge, Tone};
use crate::components::code_block::{CodeBlock, Language};
use crate::components::icon::Icon;

const EXAMPLES_URL: &str =
    "https://github.com/are-you-fruits-studio/aphyn/tree/dev/crates/example/";

/// Every example crate in the engine repository: name, icon, and whether it is new.
const EXAMPLES: &[(&str, &str, bool)] = &[
    ("cubes", "box", false),
    ("ecs", "cpu", false),
    ("boids", "move-3d", false),
    ("physics", "move", true),
    ("collisions", "mouse-pointer-2", false),
    ("gizmos", "rotate-3d", false),
    ("material", "layers", true),
    ("normals", "sun", false),
    ("bloom", "sparkle", false),
    ("text", "type", false),
    ("ui", "panel-left", false),
    ("audio_screen_saver", "music", false),
    ("showcase", "star", false),
];

#[function_component(Examples)]
pub fn examples(props: &PageProps) -> Html {
    let lang = props.lang;
    let t = |key| lang.t(key);

    html! {
        <>
            <section class="wrap" style="padding-block: clamp(48px, 11vw, 88px) clamp(32px, 6vw, 48px);">
                <div class="eyebrow">{ t("examples.eyebrow") }</div>
                <h1 class="h1 h1-page">{ t("examples.title") }</h1>
                <p class="lead">{ with_code(t("examples.lead.before"), "crates/example/", t("examples.lead.after")) }</p>
                <div style="max-width: 720px;">
                    <CodeBlock code={t("examples.run")} language={Language::Shell} filename="terminal" {lang} />
                </div>
            </section>

            <section class="wrap" style="padding-bottom: clamp(40px, 8vw, 64px);" aria-labelledby="start-here">
                <div class="featured">
                    <img src="/img/showcase.gif" width="800" height="450" alt={t("home.showcase.alt")} />
                    <div class="featured-text">
                        <Badge tone={Tone::Accent} mono=true>{ "showcase" }</Badge>
                        <h2 id="start-here">{ t("examples.featured.title") }</h2>
                        <p>{ t("examples.featured.body") }</p>
                        <a class="more" href={format!("{EXAMPLES_URL}showcase")}>
                            { t("examples.featured.more") }
                            <Icon name="arrow-right" />
                        </a>
                    </div>
                </div>
            </section>

            <section class="wrap" style="padding-bottom: clamp(56px, 12vw, 96px);" aria-labelledby="all-examples">
                <h2 id="all-examples" class="h3">{ t("examples.all") }</h2>
                <div class="grid" style="--col: 340px;">
                    { for EXAMPLES.iter().map(|&(name, icon, new)| html! {
                        <a class="ex-card" href={format!("{EXAMPLES_URL}{name}")}>
                            <div class="ex-head">
                                <div class="ex-name">
                                    <Icon name={icon} size={18} />
                                    <span>{ name }</span>
                                </div>
                                if new { <Badge tone={Tone::Highlight} mono=true>{ t("common.new") }</Badge> }
                            </div>
                            <p>{ lang.t(&format!("example.{name}")) }</p>
                            <code>{ format!("-p aphyn_example_{name}") }</code>
                        </a>
                    }) }
                </div>
            </section>
        </>
    }
}
