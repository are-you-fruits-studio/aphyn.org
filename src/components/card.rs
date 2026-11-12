use yew::prelude::*;

use super::icon::Icon;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub title: &'static str,
    #[prop_or_default]
    pub icon: Option<&'static str>,
    #[prop_or_default]
    pub eyebrow: Option<&'static str>,
    /// Makes the whole card a link with a hover arrow.
    #[prop_or_default]
    pub href: Option<&'static str>,
    pub children: Html,
}

/// Content card: 1px border, 8px radius, no shadow.
#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let inner = html! {
        <>
            if let Some(icon) = props.icon {
                <span class="card-icon" aria-hidden="true"><Icon name={icon} size={17} /></span>
            }
            if let Some(eyebrow) = props.eyebrow {
                <span class="card-eyebrow">{ eyebrow }</span>
            }
            <h3 class="card-title">
                { props.title }
                if props.href.is_some() { <Icon name="arrow-right" size={15} /> }
            </h3>
            <div class="card-body">{ props.children.clone() }</div>
        </>
    };
    match props.href {
        Some(href) => html! { <a class="card" {href}>{ inner }</a> },
        None => html! { <div class="card">{ inner }</div> },
    }
}
