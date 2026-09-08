use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

pub struct EventListener {
    target: web_sys::EventTarget,
    event: String,
    closure: Closure<dyn FnMut(web_sys::Event)>,
}

impl EventListener {
    pub fn new(
        target: &web_sys::EventTarget,
        event: &str,
        handler: impl FnMut(web_sys::Event) + 'static,
    ) -> Option<Self> {
        let closure = Closure::<dyn FnMut(web_sys::Event)>::new(handler);
        target
            .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
            .ok()?;
        Some(Self {
            target: target.clone(),
            event: event.to_string(),
            closure,
        })
    }
}

impl Drop for EventListener {
    fn drop(&mut self) {
        let _ = self.target.remove_event_listener_with_callback(
            &self.event,
            self.closure.as_ref().unchecked_ref(),
        );
    }
}

pub fn on_escape(mut handler: impl FnMut() + 'static) -> Option<EventListener> {
    let document = web_sys::window()?.document()?;
    EventListener::new(document.as_ref(), "keydown", move |event| {
        let is_escape = event
            .dyn_ref::<web_sys::KeyboardEvent>()
            .is_some_and(|key_event| key_event.key() == "Escape");
        if is_escape {
            handler();
        }
    })
}
