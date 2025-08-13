//! Theme system: accent colors, dark/light, contrast.
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct MetroTheme {
    pub accent: String,
    pub dark: bool,
    pub high_contrast: bool,
}

impl Default for MetroTheme {
    fn default() -> Self {
        Self {
            accent: "#008eab".into(),
            dark: false,
            high_contrast: false,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ThemeProviderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub value: Option<MetroTheme>,
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    let theme = use_state(|| props.value.clone().unwrap_or_default());
    let current = (*theme).clone();
    html! {<ContextProvider<MetroTheme> context={current}>{ for props.children.iter() }</ContextProvider<MetroTheme>>}
}

// NOTE: Access the theme inside components directly with `use_context::<MetroTheme>()`.
