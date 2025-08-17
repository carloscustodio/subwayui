use subwayui::components::{AppBar, Tile};
use subwayui::typography::{MetroTextStyle, Text};
use subwayui::MetroProvider;
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[function_component(Showcase)]
fn showcase() -> Html {
    html! {
        <div class="showcase-container">
            <section class="tiles-section">
                <h2>{"Tiles"}</h2>
                <div class="tiles-grid">
                    <Tile title="Weather" tilt=true>
                        <Text variant={MetroTextStyle::Title}>{"72°"}</Text>
                        <Text variant={MetroTextStyle::Caption}>{"Partly Cloudy"}</Text>
                    </Tile>
                    <Tile title="Music" tilt=true>
                        <Text variant={MetroTextStyle::Subtitle}>{"Now Playing"}</Text>
                        <Text>{"Metro Theme Loop"}</Text>
                    </Tile>
                    <Tile title="Calendar" tilt=true>
                        <Text variant={MetroTextStyle::Subtitle}>{"Aug 12"}</Text>
                        <Text>{"3 meetings"}</Text>
                    </Tile>
                    <Tile title="Mail" tilt=true>
                        <Text variant={MetroTextStyle::Caption}>{"2 new messages"}</Text>
                    </Tile>
                    <Tile title="News" tilt=true>
                        <Text>{"Skeleton ready"}</Text>
                    </Tile>
                </div>
            </section>
            <section class="typography-section">
                <h2>{"Typography"}</h2>
                <div class="typography-samples">
                    <Text variant={MetroTextStyle::Title}>{"Title Style"}</Text>
                    <Text variant={MetroTextStyle::Subtitle}>{"Subtitle Style"}</Text>
                    <Text>{"Base text sample showing body copy."}</Text>
                    <Text variant={MetroTextStyle::Caption}>{"Caption text"}</Text>
                </div>
            </section>
        </div>
    }
}

#[function_component(DemoApp)]
fn demo_app() -> Html {
    html! {
        <MetroProvider>
            <AppBar title="Metro UI Showcase">
                <span>{"Dev Build"}</span>
            </AppBar>
            <main class="app-content">
                <Showcase />
            </main>
        </MetroProvider>
    }
}

// Initialize logger with proper platform detection
fn init_logging() {
    #[cfg(target_arch = "wasm32")]
    {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
        log::info!("WASM logger initialized");
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        log::info!("Native logger initialized");
    }
}

// WASM entry point for Tauri's webview
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start_app() {
    init_logging();

    #[cfg(feature = "static-css")]
    {
        subwayui::inject_global_styles();
    }

    yew::Renderer::<DemoApp>::new().render();
}

// Native entry point (for development/testing)
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    init_logging();
    log::info!("Application starting in native mode");
    // In Tauri, this would be called by the webview context
}

// WASM main function
#[cfg(target_arch = "wasm32")]
fn main() {
    // This is handled by start_app() in WASM builds
}
