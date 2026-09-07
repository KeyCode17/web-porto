use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

type TFrameClosure = Rc<RefCell<Option<Closure<dyn FnMut()>>>>;

pub struct AnimationFrameLoop {
    cancelled: Rc<Cell<bool>>,
    _closure: TFrameClosure,
}

impl Drop for AnimationFrameLoop {
    fn drop(&mut self) {
        self.cancelled.set(true);
    }
}

fn request_frame(closure: &Closure<dyn FnMut()>) -> bool {
    web_sys::window()
        .and_then(|window| {
            window
                .request_animation_frame(closure.as_ref().unchecked_ref())
                .ok()
        })
        .is_some()
}

pub fn start_animation_loop(mut frame: impl FnMut() + 'static) -> Option<AnimationFrameLoop> {
    let cancelled = Rc::new(Cell::new(false));
    let closure: TFrameClosure = Rc::new(RefCell::new(None));

    let scheduled = closure.clone();
    let stop = cancelled.clone();

    *closure.borrow_mut() = Some(Closure::new(move || {
        if stop.get() {
            return;
        }
        frame();
        if let Some(next) = scheduled.borrow().as_ref() {
            request_frame(next);
        }
    }));

    let started = closure.borrow().as_ref().is_some_and(request_frame);

    match started {
        true => Some(AnimationFrameLoop {
            cancelled,
            _closure: closure,
        }),
        false => None,
    }
}
