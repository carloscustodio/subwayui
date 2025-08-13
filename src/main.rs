use metro_ui::components::{AppBar, Tile};
use metro_ui::typography::{MetroTextStyle, Text};
use metro_ui::{metro_app, MetroProvider};
use yew::prelude::*;

#[function_component(Showcase)]
fn showcase() -> Html {
    html! { <div style="padding:32px;">
        <div class="section">
            <h2>{"Tiles"}</h2>
            <div class="showcase-grid" style="display:grid;grid-template-columns:repeat(auto-fill,minmax(180px,1fr));gap:24px;">
                <Tile title={"Weather".to_string()} tilt={true} tilt_debug={true}>
                    <Text variant={MetroTextStyle::Title}>{"72°"}</Text>
                    <Text variant={MetroTextStyle::Caption}>{"Partly Cloudy"}</Text>
                </Tile>
                <Tile title={"Music".to_string()} tilt={true} tilt_debug={true}>
                    <Text variant={MetroTextStyle::Subtitle}>{"Now Playing"}</Text>
                    <Text>{"Metro Theme Loop"}</Text>
                </Tile>
                <Tile title={"Calendar".to_string()} tilt={true} tilt_debug={true}>
                    <Text variant={MetroTextStyle::Subtitle}>{"Aug 12"}</Text>
                    <Text>{"3 meetings"}</Text>
                </Tile>
                <Tile title={"Mail".to_string()} tilt={true} tilt_debug={true}>
                    <Text variant={MetroTextStyle::Caption}>{"2 new messages"}</Text>
                </Tile>
                <Tile title={"News".to_string()} tilt={true} tilt_debug={true}>
                    <Text>{"Skeleton ready"}</Text>
                </Tile>
            </div>
        </div>
        <div class="section">
            <h2>{"Typography"}</h2>
            <Text variant={MetroTextStyle::Title}>{"Title Style"}</Text><br />
            <Text variant={MetroTextStyle::Subtitle}>{"Subtitle Style"}</Text><br />
            <Text>{"Base text sample showing body copy."}</Text><br />
            <Text variant={MetroTextStyle::Caption}>{"Caption text"}</Text>
        </div>
    </div> }
}

#[function_component(DemoApp)]
fn demo_app() -> Html {
    html! { <MetroProvider>
        <AppBar title={"Metro UI Showcase".to_string()}>
            <span>{"Dev Build"}</span>
        </AppBar>
        <Showcase />
    </MetroProvider> }
}

fn main() {
    // Initialize WASM logger once at startup so `log` macros output to browser console.
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    metro_app!(DemoApp);
}
