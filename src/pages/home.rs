use yew::prelude::*;

use super::{with_code, PageProps};
use crate::components::badge::{Badge, Tone};
use crate::components::berry::Berry;
use crate::components::card::Card;
use crate::components::code_block::{CodeBlock, Language};
use crate::components::header::{DOCS_URL, REPO_URL};
use crate::components::icon::Icon;
use crate::components::nav_link::NavLink;
use crate::components::tabs::Tabs;
use crate::releases::Releases;
use crate::route::Page;

const ISSUES_URL: &str = "https://github.com/are-you-fruits-studio/aphyn/issues";

#[function_component(Home)]
pub fn home(props: &PageProps) -> Html {
    let lang = props.lang;
    let t = |key| lang.t(key);
    let releases = use_context::<Releases>().expect("releases context");
    let version = releases.latest_tag();

    html! {
        <>
            <section class="hero">
                <div class="wrap">
                    <div class="badges">
                        <Badge tone={Tone::Warning} dot=true>{ t("home.badge.early") }</Badge>
                        <Badge tone={Tone::Accent} mono=true>{ format!("{version} · {}", t("home.badge.latest")) }</Badge>
                    </div>
                    <h1 class="h1 h1-hero">{ t("home.hero.title") }</h1>
                    <p class="lead">{ t("home.hero.lead") }</p>
                    <div class="btn-row">
                        <a class="btn btn-primary" href="#quick-start">
                            { t("home.hero.cta") }
                            <Icon name="arrow-right" />
                        </a>
                        <a class="btn btn-secondary" href={REPO_URL}>
                            <Icon name="github" />
                            { t("home.hero.source") }
                        </a>
                    </div>
                    <div class="meta-row">
                        <span>{ "Windows" }</span>
                        <span>{ "Linux" }</span>
                        <span>{ "x86_64" }</span>
                        <span>{ "MIT OR Apache-2.0" }</span>
                        <span>{ t("home.meta.since") }</span>
                    </div>
                </div>
            </section>

            <div class="wrap showcase">
                <figure>
                    <img src="/img/showcase.gif" width="800" height="450" alt={t("home.showcase.alt")} />
                    <figcaption>
                        <span>{ t("home.showcase.caption") }</span>
                        <code>{ "cargo run --release -p aphyn_example_showcase" }</code>
                    </figcaption>
                </figure>
            </div>

            <section class="band" aria-labelledby="why">
                <div class="wrap section">
                    <div class="eyebrow">{ t("home.why.eyebrow") }</div>
                    <h2 id="why" class="h2" style="max-width: 760px; margin-bottom: 48px;">{ t("home.why.title") }</h2>
                    <div class="grid" style="--col: 260px;">
                        <Card icon="boxes" title={t("home.why.crates.title")}>{ t("home.why.crates.body") }</Card>
                        <Card icon="cpu" title={t("home.why.data.title")}>{ t("home.why.data.body") }</Card>
                        <Card icon="leaf" title={t("home.why.grown.title")}>{ t("home.why.grown.body") }</Card>
                        <Card icon="lock-open" title={t("home.why.license.title")}>{ t("home.why.license.body") }</Card>
                    </div>
                </div>
            </section>

            <section class="wrap section" aria-labelledby="features">
                <div class="split" style="gap: 48px 64px;">
                    <div>
                        <div class="eyebrow">{ t("home.features.eyebrow") }</div>
                        <h2 id="features" class="h2">{ t("home.features.title") }</h2>
                        <p class="body" style="margin-bottom: 24px;">{ t("home.features.body") }</p>
                        <NavLink class="more" to={Page::Examples.route(lang)}>
                            { t("home.features.more") }
                            <Icon name="arrow-right" />
                        </NavLink>
                    </div>
                    <dl class="features">
                        { feature("cpu", t("feature.ecs.title"), t("feature.ecs.body"), false, lang) }
                        { feature("layers", t("feature.render.title"), t("feature.render.body"), false, lang) }
                        { feature("type", t("feature.ui.title"), t("feature.ui.body"), false, lang) }
                        { feature("music", t("feature.audio.title"), t("feature.audio.body"), false, lang) }
                        { feature("box", t("feature.collision.title"), t("feature.collision.body"), false, lang) }
                        { feature("file-code", t("feature.prefabs.title"), t("feature.prefabs.body"), false, lang) }
                        { feature("package", t("feature.build.title"), t("feature.build.body"), false, lang) }
                        { feature("move-3d", t("feature.physics.title"), t("feature.physics.body"), true, lang) }
                    </dl>
                </div>
            </section>

            <section id="quick-start" class="band" aria-labelledby="quick-start-title">
                <div class="wrap section split">
                    <div>
                        <div class="eyebrow">{ t("home.quick.eyebrow") }</div>
                        <h2 id="quick-start-title" class="h2">{ t("home.quick.title") }</h2>
                        <p class="body">{ with_code(t("home.quick.need.before"), "rustup update", t("home.quick.need.after")) }</p>
                        <p class="body" style="margin-bottom: 28px;">{ t("home.quick.pin") }</p>
                        <ol class="steps">
                            <li><span class="num">{ "01" }</span><span>{ t("home.quick.step1") }</span></li>
                            <li><span class="num">{ "02" }</span><span>{ with_code(t("home.quick.step2"), "scripts/", "") }</span></li>
                            <li><span class="num">{ "03" }</span><span>{ with_code(t("home.quick.step3"), "aphyn build", "") }</span></li>
                        </ol>
                        <NavLink class="more" to={Page::Docs.route(lang)}>
                            { t("home.quick.more") }
                            <Icon name="arrow-right" />
                        </NavLink>
                    </div>
                    <QuickStartCode lang={lang} version={version.clone()} />
                </div>
            </section>

            <section class="wrap section" aria-labelledby="status">
                <div class="eyebrow">{ t("home.status.eyebrow") }</div>
                <h2 id="status" class="h2" style="max-width: 760px; margin-bottom: 16px;">{ t("home.status.title") }</h2>
                <p class="body" style="margin-bottom: 40px;">{ t("home.status.body") }</p>
                <div class="grid" style="--col: 340px;">
                    <div class="panel">
                        <div class="panel-head"><Badge tone={Tone::Info} dot=true>{ t("status.growing") }</Badge></div>
                        <ul class="list">
                            <li>{ with_code(t("home.status.editor.before"), "aphyn", t("home.status.editor.after")) }</li>
                            <li>{ t("home.status.physics") }</li>
                        </ul>
                    </div>
                    <div class="panel">
                        <div class="panel-head"><Badge>{ t("status.not_yet") }</Badge></div>
                        <p class="dots">{ t("home.status.not_yet") }</p>
                        <p class="comment">{ t("home.status.comment") }</p>
                    </div>
                </div>
                <div style="margin-top: 24px;">
                    <NavLink class="more" to={Page::Roadmap.route(lang)}>
                        { t("home.status.more") }
                        <Icon name="arrow-right" />
                    </NavLink>
                </div>
            </section>

            <section class="band" aria-labelledby="name">
                <div class="wrap section split center">
                    <div>
                        <div class="word" lang="uk">{ t("home.name.word") }</div>
                        <p id="name" class="name-lede">{ t("home.name.lede") }</p>
                        <p class="name-note">{ t("home.name.note") }</p>
                    </div>
                    <div class="berry-wrap"><Berry /></div>
                </div>
            </section>

            <section class="wrap section" aria-labelledby="follow">
                <h2 id="follow" class="h2 h2-sm" style="margin-bottom: 12px;">{ t("home.follow.title") }</h2>
                <p class="body" style="margin-bottom: 32px;">{ t("home.follow.body") }</p>
                <div class="grid" style="--col: 300px;">
                    <Card href={REPO_URL} icon="github" eyebrow="GitHub" title={t("home.follow.code.title")}>{ t("home.follow.code.body") }</Card>
                    <Card href={ISSUES_URL} icon="bug" eyebrow={t("home.follow.issues.eyebrow")} title={t("home.follow.issues.title")}>{ t("home.follow.issues.body") }</Card>
                    <Card href={DOCS_URL} icon="book-open" eyebrow="docs.aphyn.org" title={t("home.follow.docs.title")}>{ t("home.follow.docs.body") }</Card>
                </div>
            </section>
        </>
    }
}

fn feature(
    icon: &'static str,
    title: &'static str,
    body: &'static str,
    new: bool,
    lang: crate::i18n::Lang,
) -> Html {
    html! {
        <div class="feature">
            <Icon name={icon} size={20} />
            <div>
                <dt>
                    { title }
                    if new { { " " }<span class="tag-new">{ lang.t("common.new") }</span> }
                </dt>
                <dd>{ body }</dd>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct QuickStartProps {
    lang: crate::i18n::Lang,
    version: String,
}

#[function_component(QuickStartCode)]
fn quick_start_code(props: &QuickStartProps) -> Html {
    let lang = props.lang;
    let selected = use_state(|| 0usize);
    let tabs = vec![
        lang.t("code.tab.run"),
        "main.rs",
        "Cargo.toml",
        "aphyn build",
    ];

    let (filename, language, code, note): (&str, Language, String, &str) = match *selected {
        1 => (
            "src/main.rs",
            Language::Rust,
            MAIN_RS.to_owned(),
            lang.t("code.main.note"),
        ),
        2 => (
            "scripts/Cargo.toml",
            Language::Toml,
            CARGO_TOML.replace("{version}", &props.version),
            lang.t("code.cargo.note"),
        ),
        3 => (
            "terminal",
            Language::Shell,
            lang.t("code.build.body").to_owned(),
            lang.t("code.build.note"),
        ),
        _ => (
            "terminal",
            Language::Shell,
            lang.t("code.run.body").to_owned(),
            lang.t("code.run.note"),
        ),
    };

    let on_select = {
        let selected = selected.clone();
        Callback::from(move |i| selected.set(i))
    };

    html! {
        <div class="code-col">
            <Tabs items={tabs} selected={*selected} {on_select} label={lang.t("code.tabs_label")} />
            <CodeBlock code={code} {language} filename={filename.to_owned()} {lang} line_numbers=true />
            <p class="code-note">{ note }</p>
        </div>
    }
}

const MAIN_RS: &str = "use aphyn::*;

fn main() {
    let mut app = App::new();

    let AppInitCtx { mut data, mut behavior, render, .. } = app.init_ctx_mut();
    add_defult_modules_to(data.as_mut(), behavior.as_mut(), render);

    let mut behavior = behavior.as_mut();
    behavior.get_mut(Schedule::Start).insert_system(setup_scene);
    behavior.get_mut(Schedule::Update).insert_system(spin);

    app.run();
}";

const CARGO_TOML: &str = r#"[package]
name = "lib_app"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[[bin]]
name = "launcher"
path = "src/main.rs"

[dependencies]
aphyn = { git = "https://github.com/are-you-fruits-studio/aphyn", tag = "{version}" }"#;
