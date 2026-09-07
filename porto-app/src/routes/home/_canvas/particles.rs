use crate::libs::EventListener;
use crate::libs::{AnimationFrameLoop, start_animation_loop};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const PARTICLE_COUNT: usize = 120;
const CONNECTION_DISTANCE: f64 = 120.0;
const COLORS: [&str; 4] = ["#02182B", "#568EA3", "#591F0A", "#D65108"];

struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    radius: f64,
    color: &'static str,
}

struct TextRect {
    left: f64,
    top: f64,
    right: f64,
    bottom: f64,
    _cx: f64,
    _cy: f64,
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

fn get_text_rect() -> Option<TextRect> {
    let document = web_sys::window()?.document()?;
    let el = document.get_element_by_id("hero-name")?;
    let canvas = document.get_element_by_id("hero-canvas")?;

    let text_rect = el.get_bounding_client_rect();
    let canvas_rect = canvas.get_bounding_client_rect();

    let left = text_rect.left() - canvas_rect.left();
    let top = text_rect.top() - canvas_rect.top();
    let right = text_rect.right() - canvas_rect.left();
    let bottom = text_rect.bottom() - canvas_rect.top();

    let pad = 20.0;
    Some(TextRect {
        left: left - pad,
        top: top - pad,
        right: right + pad,
        bottom: bottom + pad,
        _cx: (left + right) / 2.0,
        _cy: (top + bottom) / 2.0,
    })
}

fn repel_from_text(p: &mut Particle, tr: &TextRect) {
    let nearest_x = p.x.clamp(tr.left, tr.right);
    let nearest_y = p.y.clamp(tr.top, tr.bottom);
    let dx = p.x - nearest_x;
    let dy = p.y - nearest_y;
    let dist = (dx * dx + dy * dy).sqrt();

    if dist < 0.1 {
        let cx = (tr.left + tr.right) / 2.0;
        let cy = (tr.top + tr.bottom) / 2.0;
        let to_cx = p.x - cx;
        let to_cy = p.y - cy;
        let d = (to_cx * to_cx + to_cy * to_cy).sqrt().max(1.0);
        p.vx += (to_cx / d) * 0.15;
        p.vy += (to_cy / d) * 0.15;
    } else {
        let repel_range = 25.0;
        if dist < repel_range {
            let force = (1.0 - dist / repel_range) * 0.1;
            p.vx += (dx / dist) * force;
            p.vy += (dy / dist) * force;
        }
    }
}

fn clamp_velocity(p: &mut Particle) {
    let max_speed = 2.0;
    let speed = (p.vx * p.vx + p.vy * p.vy).sqrt();
    if speed > max_speed {
        p.vx = (p.vx / speed) * max_speed;
        p.vy = (p.vy / speed) * max_speed;
    }
}

pub struct ParticleField {
    _frames: AnimationFrameLoop,
    _resize: Option<EventListener>,
}

fn viewport_size() -> Option<(f64, f64)> {
    let window = web_sys::window()?;
    let width = window.inner_width().ok()?.as_f64()?;
    let height = window.inner_height().ok()?.as_f64()?;
    Some((width, height))
}

pub fn start_particles(canvas_id: &str) -> Option<ParticleField> {
    let document = web_sys::window()?.document()?;

    let canvas = document
        .get_element_by_id(canvas_id)?
        .dyn_into::<HtmlCanvasElement>()
        .ok()?;

    let (width, height) = viewport_size()?;
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let ctx = canvas
        .get_context("2d")
        .ok()??
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()?;

    let particles = Rc::new(RefCell::new(init_particles(width, height)));
    let canvas_rc = Rc::new(canvas);
    let ctx_rc = Rc::new(ctx);

    let resize = {
        let canvas_clone = canvas_rc.clone();
        web_sys::window().and_then(|window| {
            EventListener::new(window.as_ref(), "resize", move |_| {
                if let Some((new_width, new_height)) = viewport_size() {
                    canvas_clone.set_width(new_width as u32);
                    canvas_clone.set_height(new_height as u32);
                }
            })
        })
    };

    let canvas_for_loop = canvas_rc.clone();
    let ctx_for_loop = ctx_rc.clone();

    let frames = start_animation_loop(move || {
        let w = canvas_for_loop.width() as f64;
        let h = canvas_for_loop.height() as f64;

        ctx_for_loop.clear_rect(0.0, 0.0, w, h);

        let mut parts = particles.borrow_mut();

        let text_rect = get_text_rect();

        for p in parts.iter_mut() {
            p.x += p.vx;
            p.y += p.vy;

            if let Some(ref tr) = text_rect {
                repel_from_text(p, tr);
            }

            clamp_velocity(p);

            if p.x <= 0.0 || p.x >= w {
                p.vx = -p.vx;
                p.x = p.x.clamp(0.0, w);
            }
            if p.y <= 0.0 || p.y >= h {
                p.vy = -p.vy;
                p.y = p.y.clamp(0.0, h);
            }
        }

        let len = parts.len();
        for i in 0..len {
            for j in (i + 1)..len {
                let dx = parts[i].x - parts[j].x;
                let dy = parts[i].y - parts[j].y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < CONNECTION_DISTANCE {
                    let alpha = 1.0 - dist / CONNECTION_DISTANCE;
                    let style = format!("rgba(2, 24, 43, {:.2})", alpha * 0.3);
                    ctx_for_loop.set_stroke_style_str(&style);
                    ctx_for_loop.set_line_width(0.5);
                    ctx_for_loop.begin_path();
                    ctx_for_loop.move_to(parts[i].x, parts[i].y);
                    ctx_for_loop.line_to(parts[j].x, parts[j].y);
                    ctx_for_loop.stroke();
                }
            }
        }

        for p in parts.iter() {
            ctx_for_loop.set_fill_style_str(p.color);
            ctx_for_loop.begin_path();
            let _ = ctx_for_loop.arc(p.x, p.y, p.radius, 0.0, std::f64::consts::TAU);
            ctx_for_loop.fill();
        }
    })?;

    Some(ParticleField {
        _frames: frames,
        _resize: resize,
    })
}
