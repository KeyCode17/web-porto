use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::styles::theme;

#[component]
pub fn CustomCursor() -> Element {
    let mut x = use_signal(|| 0.0f64);
    let mut y = use_signal(|| 0.0f64);

    use_effect(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let cb = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            x.set(e.client_x() as f64);
            y.set(e.client_y() as f64);
        });

        document
            .add_event_listener_with_callback("mousemove", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();
    });

    let cx = *x.read();
    let cy = *y.read();

    rsx! {
        div {
            class: "custom-cursor",
            style: "pointer-events: none; position: fixed; z-index: 9999; width: 20px; height: 20px; border: 2px solid {theme::DEEP_NAVY}; border-radius: 50%; transform: translate(-50%, -50%); left: {cx}px; top: {cy}px; transition: left 0.08s ease-out, top 0.08s ease-out, width 0.2s, height 0.2s; mix-blend-mode: difference;",
        }
    }
}
