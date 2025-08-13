//! Layout primitives (panorama regions, grid, stacks).
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StackProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(8)]
    pub gap: u32,
    #[prop_or(false)]
    pub horizontal: bool,
}

#[function_component(Stack)]
pub fn stack(props: &StackProps) -> Html {
    let dir = if props.horizontal { "row" } else { "column" };
    let style = format!("display:flex;flex-direction:{dir};gap:{}px;", props.gap);
    html! { <div class={classes!("mu-stack")} {style}>{ for props.children.iter() }</div> }
}
