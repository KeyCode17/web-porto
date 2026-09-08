use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

const REVEAL_SELECTOR: &str = "[data-reveal]";
const REVEALED_CLASS: &str = "revealed";
const REVEAL_THRESHOLD: f64 = 0.1;

pub struct ScrollReveals {
    observer: web_sys::IntersectionObserver,
    _callback: Closure<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>,
}

impl Drop for ScrollReveals {
    fn drop(&mut self) {
        self.observer.disconnect();
    }
}

pub fn init_scroll_reveals() -> Option<ScrollReveals> {
    let document = web_sys::window()?.document()?;

    let callback = Closure::<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>::new(
        move |entries: js_sys::Array, observer: web_sys::IntersectionObserver| {
            for i in 0..entries.length() {
                let Ok(entry) = entries
                    .get(i)
                    .dyn_into::<web_sys::IntersectionObserverEntry>()
                else {
                    continue;
                };
                if !entry.is_intersecting() {
                    continue;
                }
                let target = entry.target();
                let _ = target.class_list().add_1(REVEALED_CLASS);
                observer.unobserve(&target);
            }
        },
    );

    let options = web_sys::IntersectionObserverInit::new();
    options.set_threshold(&JsValue::from(REVEAL_THRESHOLD));

    let observer = web_sys::IntersectionObserver::new_with_options(
        callback.as_ref().unchecked_ref(),
        &options,
    )
    .ok()?;

    let elements = document.query_selector_all(REVEAL_SELECTOR).ok()?;
    for i in 0..elements.length() {
        let Some(node) = elements.get(i) else {
            continue;
        };
        let Ok(element) = node.dyn_into::<web_sys::Element>() else {
            continue;
        };
        observer.observe(&element);
    }

    Some(ScrollReveals {
        observer,
        _callback: callback,
    })
}
