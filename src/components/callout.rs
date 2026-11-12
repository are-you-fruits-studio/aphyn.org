use yew::prelude::*;

use super::icon::Icon;

#[derive(Properties, PartialEq)]
pub struct CalloutProps {
    pub title: &'static str,
    pub children: Html,
}

/// A warning aside: soft tinted fill, no side stripe.
#[function_component(Callout)]
pub fn callout(props: &CalloutProps) -> Html {
    html! {
        <aside class="callout">
            <Icon name="triangle-alert" size={18} />
            <div>
                <strong>{ props.title }</strong>
                { props.children.clone() }
            </div>
        </aside>
    }
}
