use yew::prelude::*;

/// The large animated mark from the name section. The calyx cut-out grows out
/// of the berry in the colours of the X (rowan), Y (leaf) and Z (bloom) axes
/// with a white pivot, the colours fade to the plain cut-out, it holds, then
/// shrinks back. All of it is CSS (`.berry` in site.css); with reduced motion
/// it is the still logo.
#[function_component(Berry)]
pub fn berry() -> Html {
    Html::from_html_unchecked(AttrValue::from(
        r##"<svg class="berry" viewBox="0 0 32 32" role="img" aria-label="Aphyn"><defs><mask id="berry-calyx"><rect width="32" height="32" fill="#fff"/><g class="calyx-g"><path class="calyx-arm" d="M20.5 11 21.19 7.06"/><path class="calyx-arm" d="M20.5 11 23.56 13.57"/><path class="calyx-arm" d="M20.5 11 16.74 12.37"/></g></mask></defs><circle class="fill" cx="16" cy="16" r="14" mask="url(#berry-calyx)"/><g class="axis-g"><path class="axis-arm axis-y" d="M20.5 11 21.19 7.06"/><path class="axis-arm axis-x" d="M20.5 11 23.56 13.57"/><path class="axis-arm axis-z" d="M20.5 11 16.74 12.37"/><circle class="calyx-dot" cx="20.5" cy="11" r="1.05"/></g></svg>"##,
    ))
}
