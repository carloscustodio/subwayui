//! Metro-inspired UI component library for Yew.
//!
//! Goals:
//! * Bold typography, panoramic layouts, immersive content-first blocks.
//! * Motion as meaning: fast, clean transitions (optional `animations` feature).
//! * Theming via accent color + light/dark and high contrast modes.
//! * Accessible by default.
//!
//! This crate currently provides a minimal skeleton; components will expand over time.

pub mod components;
pub mod icons;
pub mod layout;
pub mod theme;
pub mod typography;
pub mod utils;

#[cfg(feature = "static-css")]
use yew::platform::spawn_local;
#[cfg(feature = "static-css")]
use yew::prelude::*;

/// Call during app startup to inject the base stylesheet (if `static-css` feature enabled).
#[cfg(feature = "static-css")]
pub fn inject_global_styles() {
    spawn_local(async {
        let style = include_str!("styles/subway.css");
        let document = gloo_utils::document();
        let head = document.head().expect("document should have a head");
        let style_el = document
            .create_element("style")
            .expect("create style element");
        style_el.set_attribute("data-metro-ui", "base").ok();
        style_el.set_text_content(Some(style));
        head.append_child(&style_el).ok();
    });
}

/// Root Metro provider (placeholder) – will later manage theming, typography scale, etc.
#[derive(Properties, PartialEq)]
pub struct MetroProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(MetroProvider)]
pub fn metro_provider(props: &MetroProviderProps) -> Html {
    html! { <div data-metro-ui-root="true">{ for props.children.iter() }</div> }
}

/// Convenience macro to start a Metro-styled app.
#[macro_export]
macro_rules! metro_app {
    ($root:ty) => {{
        #[cfg(feature = "static-css")]
        {
            $crate::inject_global_styles();
        }
        yew::Renderer::<$root>::new().render();
    }};
}
