mod clock;
mod constants;
mod draw;
mod particles;
mod state;

use crate::libs::{AnimationFrameLoop, start_animation_loop};
pub use clock::Direction;
use draw::draw;
use state::RangeState;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct ShootingRange {
    state: Rc<RefCell<RangeState>>,
    _frames: AnimationFrameLoop,
}

impl ShootingRange {
    pub fn queue_shots(&self, shots: usize, direction: Direction) {
        self.state.borrow_mut().queue(shots.max(1), direction);
    }
}

pub fn start_range(canvas_id: &str) -> Option<ShootingRange> {
    let document = web_sys::window()?.document()?;
    let canvas = document
        .get_element_by_id(canvas_id)?
        .dyn_into::<HtmlCanvasElement>()
        .ok()?;

    let rect = canvas.get_bounding_client_rect();
    if rect.width() < 1.0 || rect.height() < 1.0 {
        return None;
    }
    canvas.set_width(rect.width() as u32);
    canvas.set_height(rect.height() as u32);

    let ctx = canvas
        .get_context("2d")
        .ok()??
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()?;

    let state = Rc::new(RefCell::new(RangeState::new()));
    let canvas_rc = Rc::new(canvas);

    let loop_state = state.clone();
    let loop_canvas = canvas_rc.clone();
    let frames = start_animation_loop(move || {
        let width = loop_canvas.width() as f64;
        let height = loop_canvas.height() as f64;
        let mut current = loop_state.borrow_mut();
        current.step(width, height);
        draw(&ctx, &current, width, height);
    })?;

    Some(ShootingRange {
        state,
        _frames: frames,
    })
}
