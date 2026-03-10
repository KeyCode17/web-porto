use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn init_hero_zoom() {
    let window = web_sys::window().unwrap();

    let cb = Closure::<dyn FnMut()>::new(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let container = match document.get_element_by_id("hero-zoom-container") {
            Some(el) => el,
            None => return,
        };

        let rect = container.get_bounding_client_rect();
        let container_top = rect.top();
        let container_height = rect.height();
        let vh = window.inner_height().unwrap().as_f64().unwrap();

        // 0.0 at top, 1.0 when fully scrolled through
        let progress = (-container_top / (container_height - vh)).clamp(0.0, 1.0);

        // === Phase 1: Hero zoom (0% - 50%) ===
        let hero_progress = (progress / 0.5).clamp(0.0, 1.0);

        // Ease-in cubic: starts slow, accelerates — feels smooth
        let eased = hero_progress * hero_progress * hero_progress;

        // Name scales 1x -> 20x with easing
        let scale = 1.0 + eased * 19.0;

        // Subtitle fades out instantly
        let subtitle_opacity = (1.0 - hero_progress * 4.0).clamp(0.0, 1.0);

        // Background overlay to WARM_BEIGE
        let bg_opacity = (hero_progress / 0.6).clamp(0.0, 1.0);

        // Hero text wrapper fades out at end of phase 1
        let hero_text_opacity = (1.0 - (hero_progress - 0.7) / 0.3).clamp(0.0, 1.0);

        if let Some(el) = document.get_element_by_id("hero-name") {
            let el: web_sys::HtmlElement = el.dyn_into().unwrap();
            el.style().set_property("transform", &format!("scale({})", scale)).unwrap();
        }
        if let Some(el) = document.get_element_by_id("hero-subtitle") {
            let el: web_sys::HtmlElement = el.dyn_into().unwrap();
            el.style().set_property("opacity", &format!("{}", subtitle_opacity)).unwrap();
        }
        if let Some(el) = document.get_element_by_id("hero-bg-overlay") {
            let el: web_sys::HtmlElement = el.dyn_into().unwrap();
            el.style().set_property("opacity", &format!("{}", bg_opacity)).unwrap();
        }
        if let Some(el) = document.get_element_by_id("hero-text-wrapper") {
            let el: web_sys::HtmlElement = el.dyn_into().unwrap();
            el.style().set_property("opacity", &format!("{}", hero_text_opacity)).unwrap();
        }

        // === Phase 2: About appears (40% - 80%) ===
        let about_progress = ((progress - 0.4) / 0.4).clamp(0.0, 1.0);

        // About content fades in (0 -> 1)
        if let Some(el) = document.get_element_by_id("about-content") {
            let el: web_sys::HtmlElement = el.dyn_into().unwrap();
            el.style().set_property("opacity", &format!("{}", about_progress)).unwrap();
            // Enable pointer events when visible
            let pe = if about_progress > 0.5 { "auto" } else { "none" };
            el.style().set_property("pointer-events", pe).unwrap();
        }
    });

    window
        .add_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref())
        .unwrap();
    cb.forget();
}
