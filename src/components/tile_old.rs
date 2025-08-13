//! Live tile style content block.
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
    /// If true, logs mouse enter + periodic move debug info to console.
    #[prop_or(false)]
    pub tilt_debug: bool,
}

#[function_component(Tile)]
pub fn tile(props: &TileProps) -> Html {
    // State for current tilt (deg). We keep as (x,y) representing rotateX, rotateY.
    let tilt_state = use_state(|| (0f32, 0f32));
    let container_ref = use_node_ref();

    // Effect: log once when tilt listeners will be active.
    {
        let title = props.title.clone();
        let tilt = props.tilt;
        let debug = props.tilt_debug;
        use_effect(move || {
            if tilt && debug {
                if let Some(t) = &title {
                    log::info!("tile render (tilt active): {t}");
                } else {
                    log::info!("tile render (tilt active, untitled)");
                }
            }
            || ()
        });
    }

    // Counter (moves) for throttled debug logging.
    let move_count = use_state(|| 0u32);

    // Mouse move handler (only attached if tilt enabled)
    let on_mouse_move = {
        let tilt_state = tilt_state.clone();
        let node_ref = container_ref.clone();
        let max = props.tilt_max;
        let move_count = move_count.clone();
        let debug = props.tilt_debug;
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

                    if debug {
                        let c = *move_count + 1;
                        move_count.set(c);
                        if c % 20 == 0 {
                            // log every 20th event
                            log::debug!("tile mousemove tilt=({rotate_x:.1},{rotate_y:.1}) norm=({nx:.2},{ny:.2})");
                        }
                    }
                }
            }
        })
    };

    // Clone the state handle so we can still read the original after moving this clone.
    let reset_handle = tilt_state.clone();
    let on_mouse_leave = { Callback::from(move |_| reset_handle.set((0.0, 0.0))) };

    // Mouse enter logs (only when debug enabled)
    let on_mouse_enter = if props.tilt && props.tilt_debug {
        let title = props.title.clone();
        Some(Callback::from(move |_| {
            if let Some(t) = &title {
                log::info!("tile enter: {t}");
            } else {
                log::info!("tile enter (untitled)");
            }
        }))
    } else {
        None
    };

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

    let base = html! {
        <div
            ref={container_ref.clone()}
            class={classes!("mu-tile")}
            style={style}
            onmousemove={props.tilt.then(|| on_mouse_move)}
            onmouseleave={props.tilt.then(|| on_mouse_leave)}
            onmouseenter={on_mouse_enter}
        >
            { if let Some(t) = &props.title { html!(<div class="mu-tile-title">{t}</div>) } else { Html::default() } }
            <div class="mu-tile-body">{ for props.children.iter() }</div>
        </div>
    };

    base
}
