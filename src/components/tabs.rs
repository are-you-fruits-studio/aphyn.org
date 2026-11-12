use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TabsProps {
    pub items: Vec<&'static str>,
    pub selected: usize,
    pub on_select: Callback<usize>,
    /// Accessible name of the tab list.
    pub label: &'static str,
}

/// Underline tabs.
#[function_component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
    html! {
        <div class="tabs" role="tablist" aria-label={props.label}>
            { for props.items.iter().enumerate().map(|(i, item)| {
                let on_select = props.on_select.clone();
                html! {
                    <button
                        type="button"
                        role="tab"
                        aria-selected={(i == props.selected).to_string()}
                        onclick={Callback::from(move |_| on_select.emit(i))}
                    >
                        { *item }
                    </button>
                }
            }) }
        </div>
    }
}
