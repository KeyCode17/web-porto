use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, TouchEvent};

struct Node {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    name: String,
    #[allow(dead_code)]
    category: String,
    #[allow(dead_code)]
    proficiency: u8,
    color: &'static str,
    radius: f64,
    is_dragging: bool,
}

struct Edge {
    source: usize,
    target: usize,
}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    drag_index: Option<usize>,
}

fn category_color(cat: &str) -> &'static str {
    match cat {
        "language" => "#02182B",
        "ai" => "#D65108",
        "framework" => "#568EA3",
        "tool" => "#568EA3",
        "infra" => "#591F0A",
        _ => "#02182B",
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register requestAnimationFrame");
}

fn build_graph(skills_json: &str, width: f64, height: f64) -> Graph {
    let skills: Vec<porto_shared::Skill> =
        serde_json::from_str(skills_json).expect("Invalid skills JSON");

    let cx = width / 2.0;
    let cy = height / 2.0;

    let spread = (width.min(height) * 0.35).max(100.0);
    // Scale node sizes down on small canvases
    let size_scale = (width.min(height) / 600.0).clamp(0.4, 1.0);
    let nodes: Vec<Node> = skills
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let angle = (i as f64 / skills.len() as f64) * std::f64::consts::TAU;
            Node {
                x: cx + angle.cos() * spread + js_sys::Math::random() * 40.0 - 20.0,
                y: cy + angle.sin() * spread + js_sys::Math::random() * 40.0 - 20.0,
                vx: 0.0,
                vy: 0.0,
                name: s.name.clone(),
                category: s.category.clone(),
                proficiency: s.proficiency,
                color: category_color(&s.category),
                radius: (s.proficiency as f64 * 4.0 + 8.0) * size_scale,
                is_dragging: false,
            }
        })
        .collect();

    // Build edges from connections
    let mut edges = Vec::new();
    for (i, s) in skills.iter().enumerate() {
        for conn in &s.connections {
            if let Some(j) = skills.iter().position(|other| &other.name == conn) {
                if i < j {
                    edges.push(Edge {
                        source: i,
                        target: j,
                    });
                }
            }
        }
    }

    Graph {
        nodes,
        edges,
        drag_index: None,
    }
}

fn simulate(graph: &mut Graph, width: f64, height: f64) {
    let cx = width / 2.0;
    let cy = height / 2.0;
    let n = graph.nodes.len();

    // Scale physics for small canvases
    let area_scale = (width.min(height) / 600.0).clamp(0.3, 1.0);

    // Repulsion between all node pairs — weaker on small screens
    let repulsion_strength = 8000.0 * area_scale;
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = graph.nodes[i].x - graph.nodes[j].x;
            let dy = graph.nodes[i].y - graph.nodes[j].y;
            let min_dist = graph.nodes[i].radius + graph.nodes[j].radius + 10.0;
            let dist = (dx * dx + dy * dy).sqrt().max(min_dist * 0.5);
            let force = (repulsion_strength / (dist * dist)).min(15.0);
            let fx = force * dx / dist;
            let fy = force * dy / dist;

            if !graph.nodes[i].is_dragging {
                graph.nodes[i].vx += fx;
                graph.nodes[i].vy += fy;
            }
            if !graph.nodes[j].is_dragging {
                graph.nodes[j].vx -= fx;
                graph.nodes[j].vy -= fy;
            }
        }
    }

    // Attraction along edges — shorter rest length on small screens
    let ideal_length = 100.0 * area_scale;
    for edge in &graph.edges {
        let dx = graph.nodes[edge.target].x - graph.nodes[edge.source].x;
        let dy = graph.nodes[edge.target].y - graph.nodes[edge.source].y;
        let dist = (dx * dx + dy * dy).sqrt().max(1.0);
        let force = (dist - ideal_length) * 0.003;
        let fx = force * dx / dist;
        let fy = force * dy / dist;

        if !graph.nodes[edge.source].is_dragging {
            graph.nodes[edge.source].vx += fx;
            graph.nodes[edge.source].vy += fy;
        }
        if !graph.nodes[edge.target].is_dragging {
            graph.nodes[edge.target].vx -= fx;
            graph.nodes[edge.target].vy -= fy;
        }
    }

    // Centering force — stronger on small screens to keep nodes away from edges
    let center_strength = 0.003 + (1.0 - area_scale) * 0.012;
    let padding = 20.0;
    for node in graph.nodes.iter_mut() {
        if node.is_dragging {
            node.vx = 0.0;
            node.vy = 0.0;
            continue;
        }
        node.vx += (cx - node.x) * center_strength;
        node.vy += (cy - node.y) * center_strength;
        // Damping
        node.vx *= 0.85;
        node.vy *= 0.85;
        // Update position
        node.x += node.vx;
        node.y += node.vy;
        // Keep within bounds with padding for labels
        node.x = node.x.clamp(node.radius + padding, width - node.radius - padding);
        node.y = node.y.clamp(node.radius + padding, height - node.radius - padding);
    }
}

fn render(graph: &Graph, ctx: &CanvasRenderingContext2d, width: f64, height: f64, frame: f64) {
    ctx.clear_rect(0.0, 0.0, width, height);

    // Draw edges
    ctx.set_stroke_style_str("#568EA3");
    ctx.set_line_width(0.5);
    for edge in &graph.edges {
        let src = &graph.nodes[edge.source];
        let tgt = &graph.nodes[edge.target];
        ctx.begin_path();
        ctx.move_to(src.x, src.y);
        ctx.line_to(tgt.x, tgt.y);
        ctx.stroke();
    }

    // Draw nodes
    for (i, node) in graph.nodes.iter().enumerate() {
        // Subtle breathing pulse — each node slightly offset in phase
        let pulse_scale = (width.min(height) / 600.0).clamp(0.5, 1.0);
        let pulse = ((frame * 0.02 + i as f64 * 0.5).sin()) * 1.5 * pulse_scale;
        let r = node.radius + pulse;

        // Fill
        ctx.set_fill_style_str(node.color);
        ctx.begin_path();
        ctx.arc(node.x, node.y, r, 0.0, std::f64::consts::TAU)
            .unwrap();
        ctx.fill();

        // Border
        let border_color = if node.color == "#591F0A" {
            "#02182B"
        } else {
            node.color
        };
        ctx.set_stroke_style_str(border_color);
        ctx.set_line_width(2.0);
        ctx.begin_path();
        ctx.arc(node.x, node.y, r, 0.0, std::f64::consts::TAU)
            .unwrap();
        ctx.stroke();

        // Label — position adapts to avoid clipping at edges
        ctx.set_fill_style_str("#02182B");
        let font_size = (node.radius * 0.6).max(7.0).min(13.0);
        ctx.set_font(&format!("bold {}px sans-serif", font_size));
        ctx.set_text_align("center");
        if node.y + node.radius + 20.0 > height {
            // Near bottom — draw label above
            ctx.set_text_baseline("bottom");
            ctx.fill_text(&node.name, node.x, node.y - node.radius - 3.0).unwrap();
        } else {
            // Normal — draw label below
            ctx.set_text_baseline("top");
            ctx.fill_text(&node.name, node.x, node.y + node.radius + 3.0).unwrap();
        }
    }
}

fn get_touch_coords(canvas: &HtmlCanvasElement, event: &TouchEvent) -> Option<(f64, f64)> {
    let touch = event.touches().get(0)?;
    let rect = canvas.get_bounding_client_rect();
    let scale_x = canvas.width() as f64 / rect.width();
    let scale_y = canvas.height() as f64 / rect.height();
    let x = (touch.client_x() as f64 - rect.left()) * scale_x;
    let y = (touch.client_y() as f64 - rect.top()) * scale_y;
    Some((x, y))
}

fn get_canvas_coords(canvas: &HtmlCanvasElement, event: &MouseEvent) -> (f64, f64) {
    let rect = canvas.get_bounding_client_rect();
    let scale_x = canvas.width() as f64 / rect.width();
    let scale_y = canvas.height() as f64 / rect.height();
    let x = (event.client_x() as f64 - rect.left()) * scale_x;
    let y = (event.client_y() as f64 - rect.top()) * scale_y;
    (x, y)
}

fn find_node_at(graph: &Graph, x: f64, y: f64) -> Option<usize> {
    for (i, node) in graph.nodes.iter().enumerate() {
        let dx = node.x - x;
        let dy = node.y - y;
        if dx * dx + dy * dy <= node.radius * node.radius {
            return Some(i);
        }
    }
    None
}

pub fn start_force_graph(canvas_id: &str, skills_json: &str) {
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

    // Size canvas to its CSS layout size
    let rect = canvas.get_bounding_client_rect();
    let width = rect.width();
    let height = rect.height();
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    let graph = Rc::new(RefCell::new(build_graph(skills_json, width, height)));
    let canvas_rc = Rc::new(canvas);
    let ctx_rc = Rc::new(ctx);

    // Mouse event handlers
    {
        let graph_clone = graph.clone();
        let canvas_clone = canvas_rc.clone();
        let mousedown = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
            let (x, y) = get_canvas_coords(&canvas_clone, &event);
            let mut g = graph_clone.borrow_mut();
            if let Some(idx) = find_node_at(&g, x, y) {
                g.nodes[idx].is_dragging = true;
                g.drag_index = Some(idx);
            }
        });
        canvas_rc
            .add_event_listener_with_callback("mousedown", mousedown.as_ref().unchecked_ref())
            .unwrap();
        mousedown.forget();
    }

    {
        let graph_clone = graph.clone();
        let canvas_clone = canvas_rc.clone();
        let mousemove = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
            let (x, y) = get_canvas_coords(&canvas_clone, &event);
            let mut g = graph_clone.borrow_mut();
            if let Some(idx) = g.drag_index {
                g.nodes[idx].x = x;
                g.nodes[idx].y = y;
            }
            // Change cursor when hovering over a node
            let hovering = find_node_at(&g, x, y).is_some();
            let cursor = if g.drag_index.is_some() {
                "grabbing"
            } else if hovering {
                "grab"
            } else {
                "default"
            };
            canvas_clone.style().set_property("cursor", cursor).unwrap();
        });
        canvas_rc
            .add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref())
            .unwrap();
        mousemove.forget();
    }

    {
        let graph_clone = graph.clone();
        let mouseup = Closure::<dyn FnMut(MouseEvent)>::new(move |_event: MouseEvent| {
            let mut g = graph_clone.borrow_mut();
            if let Some(idx) = g.drag_index {
                g.nodes[idx].is_dragging = false;
                g.drag_index = None;
            }
        });
        canvas_rc
            .add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref())
            .unwrap();
        mouseup.forget();
    }

    // Touch event handlers
    {
        let graph_clone = graph.clone();
        let canvas_clone = canvas_rc.clone();
        let touchstart = Closure::<dyn FnMut(TouchEvent)>::new(move |event: TouchEvent| {
            event.prevent_default();
            if let Some((x, y)) = get_touch_coords(&canvas_clone, &event) {
                let mut g = graph_clone.borrow_mut();
                if let Some(idx) = find_node_at(&g, x, y) {
                    g.nodes[idx].is_dragging = true;
                    g.drag_index = Some(idx);
                }
            }
        });
        canvas_rc
            .add_event_listener_with_callback("touchstart", touchstart.as_ref().unchecked_ref())
            .unwrap();
        touchstart.forget();
    }

    {
        let graph_clone = graph.clone();
        let canvas_clone = canvas_rc.clone();
        let touchmove = Closure::<dyn FnMut(TouchEvent)>::new(move |event: TouchEvent| {
            event.prevent_default();
            if let Some((x, y)) = get_touch_coords(&canvas_clone, &event) {
                let mut g = graph_clone.borrow_mut();
                if let Some(idx) = g.drag_index {
                    g.nodes[idx].x = x;
                    g.nodes[idx].y = y;
                }
            }
        });
        canvas_rc
            .add_event_listener_with_callback("touchmove", touchmove.as_ref().unchecked_ref())
            .unwrap();
        touchmove.forget();
    }

    {
        let graph_clone = graph.clone();
        let touchend = Closure::<dyn FnMut(TouchEvent)>::new(move |_event: TouchEvent| {
            let mut g = graph_clone.borrow_mut();
            if let Some(idx) = g.drag_index {
                g.nodes[idx].is_dragging = false;
                g.drag_index = None;
            }
        });
        canvas_rc
            .add_event_listener_with_callback("touchend", touchend.as_ref().unchecked_ref())
            .unwrap();
        touchend.forget();
    }

    // Handle window resize — rescale canvas and node positions
    {
        let canvas_clone = canvas_rc.clone();
        let graph_clone = graph.clone();
        let resize_cb = Closure::<dyn FnMut()>::new(move || {
            let rect = canvas_clone.get_bounding_client_rect();
            let new_w = rect.width();
            let new_h = rect.height();
            if new_w < 1.0 || new_h < 1.0 {
                return;
            }
            let old_w = canvas_clone.width() as f64;
            let old_h = canvas_clone.height() as f64;
            if (new_w - old_w).abs() < 1.0 && (new_h - old_h).abs() < 1.0 {
                return;
            }
            canvas_clone.set_width(new_w as u32);
            canvas_clone.set_height(new_h as u32);

            // Rescale node positions and radii
            let scale_x = new_w / old_w;
            let scale_y = new_h / old_h;
            let new_size_scale = (new_w.min(new_h) / 600.0).clamp(0.4, 1.0);
            let old_size_scale = (old_w.min(old_h) / 600.0).clamp(0.4, 1.0);
            let radius_scale = new_size_scale / old_size_scale;

            let mut g = graph_clone.borrow_mut();
            for node in g.nodes.iter_mut() {
                node.x *= scale_x;
                node.y *= scale_y;
                node.radius *= radius_scale;
                node.x = node.x.clamp(node.radius + 20.0, new_w - node.radius - 20.0);
                node.y = node.y.clamp(node.radius + 20.0, new_h - node.radius - 20.0);
            }
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
    let graph_for_loop = graph.clone();

    let frame_count = Rc::new(RefCell::new(0.0f64));

    *g.borrow_mut() = Some(Closure::new(move || {
        let w = canvas_for_loop.width() as f64;
        let h = canvas_for_loop.height() as f64;

        let mut frame = frame_count.borrow_mut();
        *frame += 1.0;

        let mut gr = graph_for_loop.borrow_mut();
        simulate(&mut gr, w, h);
        render(&gr, &ctx_for_loop, w, h, *frame);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
