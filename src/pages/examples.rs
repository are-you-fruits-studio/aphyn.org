use yew::prelude::*;

use super::PageProps;
use crate::components::badge::{Badge, Tone};
use crate::components::code_block::{CodeBlock, Language};
use crate::components::icon::Icon;

/// The repository that holds the examples.
pub const EXAMPLES_REPO_URL: &str = "https://github.com/are-you-fruits-studio/aphyn.examples";
const EXAMPLES_URL: &str =
    "https://github.com/are-you-fruits-studio/aphyn.examples/tree/main/examples/";
const ASSETS_URL: &str =
    "https://github.com/are-you-fruits-studio/aphyn.examples/blob/main/ASSETS.md";

/// The `[lib]`, `[[bin]]` and engine dependency every example's `scripts/Cargo.toml` shares.
const CARGO_TOML: &str = r#"[lib]
name = "aphynapp"
crate-type = ["cdylib", "lib"]

[[bin]]
name = "launcher"
path = "src/main.rs"

[dependencies]
aphyn = { git = "https://github.com/are-you-fruits-studio/aphyn", tag = "v0.23.0" }"#;

/// Topics for the filter, in the order the chips appear.
#[derive(Clone, Copy, PartialEq)]
enum Topic {
    Basics,
    Rendering,
    Input,
    Physics,
    Ui,
    Audio,
}

impl Topic {
    const ALL: [Topic; 6] = [
        Topic::Basics,
        Topic::Rendering,
        Topic::Input,
        Topic::Physics,
        Topic::Ui,
        Topic::Audio,
    ];

    fn key(self) -> &'static str {
        match self {
            Topic::Basics => "examples.topic.basics",
            Topic::Rendering => "examples.topic.rendering",
            Topic::Input => "examples.topic.input",
            Topic::Physics => "examples.topic.physics",
            Topic::Ui => "examples.topic.ui",
            Topic::Audio => "examples.topic.audio",
        }
    }
}

/// How an example is driven: the locale key of the label and its icon.
type Controls = Option<(&'static str, &'static str)>;

const MOUSE: Controls = Some(("examples.controls.mouse", "mouse-pointer-2"));

/// One example in `examples/<name>/` of the examples repository.
struct Example {
    name: &'static str,
    icon: &'static str,
    topic: Topic,
    controls: Controls,
}

const fn ex(name: &'static str, icon: &'static str, topic: Topic, controls: Controls) -> Example {
    Example {
        name,
        icon,
        topic,
        controls,
    }
}

/// Every example, in the order of the repository's README.
const EXAMPLES: &[Example] = &[
    ex("cubes", "box", Topic::Basics, None),
    ex(
        "ecs",
        "cpu",
        Topic::Basics,
        Some(("examples.controls.console", "terminal")),
    ),
    ex("boids", "move-3d", Topic::Basics, None),
    ex(
        "gizmos",
        "rotate-3d",
        Topic::Rendering,
        Some(("examples.controls.fly", "keyboard")),
    ),
    ex("collisions", "grid-3x3", Topic::Input, MOUSE),
    ex("text", "type", Topic::Ui, None),
    ex("ui", "panel-left", Topic::Ui, MOUSE),
    ex("material", "layers", Topic::Rendering, None),
    ex(
        "physics",
        "move",
        Topic::Physics,
        Some(("examples.controls.physics", "hand")),
    ),
    ex("audio_screen_saver", "music", Topic::Audio, None),
    ex("normals", "sun", Topic::Rendering, MOUSE),
    ex("bloom", "sparkle", Topic::Rendering, None),
    ex(
        "showcase",
        "star",
        Topic::Rendering,
        Some(("examples.controls.capture", "image")),
    ),
    ex(
        "audio_sfx",
        "keyboard",
        Topic::Audio,
        Some(("examples.controls.digits", "keyboard")),
    ),
    ex("mouse_input", "mouse-pointer-2", Topic::Input, MOUSE),
    ex(
        "http_control",
        "globe",
        Topic::Input,
        Some(("examples.controls.http", "globe")),
    ),
    ex(
        "babble",
        "message-circle",
        Topic::Audio,
        Some(("examples.controls.type", "keyboard")),
    ),
    ex(
        "custom_shaders",
        "file-code",
        Topic::Rendering,
        Some(("examples.controls.shaders", "keyboard")),
    ),
    ex(
        "voxel_house",
        "boxes",
        Topic::Physics,
        Some(("examples.controls.voxel", "keyboard")),
    ),
];

#[function_component(Examples)]
pub fn examples(props: &PageProps) -> Html {
    let lang = props.lang;
    let t = |key| lang.t(key);
    // `None` shows every example.
    let topic = use_state(|| None::<Topic>);

    let chip = |value: Option<Topic>, label: &'static str, count: usize| {
        let pressed = *topic == value;
        let onclick = {
            let topic = topic.clone();
            Callback::from(move |_| topic.set(value))
        };
        html! {
            <button type="button" class="chip" aria-pressed={pressed.to_string()} {onclick}>
                { label }
                <span class="chip-count">{ count }</span>
            </button>
        }
    };

    let shown = EXAMPLES
        .iter()
        .filter(|e| topic.is_none_or(|t| e.topic == t));

    html! {
        <>
            <section class="page-hero" style="padding-bottom: clamp(32px, 6vw, 48px);">
                <div class="wrap split center">
                    <div>
                        <div class="eyebrow">{ t("examples.eyebrow") }</div>
                        <h1 class="h1 h1-page">{ t("examples.title") }</h1>
                        <p class="lead">
                            { t("examples.lead.before") }{ " " }
                            <a class="inline-code" href={EXAMPLES_REPO_URL}>{ "aphyn.examples" }</a>
                            { t("examples.lead.after") }
                        </p>
                        <a class="btn btn-primary" href={EXAMPLES_REPO_URL}>
                            <Icon name="github" />
                            { t("examples.open_repo") }
                        </a>
                        <div class="meta-row" style="margin-top: 24px;">
                            <span>{ "MIT OR Apache-2.0" }</span>
                            <span>{ t("examples.fact.tag") }</span>
                            <span>{ t("examples.fact.ci") }</span>
                            <span>{ t("examples.fact.nightly") }</span>
                        </div>
                    </div>
                    <div class="code-col">
                        <CodeBlock code={t("examples.run")} language={Language::Shell} filename="terminal" {lang} />
                        <p class="code-note" style="overflow-wrap: anywhere;">{ t("examples.run_note") }</p>
                    </div>
                </div>
            </section>

            <section class="wrap" style="padding-bottom: clamp(40px, 8vw, 64px);" aria-labelledby="start-here">
                <div class="featured">
                    <img src="/img/showcase.gif" width="800" height="450" alt={t("home.showcase.alt")} />
                    <div class="featured-text">
                        <Badge tone={Tone::Accent} mono=true>{ "showcase" }</Badge>
                        <h2 id="start-here">{ t("examples.featured.title") }</h2>
                        <p>{ t("examples.featured.body") }</p>
                        <p class="featured-note">{ t("examples.featured.capture") }</p>
                        <a class="more" href={format!("{EXAMPLES_URL}showcase")}>
                            { t("examples.featured.more") }
                            <Icon name="arrow-right" />
                        </a>
                    </div>
                </div>
            </section>

            <section class="wrap" style="padding-bottom: clamp(56px, 12vw, 96px);" aria-labelledby="all-examples">
                <div class="ex-toolbar">
                    <h2 id="all-examples" class="h3">{ t("examples.all") }</h2>
                    <div class="chips" role="group" aria-label={t("examples.filter_label")}>
                        { chip(None, t("examples.topic.all"), EXAMPLES.len()) }
                        { for Topic::ALL.iter().map(|&k| {
                            chip(Some(k), t(k.key()), EXAMPLES.iter().filter(|e| e.topic == k).count())
                        }) }
                    </div>
                </div>
                <div class="grid" style="--col: 340px;">
                    { for shown.map(|e| html! {
                        <a class="ex-card" href={format!("{EXAMPLES_URL}{}", e.name)}>
                            <div class="ex-head">
                                <div class="ex-name">
                                    <Icon name={e.icon} size={18} />
                                    <span>{ e.name }</span>
                                </div>
                            </div>
                            <p>{ lang.t(&format!("example.{}", e.name)) }</p>
                            <div class="ex-foot">
                                <code>{ format!("just dev {}", e.name) }</code>
                                if let Some((label, icon)) = e.controls {
                                    <span class="ex-controls">
                                        <Icon name={icon} size={12} />
                                        { t(label) }
                                    </span>
                                }
                            </div>
                        </a>
                    }) }
                </div>
            </section>

            <section class="band" aria-labelledby="one-project" style="border-bottom: 0;">
                <div class="wrap section split">
                    <div>
                        <div class="eyebrow">{ t("examples.project.eyebrow") }</div>
                        <h2 id="one-project" class="h2 h2-sm">{ t("examples.project.title") }</h2>
                        <p class="body">{ t("examples.project.crate") }</p>
                        <p class="body">{ t("examples.project.build") }</p>
                        <p class="body body-sm">
                            { t("examples.project.assets") }{ " " }
                            <a class="inline-code" href={ASSETS_URL}>{ "ASSETS.md" }</a>
                            { "." }
                        </p>
                    </div>
                    <div class="code-col code-stack">
                        <CodeBlock code={t("examples.project.tree")} language={Language::Shell} filename="examples/cubes" {lang} />
                        <CodeBlock code={CARGO_TOML} language={Language::Toml} filename="scripts/Cargo.toml" {lang} />
                    </div>
                </div>
            </section>
        </>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::LOCALES;

    /// Every locale must hold the text the page asks for; the i18n test only
    /// compares locales with each other, so a key missing from all of them
    /// would slip through.
    #[test]
    fn every_example_and_control_has_text() {
        let has = |source: &str, key: &str| {
            source
                .lines()
                .any(|l| l.split_once('=').is_some_and(|(k, _)| k.trim() == key))
        };
        let mut keys: Vec<String> = Vec::new();
        for e in EXAMPLES {
            keys.push(format!("example.{}", e.name));
            if let Some((label, _)) = e.controls {
                keys.push(label.to_owned());
            }
        }
        keys.extend(Topic::ALL.iter().map(|t| t.key().to_owned()));
        for locale in LOCALES {
            for key in &keys {
                assert!(has(locale.source, key), "{} misses {key}", locale.code);
            }
        }
    }
}
