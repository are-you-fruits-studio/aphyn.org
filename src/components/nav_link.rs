use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[derive(Properties, PartialEq)]
pub struct NavLinkProps {
    pub to: Route,
    #[prop_or_default]
    pub class: Classes,
    /// Marks the link as the current page.
    #[prop_or(false)]
    pub current: bool,
    #[prop_or_default]
    pub hreflang: Option<&'static str>,
    /// Runs after navigating, e.g. to close the phone menu.
    #[prop_or_default]
    pub onnavigate: Option<Callback<()>>,
    pub children: Html,
}

/// An in-site link: a real `<a href>` that navigates without reloading,
/// unless the visitor opens it in a new tab or window.
#[function_component(NavLink)]
pub fn nav_link(props: &NavLinkProps) -> Html {
    let navigator = use_navigator();
    let onclick = {
        let to = props.to.clone();
        let onnavigate = props.onnavigate.clone();
        Callback::from(move |e: MouseEvent| {
            if e.ctrl_key() || e.meta_key() || e.shift_key() || e.alt_key() || e.button() != 0 {
                return;
            }
            e.prevent_default();
            if let Some(navigator) = &navigator {
                navigator.push(&to);
            }
            if let Some(cb) = &onnavigate {
                cb.emit(());
            }
        })
    };
    html! {
        <a
            href={props.to.to_path()}
            class={props.class.clone()}
            aria-current={props.current.then_some("page")}
            hreflang={props.hreflang}
            {onclick}
        >
            { props.children.clone() }
        </a>
    }
}
