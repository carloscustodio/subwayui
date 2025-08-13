//! Application bar (top command surface)
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AppBarProps {
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AppBar)]
pub fn app_bar(props: &AppBarProps) -> Html {
    html! { <header class={classes!("mu-appbar")}>{
        if let Some(t) = &props.title { html!(<span class="mu-appbar-title">{t}</span>) } else { Html::default() }
    }<nav class="mu-appbar-actions">{ for props.children.iter() }</nav></header> }
}
