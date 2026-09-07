use crate::utils::EventListener;
use dioxus::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;

const HERO_OVERLAY_ID: &str = "hero-bg-overlay";
const DARK_SECTION_IDS: &[&str] = &["contact", "experience"];
const OVERLAY_VISIBLE_THRESHOLD: f64 = 0.5;
const CURSOR_COLOR_ON_DARK: &str = "#00E676";
const CURSOR_COLOR_ON_LIGHT: &str = "#00C853";

fn is_on_dark_bg(cx: f64, cy: f64) -> bool {
    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return false,
    };

    for id in DARK_SECTION_IDS {
        if let Some(el) = document.get_element_by_id(id) {
            let rect = el.get_bounding_client_rect();
            if cy >= rect.top() && cy <= rect.bottom() && cx >= rect.left() && cx <= rect.right() {
                return true;
            }
        }
    }

    if let Some(overlay) = document.get_element_by_id(HERO_OVERLAY_ID)
        && let Ok(html) = overlay.dyn_into::<web_sys::HtmlElement>()
    {
        let rect = html.get_bounding_client_rect();
        if cy >= rect.top()
            && cy <= rect.bottom()
            && cx >= rect.left()
            && cx <= rect.right()
            && let Some(window) = web_sys::window()
            && let Some(computed) = window.get_computed_style(&html).ok().flatten()
            && let Ok(opacity_str) = computed.get_property_value("opacity")
            && let Ok(opacity) = opacity_str.parse::<f64>()
            && opacity > OVERLAY_VISIBLE_THRESHOLD
        {
            return true;
        }
    }

    false
}

#[component]
pub fn CustomCursor() -> Element {
    let mut x = use_signal(|| 0.0f64);
    let mut y = use_signal(|| 0.0f64);
    let mut on_dark = use_signal(|| false);

    let _cursor_listener = use_hook(|| {
        Rc::new(
            web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| {
                    EventListener::new(document.as_ref(), "mousemove", move |event| {
                        let Some(mouse_event) = event.dyn_ref::<web_sys::MouseEvent>() else {
                            return;
                        };
                        let cx = mouse_event.client_x() as f64;
                        let cy = mouse_event.client_y() as f64;
                        x.set(cx);
                        y.set(cy);
                        on_dark.set(is_on_dark_bg(cx, cy));
                    })
                }),
        )
    });

    let cx = *x.read();
    let cy = *y.read();
    let color = match *on_dark.read() {
        true => CURSOR_COLOR_ON_DARK,
        false => CURSOR_COLOR_ON_LIGHT,
    };

    rsx! {
        div {
            class: "custom-cursor",
            style: "pointer-events: none; position: fixed; z-index: 9999; width: 20px; height: 20px; border: 2px solid {color}; border-radius: 50%; transform: translate(-50%, -50%); left: {cx}px; top: {cy}px; transition: left 0.08s ease-out, top 0.08s ease-out, width 0.2s, height 0.2s, border-color 0.2s;",
        }
    }
}
