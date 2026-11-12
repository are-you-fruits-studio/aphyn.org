use yew::prelude::*;

use super::icon_data::svg_inner;

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub name: &'static str,
    #[prop_or(16)]
    pub size: u32,
}

/// A Lucide icon drawn in `currentColor`, hidden from screen readers.
#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let markup = format!(
        r#"<svg width="{s}" height="{s}" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" style="flex:none;display:block">{inner}</svg>"#,
        s = props.size,
        inner = svg_inner(props.name),
    );
    Html::from_html_unchecked(AttrValue::from(markup))
}
