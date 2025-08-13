//! Typography scale & helpers inspired by Metro's crisp, content-first aesthetic.
use yew::prelude::*;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MetroTextStyle {
    Title,
    Subtitle,
    Base,
    Caption,
}

#[derive(Properties, PartialEq)]
pub struct TextProps {
    pub children: Children,
    #[prop_or(MetroTextStyle::Base)]
    pub variant: MetroTextStyle,
}

#[function_component(Text)]
pub fn text(props: &TextProps) -> Html {
    let class = match props.variant {
        MetroTextStyle::Title => "mu-text-title",
        MetroTextStyle::Subtitle => "mu-text-subtitle",
        MetroTextStyle::Base => "mu-text-base",
        MetroTextStyle::Caption => "mu-text-caption",
    };
    html! { <span class={classes!("mu-text", class)}>{ for props.children.iter() }</span> }
}
