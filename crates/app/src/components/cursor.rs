use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::styles::theme;

fn is_on_dark_bg(cx: f64, cy: f64) -> bool {
    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return false,
    };

    // Check known dark sections by their bounding rects
    let dark_ids = ["contact", "experience"];
    for id in &dark_ids {
        if let Some(el) = document.get_element_by_id(id) {
            let rect = el.get_bounding_client_rect();
            if cy >= rect.top() && cy <= rect.bottom() && cx >= rect.left() && cx <= rect.right() {
                return true;
            }
        }
    }

    // Check the about overlay (visible when opacity > 0)
    if let Some(overlay) = document.get_element_by_id("hero-bg-overlay") {
        if let Ok(html) = overlay.dyn_into::<web_sys::HtmlElement>() {
            let rect = html.get_bounding_client_rect();
            if cy >= rect.top() && cy <= rect.bottom() && cx >= rect.left() && cx <= rect.right() {
                // Check if overlay is actually visible (opacity > 0.5)
                if let Some(window) = web_sys::window() {
                    if let Some(computed) = window.get_computed_style(&html).ok().flatten() {
                        if let Ok(opacity_str) = computed.get_property_value("opacity") {
                            if let Ok(opacity) = opacity_str.parse::<f64>() {
                                if opacity > 0.5 {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    false
}

#[component]
pub fn CustomCursor() -> Element {
    let mut x = use_signal(|| 0.0f64);
    let mut y = use_signal(|| 0.0f64);
    let mut on_dark = use_signal(|| false);

    use_effect(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let cb = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let cx = e.client_x() as f64;
            let cy = e.client_y() as f64;
            x.set(cx);
            y.set(cy);
            on_dark.set(is_on_dark_bg(cx, cy));
        });

        document
            .add_event_listener_with_callback("mousemove", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();
    });

    let cx = *x.read();
    let cy = *y.read();
    let color = if *on_dark.read() { "#00E676" } else { "#00C853" };

    rsx! {
        div {
            class: "custom-cursor",
            style: "pointer-events: none; position: fixed; z-index: 9999; width: 20px; height: 20px; border: 2px solid {color}; border-radius: 50%; transform: translate(-50%, -50%); left: {cx}px; top: {cy}px; transition: left 0.08s ease-out, top 0.08s ease-out, width 0.2s, height 0.2s, border-color 0.2s;",
        }
    }
}
