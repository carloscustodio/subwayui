//! Icon placeholders (could be SVG inline assets later).
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub name: String,
    #[prop_or(16)]
    pub size: u32,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let style = format!(
        "width:{}px;height:{}px;display:inline-block;background:currentColor;mask:unset;",
        props.size, props.size
    );
    html! { <span class={classes!("mu-icon", format!("mu-icon-{}", props.name))} {style}></span> }
}
