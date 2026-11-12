use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LogoProps {
    /// Mark height in px.
    #[prop_or(24)]
    pub size: u32,
}

/// The Aphyn lockup: a bilberry whose three-arm calyx cut-out doubles as the
/// X/Y/Z axes, next to the lowercase wordmark. Geometry from the design system.
#[function_component(Logo)]
pub fn logo(props: &LogoProps) -> Html {
    let size = props.size;
    let mark = Html::from_html_unchecked(AttrValue::from(mark_svg(size)));
    let gap = (size as f64 * 0.28).round();
    let word = (size as f64 * 0.92).round();
    html! {
        <span class="logo" role="img" aria-label="Aphyn" style={format!("gap:{gap}px")}>
            { mark }
            <span class="wordmark" aria-hidden="true" style={format!("font-size:{word}px;padding-bottom:{}px", size as f64 * 0.06)}>{ "aphyn" }</span>
        </span>
    }
}

fn mark_svg(size: u32) -> String {
    if size <= 18 {
        // The 16px drawing, with heavier arms.
        format!(
            r##"<svg class="mark" width="{size}" height="{size}" viewBox="0 0 16 16" aria-hidden="true"><defs><mask id="aphyn-mark-{size}"><rect width="16" height="16" fill="#fff"/><path d="M10.4 5.4 10.82 3.04M10.4 5.4 12.24 6.94M10.4 5.4 8.14 6.22" stroke="#000" stroke-width="1.9" stroke-linecap="round" fill="none"/></mask></defs><circle class="fill" cx="8" cy="8" r="7.5" mask="url(#aphyn-mark-{size})"/></svg>"##
        )
    } else {
        format!(
            r##"<svg class="mark" width="{size}" height="{size}" viewBox="0 0 32 32" aria-hidden="true"><defs><mask id="aphyn-mark-{size}"><rect width="32" height="32" fill="#fff"/><path d="M20.5 11 21.19 7.06M20.5 11 23.56 13.57M20.5 11 16.74 12.37" stroke="#000" stroke-width="2.8" stroke-linecap="round" fill="none"/></mask></defs><circle class="fill" cx="16" cy="16" r="14" mask="url(#aphyn-mark-{size})"/></svg>"##
        )
    }
}
