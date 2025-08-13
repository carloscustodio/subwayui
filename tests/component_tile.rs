//! WASM tests for the Tile component.
use metro_ui::components::Tile;
use metro_ui::typography::{MetroTextStyle, Text};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::Renderer;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[function_component(TestHarness)]
fn test_harness() -> Html {
    html! { <Tile title={"Test".to_string()}><Text variant={MetroTextStyle::Caption}>{"Inner"}</Text></Tile> }
}

#[wasm_bindgen_test]
fn tile_renders() {
    let renderer = Renderer::<TestHarness>::new();
    renderer.render();
    // Basic smoke: ensure a tile element is present in DOM
    let document = web_sys::window().unwrap().document().unwrap();
    let el = document.query_selector(".mu-tile").unwrap();
    assert!(el.is_some(), "Tile root should render");
}
