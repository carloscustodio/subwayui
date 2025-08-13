//! Interactive tile component with optional 3D tilt effect.
use web_sys::Element;
use yew::prelude::*;

/// Maximum degrees of rotation (default) when `tilt` prop enabled.
const DEFAULT_TILT_MAX: f32 = 18.0;

#[derive(Properties, PartialEq)]
pub struct TileProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub title: Option<String>,
    /// Enable interactive 3D tilt following pointer.
    #[prop_or(false)]
    pub tilt: bool,
    /// Override max tilt in degrees (both axes). Ignored if `tilt` = false.
    #[prop_or(DEFAULT_TILT_MAX)]
    pub tilt_max: f32,
}

#[function_component(Tile)]
pub fn tile(props: &TileProps) -> Html {
    // State for current tilt (deg). We keep as (x,y) representing rotateX, rotateY.
    let tilt_state = use_state(|| (0f32, 0f32));
    let container_ref = use_node_ref();

    // Mouse move handler (only attached if tilt enabled)
    let on_mouse_move = {
        let tilt_state = tilt_state.clone();
        let node_ref = container_ref.clone();
        let max = props.tilt_max;
        Callback::from(move |e: MouseEvent| {
            if let Some(el) = node_ref.cast::<Element>() {
                let rect = el.get_bounding_client_rect();
                let x = e.client_x() as f64 - rect.left();
                let y = e.client_y() as f64 - rect.top();
                let w = rect.width();
                let h = rect.height();
                if w > 0.0 && h > 0.0 {
                    // Normalized -0.5..0.5
                    let nx = (x / w) - 0.5;
                    let ny = (y / h) - 0.5;
                    let rotate_y = (nx * max as f64) as f32; // horizontal moves rotateY
                    let rotate_x = (-ny * max as f64) as f32; // vertical moves rotateX
                    tilt_state.set((rotate_x, rotate_y));
                }
            }
        })
    };

    // Reset tilt on mouse leave
    let reset_handle = tilt_state.clone();
    let on_mouse_leave = { Callback::from(move |_| reset_handle.set((0.0, 0.0))) };

    let (tilt_x, tilt_y) = *tilt_state;
    // Inline style variables (only when tilt enabled to avoid unnecessary rerenders elsewhere)
    let style = if props.tilt {
        // Parallax translation (smaller than rotation); heavy feel uses inverse smaller movement.
        let shift_x = -(tilt_y as f32) / props.tilt_max * 8.0; // px
        let shift_y = (tilt_x as f32) / props.tilt_max * 8.0; // px
        Some(format!(
            "--tilt-x:{:.2}deg;--tilt-y:{:.2}deg;--tilt-shift-x:{:.2}px;--tilt-shift-y:{:.2}px;",
            tilt_x, tilt_y, shift_x, shift_y
        ))
    } else {
        None
    };

    html! {
        <div
            ref={container_ref.clone()}
            class={classes!("subway-tile")}
            style={style}
            onmousemove={props.tilt.then(|| on_mouse_move)}
            onmouseleave={props.tilt.then(|| on_mouse_leave)}
        >
            { if let Some(t) = &props.title { 
                html!(<div class="subway-tile-title">{t}</div>) 
            } else { 
                Html::default() 
            } }
            <div class="subway-tile-body">{ for props.children.iter() }</div>
        </div>
    }
}
