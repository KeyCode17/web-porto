use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn init_scroll_reveals() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Create IntersectionObserver callback
    let callback = Closure::<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>::new(
        move |entries: js_sys::Array, observer: web_sys::IntersectionObserver| {
            for i in 0..entries.length() {
                let entry: web_sys::IntersectionObserverEntry =
                    entries.get(i).unchecked_into();
                if entry.is_intersecting() {
                    let target = entry.target();
                    let class_list = target.class_list();
                    class_list.add_1("revealed").unwrap();
                    observer.unobserve(&target);
                }
            }
        },
    );

    // Create observer with threshold
    let options = web_sys::IntersectionObserverInit::new();
    options.set_threshold(&JsValue::from(0.1));

    let observer = web_sys::IntersectionObserver::new_with_options(
        callback.as_ref().unchecked_ref(),
        &options,
    )
    .unwrap();

    // Observe all elements with data-reveal attribute
    let elements = document.query_selector_all("[data-reveal]").unwrap();
    for i in 0..elements.length() {
        let el = elements.get(i).unwrap();
        let el: web_sys::Element = el.unchecked_into();
        observer.observe(&el);
    }

    callback.forget();
}
