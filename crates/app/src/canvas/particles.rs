use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const PARTICLE_COUNT: usize = 120;
const CONNECTION_DISTANCE: f64 = 120.0;
const COLORS: [&str; 4] = ["#273B76", "#93B7BE", "#D5C7BC", "#4C2E05"];

struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    radius: f64,
    color: &'static str,
}

fn random_range(min: f64, max: f64) -> f64 {
    min + js_sys::Math::random() * (max - min)
}

fn init_particles(width: f64, height: f64) -> Vec<Particle> {
    (0..PARTICLE_COUNT)
        .map(|_| {
            let speed = random_range(0.2, 0.8);
            let angle = random_range(0.0, std::f64::consts::TAU);
            let color_idx = (js_sys::Math::random() * COLORS.len() as f64) as usize;
            Particle {
                x: random_range(0.0, width),
                y: random_range(0.0, height),
                vx: speed * angle.cos(),
                vy: speed * angle.sin(),
                radius: random_range(2.0, 5.0),
                color: COLORS[color_idx.min(COLORS.len() - 1)],
            }
        })
        .collect()
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register requestAnimationFrame");
}

pub fn start_particles(canvas_id: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = match document.get_element_by_id(canvas_id) {
        Some(el) => el,
        None => return,
    };
    let canvas: HtmlCanvasElement = match canvas.dyn_into() {
        Ok(c) => c,
        Err(_) => return,
    };

    let width = window.inner_width().unwrap().as_f64().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    let particles = Rc::new(RefCell::new(init_particles(width, height)));
    let canvas_rc = Rc::new(canvas);
    let ctx_rc = Rc::new(ctx);

    // Handle window resize
    {
        let canvas_clone = canvas_rc.clone();
        let resize_cb = Closure::<dyn FnMut()>::new(move || {
            let w = web_sys::window().unwrap();
            let new_w = w.inner_width().unwrap().as_f64().unwrap();
            let new_h = w.inner_height().unwrap().as_f64().unwrap();
            canvas_clone.set_width(new_w as u32);
            canvas_clone.set_height(new_h as u32);
        });
        window
            .add_event_listener_with_callback("resize", resize_cb.as_ref().unchecked_ref())
            .unwrap();
        resize_cb.forget();
    }

    // Animation loop
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    let canvas_for_loop = canvas_rc.clone();
    let ctx_for_loop = ctx_rc.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        let w = canvas_for_loop.width() as f64;
        let h = canvas_for_loop.height() as f64;

        // Clear
        ctx_for_loop.clear_rect(0.0, 0.0, w, h);

        let mut parts = particles.borrow_mut();

        // Update positions
        for p in parts.iter_mut() {
            p.x += p.vx;
            p.y += p.vy;

            if p.x <= 0.0 || p.x >= w {
                p.vx = -p.vx;
                p.x = p.x.clamp(0.0, w);
            }
            if p.y <= 0.0 || p.y >= h {
                p.vy = -p.vy;
                p.y = p.y.clamp(0.0, h);
            }
        }

        // Draw connections
        let len = parts.len();
        for i in 0..len {
            for j in (i + 1)..len {
                let dx = parts[i].x - parts[j].x;
                let dy = parts[i].y - parts[j].y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < CONNECTION_DISTANCE {
                    let alpha = 1.0 - dist / CONNECTION_DISTANCE;
                    let style = format!("rgba(39, 59, 118, {:.2})", alpha * 0.3);
                    ctx_for_loop.set_stroke_style_str(&style);
                    ctx_for_loop.set_line_width(0.5);
                    ctx_for_loop.begin_path();
                    ctx_for_loop.move_to(parts[i].x, parts[i].y);
                    ctx_for_loop.line_to(parts[j].x, parts[j].y);
                    ctx_for_loop.stroke();
                }
            }
        }

        // Draw particles
        for p in parts.iter() {
            ctx_for_loop.set_fill_style_str(p.color);
            ctx_for_loop.begin_path();
            ctx_for_loop
                .arc(p.x, p.y, p.radius, 0.0, std::f64::consts::TAU)
                .unwrap();
            ctx_for_loop.fill();
        }

        // Request next frame
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
