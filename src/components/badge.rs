use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Tone {
    Neutral,
    Accent,
    Success,
    Warning,
    Info,
    Highlight,
}

impl Tone {
    fn class(self) -> &'static str {
        match self {
            Tone::Neutral => "neutral",
            Tone::Accent => "accent",
            Tone::Success => "success",
            Tone::Warning => "warning",
            Tone::Info => "info",
            Tone::Highlight => "highlight",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    #[prop_or(Tone::Neutral)]
    pub tone: Tone,
    #[prop_or(false)]
    pub dot: bool,
    #[prop_or(false)]
    pub mono: bool,
    pub children: Html,
}

/// Small status label — state, versions, "new".
#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    let class = classes!("badge", props.tone.class(), props.mono.then_some("mono"));
    html! {
        <span {class}>
            if props.dot { <span class="dot" aria-hidden="true"></span> }
            { props.children.clone() }
        </span>
    }
}
