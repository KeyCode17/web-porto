use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

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
        "language" => "#273B76",
        "ai" => "#4C2E05",
        "framework" => "#93B7BE",
        "tool" => "#93B7BE",
        "infra" => "#D5C7BC",
        _ => "#273B76",
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register requestAnimationFrame");
}

fn build_graph(skills_json: &str, width: f64, height: f64) -> Graph {
    let skills: Vec<shared::Skill> =
        serde_json::from_str(skills_json).expect("Invalid skills JSON");

    let cx = width / 2.0;
    let cy = height / 2.0;

    let nodes: Vec<Node> = skills
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let angle = (i as f64 / skills.len() as f64) * std::f64::consts::TAU;
            let spread = 150.0;
            Node {
                x: cx + angle.cos() * spread + js_sys::Math::random() * 50.0 - 25.0,
                y: cy + angle.sin() * spread + js_sys::Math::random() * 50.0 - 25.0,
                vx: 0.0,
                vy: 0.0,
                name: s.name.clone(),
                category: s.category.clone(),
                proficiency: s.proficiency,
                color: category_color(&s.category),
                radius: s.proficiency as f64 * 5.0 + 10.0,
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

    // Repulsion between all node pairs
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = graph.nodes[i].x - graph.nodes[j].x;
            let dy = graph.nodes[i].y - graph.nodes[j].y;
            let dist_sq = dx * dx + dy * dy;
            let dist = dist_sq.sqrt().max(30.0);
            let force = (2000.0 / (dist * dist)).min(10.0);
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

    // Attraction along edges (spring)
    for edge in &graph.edges {
        let dx = graph.nodes[edge.target].x - graph.nodes[edge.source].x;
        let dy = graph.nodes[edge.target].y - graph.nodes[edge.source].y;
        let dist = (dx * dx + dy * dy).sqrt().max(1.0);
        let force = dist * 0.005;
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

    // Centering force + damping + position update
    for node in graph.nodes.iter_mut() {
        if node.is_dragging {
            node.vx = 0.0;
            node.vy = 0.0;
            continue;
        }
        // Centering
        node.vx += (cx - node.x) * 0.01;
        node.vy += (cy - node.y) * 0.01;
        // Damping
        node.vx *= 0.9;
        node.vy *= 0.9;
        // Update position
        node.x += node.vx;
        node.y += node.vy;
        // Keep within bounds
        node.x = node.x.clamp(node.radius, width - node.radius);
        node.y = node.y.clamp(node.radius, height - node.radius);
    }
}

fn render(graph: &Graph, ctx: &CanvasRenderingContext2d, width: f64, height: f64) {
    ctx.clear_rect(0.0, 0.0, width, height);

    // Draw edges
    ctx.set_stroke_style_str("#93B7BE");
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
    for node in &graph.nodes {
        // Fill
        ctx.set_fill_style_str(node.color);
        ctx.begin_path();
        ctx.arc(node.x, node.y, node.radius, 0.0, std::f64::consts::TAU)
            .unwrap();
        ctx.fill();

        // Border
        let border_color = if node.color == "#D5C7BC" {
            "#273B76"
        } else {
            node.color
        };
        ctx.set_stroke_style_str(border_color);
        ctx.set_line_width(2.0);
        ctx.begin_path();
        ctx.arc(node.x, node.y, node.radius, 0.0, std::f64::consts::TAU)
            .unwrap();
        ctx.stroke();

        // Label
        if node.radius >= 20.0 {
            // Draw text centered on node
            ctx.set_fill_style_str("#FFFFFF");
            ctx.set_font(&format!("bold {}px sans-serif", (node.radius * 0.55).max(10.0)));
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            ctx.fill_text(&node.name, node.x, node.y).unwrap();
        } else {
            // Draw text next to node
            ctx.set_fill_style_str("#273B76");
            ctx.set_font("bold 11px sans-serif");
            ctx.set_text_align("left");
            ctx.set_text_baseline("middle");
            ctx.fill_text(&node.name, node.x + node.radius + 4.0, node.y)
                .unwrap();
        }
    }
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
            let g = graph_clone.borrow();
            if let Some(idx) = g.drag_index {
                drop(g);
                let (x, y) = get_canvas_coords(&canvas_clone, &event);
                let mut g = graph_clone.borrow_mut();
                g.nodes[idx].x = x;
                g.nodes[idx].y = y;
            }
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

    // Animation loop
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    let canvas_for_loop = canvas_rc.clone();
    let ctx_for_loop = ctx_rc.clone();
    let graph_for_loop = graph.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        let w = canvas_for_loop.width() as f64;
        let h = canvas_for_loop.height() as f64;

        let mut gr = graph_for_loop.borrow_mut();
        simulate(&mut gr, w, h);
        render(&gr, &ctx_for_loop, w, h);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
