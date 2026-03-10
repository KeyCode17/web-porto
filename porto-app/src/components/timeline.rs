use std::cell::RefCell;
use std::rc::Rc;
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::data;
use crate::styles::theme;

fn parse_date(date_str: &str) -> (i32, u32) {
    if date_str.is_empty() {
        return (2026, 3);
    }
    let parts: Vec<&str> = date_str.split('-').collect();
    let year = parts[0].parse::<i32>().unwrap_or(2026);
    let month = if parts.len() > 1 {
        parts[1].parse::<u32>().unwrap_or(1)
    } else {
        1
    };
    (year, month)
}

fn date_to_months(year: i32, month: u32) -> f64 {
    (year as f64) * 12.0 + (month as f64)
}

fn bar_color(index: usize, kind: &str) -> &'static str {
    if kind == "education" {
        const EDU_COLORS: &[&str] = &[
            "#2D6A4F",
            "#1a5276",
            "#4A6741",
        ];
        return EDU_COLORS[index % EDU_COLORS.len()];
    }
    const COLORS: &[&str] = &[
        "#02182B",
        "#D65108",
        "#2D6A4F",
        "#6B4C8A",
        "#B85C38",
        "#1a5276",
    ];
    COLORS[index % COLORS.len()]
}

fn format_date_display(date_str: &str) -> String {
    if date_str.is_empty() {
        return "Present".to_string();
    }
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() == 2 {
        let month_name = match parts[1] {
            "01" => "Jan", "02" => "Feb", "03" => "Mar", "04" => "Apr",
            "05" => "May", "06" => "Jun", "07" => "Jul", "08" => "Aug",
            "09" => "Sep", "10" => "Oct", "11" => "Nov", "12" => "Dec",
            _ => parts[1],
        };
        format!("{} {}", month_name, parts[0])
    } else {
        date_str.to_string()
    }
}

#[component]
pub fn Timeline() -> Element {
    let experiences = data::load_experience();
    let mut selected = use_signal(|| None::<usize>);

    // Find time range
    let mut min_months = f64::MAX;
    let mut max_months = f64::MIN;
    for exp in experiences.iter() {
        let (sy, sm) = parse_date(&exp.start_date);
        let (ey, em) = parse_date(&exp.end_date);
        let start = date_to_months(sy, sm);
        let end = date_to_months(ey, em);
        if start < min_months { min_months = start; }
        if end > max_months { max_months = end; }
    }

    min_months -= 2.0;
    max_months += 2.0;
    let total_months = max_months - min_months;

    let start_year = (min_months / 12.0).floor() as i32;
    let end_year = (max_months / 12.0).ceil() as i32;

    // Assign rows (greedy, no overlap — need 1 month gap)
    let mut row_ends: Vec<f64> = Vec::new();
    let mut row_assignments: Vec<usize> = Vec::new();

    for exp in experiences.iter() {
        let (sy, sm) = parse_date(&exp.start_date);
        let (ey, em) = parse_date(&exp.end_date);
        let start = date_to_months(sy, sm);
        let end = date_to_months(ey, em);

        let mut assigned_row = None;
        for (i, row_end) in row_ends.iter_mut().enumerate() {
            if start >= *row_end + 1.0 {
                *row_end = end;
                assigned_row = Some(i);
                break;
            }
        }
        if assigned_row.is_none() {
            row_ends.push(end);
            assigned_row = Some(row_ends.len() - 1);
        }
        row_assignments.push(assigned_row.unwrap());
    }

    let num_rows = row_ends.len();
    let row_height: usize = 48;
    let bar_height: usize = 38;
    let axis_height: usize = 36;
    let _chart_height = num_rows * row_height + axis_height + 20;

    // Track width wide enough to spread bars nicely
    let track_width: usize = 2000;
    let hijack_height: usize = track_width * 2;

    use_effect(move || {
        setup_scroll_hijack("timeline-scroll-container", "timeline-track");
        setup_chart_click("timeline-scroll-container", "timeline-chart", "timeline-track");

        // Listen for auto-focus events from scroll hijack
        let document = web_sys::window().unwrap().document().unwrap();
        if let Some(container) = document.get_element_by_id("timeline-scroll-container") {
            let cb = Closure::<dyn FnMut()>::new(move || {
                let document = web_sys::window().unwrap().document().unwrap();
                if let Some(cont) = document.get_element_by_id("timeline-scroll-container") {
                    if let Some(idx_str) = cont.get_attribute("data-focused") {
                        if let Ok(idx) = idx_str.parse::<usize>() {
                            selected.set(Some(idx));
                        }
                    }
                }
            });
            container
                .add_event_listener_with_callback("barfocus", cb.as_ref().unchecked_ref())
                .unwrap();
            cb.forget();
        }
    });

    // Sort experiences by start date (newest first) for mobile view
    let mut sorted_indices: Vec<usize> = (0..experiences.len()).collect();
    sorted_indices.sort_by(|&a, &b| {
        let (ay, am) = parse_date(&experiences[b].start_date);
        let (by, bm) = parse_date(&experiences[a].start_date);
        (by, bm).cmp(&(ay, am))
    });

    rsx! {
        section { id: "experience",
            style: "background-color: {theme::DEEP_NAVY};",

            // === Desktop: horizontal scroll-hijack timeline ===
            div {
                class: "timeline-desktop",
                id: "timeline-scroll-container",
                style: "height: {hijack_height}px; position: relative;",

                div {
                    style: "position: sticky; top: 0; height: 100vh; overflow: hidden; display: flex; flex-direction: column; background-color: {theme::DEEP_NAVY}; padding: 60px 2rem 2rem 2rem;",

                    h2 {
                        style: "font-size: 4rem; font-weight: 700; color: {theme::DARK_BROWN}; text-transform: uppercase; margin-bottom: 1rem; flex-shrink: 0;",
                        "EXPERIENCE & EDUCATION"
                    }

                    div {
                        id: "timeline-chart",
                        style: "border: 3px solid {theme::DEEP_NAVY}; background: {theme::MINT_WHITE}; overflow: hidden; position: relative; flex: 1 1 auto; min-height: 0;",

                        // Cursor line — moves across the chart
                        div {
                            id: "timeline-cursor",
                            style: "position: absolute; top: 0; bottom: 0; width: 2px; background: {theme::DARK_BROWN}; z-index: 50; left: 0; pointer-events: none; transition: none;",
                            // Arrow head at bottom
                            div {
                                style: "position: absolute; bottom: 30px; left: 50%; transform: translateX(-50%); width: 0; height: 0; border-left: 6px solid transparent; border-right: 6px solid transparent; border-top: 8px solid {theme::DARK_BROWN};",
                            }
                        }

                        div {
                            id: "timeline-track",
                            style: "position: relative; width: {track_width}px; height: 100%; padding: 0.75rem 0; will-change: transform;",

                            for year in start_year..=end_year {
                                {
                                    let x_pos = (date_to_months(year, 1) - min_months) / total_months * 100.0;
                                    rsx! {
                                        div {
                                            style: "position: absolute; left: {x_pos}%; top: 0; bottom: {axis_height}px; width: 1px; background: rgba(2, 24, 43, 0.15); z-index: 1;",
                                        }
                                        div {
                                            style: "position: absolute; left: {x_pos}%; bottom: 6px; transform: translateX(-50%); font-family: {theme::FONT_MONO}; font-size: 0.8rem; color: {theme::DEEP_NAVY}; font-weight: 700; z-index: 2;",
                                            "{year}"
                                        }
                                    }
                                }
                            }

                            div {
                                style: "position: absolute; left: 0; right: 0; bottom: {axis_height}px; height: 2px; background: {theme::DEEP_NAVY}; z-index: 2;",
                            }

                            for (i, exp) in experiences.iter().enumerate() {
                                {
                                    let (sy, sm) = parse_date(&exp.start_date);
                                    let (ey, em) = parse_date(&exp.end_date);
                                    let start = date_to_months(sy, sm);
                                    let end = date_to_months(ey, em);
                                    let x_start = (start - min_months) / total_months * 100.0;
                                    let x_end = (end - min_months) / total_months * 100.0;
                                    let width = (x_end - x_start).max(1.5);
                                    let row = row_assignments[i];
                                    let top = row * row_height + 8;
                                    let color = bar_color(i, &exp.kind);
                                    let is_selected = *selected.read() == Some(i);
                                    let outline = if is_selected { format!("3px solid {}", theme::MUTED_TEAL) } else { "none".to_string() };
                                    let z_index = if is_selected { 20 } else { 3 };
                                    let scale = if is_selected { "scale(1.03)" } else { "scale(1)" };
                                    let bar_center_pct = x_start + width / 2.0;

                                    rsx! {
                                        div {
                                            "data-bar-index": "{i}",
                                            "data-bar-center": "{bar_center_pct}",
                                            "data-bar-left": "{x_start}",
                                            "data-bar-right": "{x_start + width}",
                                            class: "timeline-bar",
                                            onclick: move |_| {
                                                if *selected.read() == Some(i) {
                                                    selected.set(None);
                                                } else {
                                                    selected.set(Some(i));
                                                }
                                            },
                                            style: "position: absolute; left: {x_start}%; width: {width}%; top: {top}px; height: {bar_height}px; background: {color}; border-radius: 3px; cursor: pointer; z-index: {z_index}; transition: all 0.15s ease; outline: {outline}; outline-offset: 2px; transform: {scale}; display: flex; align-items: center; padding: 0 0.6rem; overflow: hidden; white-space: nowrap;",

                                            span {
                                                style: "font-family: {theme::FONT_MONO}; font-size: 0.65rem; color: {theme::MINT_WHITE}; font-weight: 600; overflow: hidden; text-overflow: ellipsis;",
                                                "{exp.role} — {exp.company}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if let Some(idx) = *selected.read() {
                        {
                            let exp = &experiences[idx];
                            let start_display = format_date_display(&exp.start_date);
                            let end_display = format_date_display(&exp.end_date);
                            let color = bar_color(idx, &exp.kind);

                            rsx! {
                                div {
                                    style: "margin-top: 0.75rem; border: 3px solid {theme::DEEP_NAVY}; background: {theme::MINT_WHITE}; position: relative; flex-shrink: 0; max-height: 35vh; overflow-y: auto; display: flex;",

                                    div {
                                        style: "width: 6px; flex-shrink: 0; background: {color};",
                                    }

                                    div { style: "padding: 1.25rem 1.5rem; flex: 1; min-width: 0;",
                                        div { style: "display: flex; justify-content: space-between; align-items: flex-start; flex-wrap: wrap; gap: 0.5rem;",
                                            div {
                                                h3 {
                                                    style: "font-size: 1.4rem; font-weight: 700; color: {theme::DEEP_NAVY}; margin: 0; line-height: 1.2;",
                                                    "{exp.role}"
                                                }
                                                p {
                                                    style: "font-size: 1rem; color: {theme::DEEP_NAVY}; font-weight: 700; margin-top: 0.2rem;",
                                                    "{exp.company}"
                                                }
                                            }
                                            p {
                                                style: "font-family: {theme::FONT_MONO}; font-size: 0.8rem; color: {theme::DARK_BROWN}; white-space: nowrap;",
                                                "{start_display} — {end_display}"
                                            }
                                        }

                                        p {
                                            style: "margin-top: 0.75rem; color: {theme::DEEP_NAVY}; line-height: 1.6; font-size: 0.9rem;",
                                            "{exp.details}"
                                        }

                                        div {
                                            style: "display: flex; flex-wrap: wrap; gap: 0.4rem; margin-top: 0.75rem;",
                                            for tech in exp.tech.iter() {
                                                span {
                                                    style: "font-family: {theme::FONT_MONO}; font-size: 0.65rem; border: 2px solid {theme::DEEP_NAVY}; padding: 0.1rem 0.4rem; color: {theme::DEEP_NAVY};",
                                                    "{tech}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // === Mobile: vertical card list ===
            div {
                class: "timeline-mobile",
                style: "background-color: {theme::DEEP_NAVY}; padding: 4rem 1rem 2rem 1rem;",

                h2 {
                    style: "font-size: 2rem; font-weight: 700; color: {theme::DARK_BROWN}; text-transform: uppercase; margin-bottom: 1.5rem;",
                    "EXPERIENCE & EDUCATION"
                }

                for &idx in sorted_indices.iter() {
                    {
                        let exp = &experiences[idx];
                        let start_display = format_date_display(&exp.start_date);
                        let end_display = format_date_display(&exp.end_date);
                        let color = bar_color(idx, &exp.kind);
                        let is_selected = *selected.read() == Some(idx);

                        rsx! {
                            div {
                                onclick: move |_| {
                                    if *selected.read() == Some(idx) {
                                        selected.set(None);
                                    } else {
                                        selected.set(Some(idx));
                                    }
                                },
                                style: "margin-bottom: 1rem; border-left: 5px solid {color}; background: {theme::MINT_WHITE}; cursor: pointer; transition: all 0.15s ease;",

                                // Header — always visible
                                div {
                                    style: "padding: 0.8rem 1rem;",
                                    div {
                                        style: "display: flex; justify-content: space-between; align-items: flex-start; gap: 0.5rem;",
                                        h3 {
                                            style: "font-size: 0.95rem; font-weight: 700; color: {theme::DEEP_NAVY}; margin: 0; line-height: 1.3;",
                                            "{exp.role}"
                                        }
                                        p {
                                            style: "font-family: {theme::FONT_MONO}; font-size: 0.65rem; color: {theme::DARK_BROWN}; white-space: nowrap; flex-shrink: 0;",
                                            "{start_display} — {end_display}"
                                        }
                                    }
                                    p {
                                        style: "font-size: 0.85rem; color: {theme::DEEP_NAVY}; font-weight: 600; margin-top: 0.15rem;",
                                        "{exp.company}"
                                    }
                                }

                                // Expandable detail
                                if is_selected {
                                    div {
                                        style: "padding: 0 1rem 0.8rem 1rem; border-top: 1px solid rgba(2, 24, 43, 0.1);",

                                        p {
                                            style: "margin-top: 0.6rem; color: {theme::DEEP_NAVY}; line-height: 1.6; font-size: 0.8rem;",
                                            "{exp.details}"
                                        }

                                        div {
                                            style: "display: flex; flex-wrap: wrap; gap: 0.3rem; margin-top: 0.6rem;",
                                            for tech in exp.tech.iter() {
                                                span {
                                                    style: "font-family: {theme::FONT_MONO}; font-size: 0.6rem; border: 1.5px solid {theme::DEEP_NAVY}; padding: 0.1rem 0.3rem; color: {theme::DEEP_NAVY};",
                                                    "{tech}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn setup_scroll_hijack(container_id: &str, track_id: &str) {
    let window = web_sys::window().unwrap();

    let container_id = container_id.to_string();
    let track_id = track_id.to_string();
    let last_focused: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));

    let cb = Closure::<dyn FnMut()>::new(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let container = match document.get_element_by_id(&container_id) {
            Some(el) => el,
            None => return,
        };
        let track = match document.get_element_by_id(&track_id) {
            Some(el) => el,
            None => return,
        };
        let cursor = match document.get_element_by_id("timeline-cursor") {
            Some(el) => el,
            None => return,
        };

        let rect = container.get_bounding_client_rect();
        let container_top = rect.top();
        let container_height = rect.height();
        let viewport_height = window.inner_height().unwrap().as_f64().unwrap();

        // Skip when container is hidden (mobile) or too small
        if container_height < 1.0 || container_height <= viewport_height {
            return;
        }

        let scroll_progress = -container_top / (container_height - viewport_height);
        let scroll_progress = scroll_progress.clamp(0.0, 1.0);

        let track_el: web_sys::HtmlElement = track.dyn_into().unwrap();
        let track_width = track_el.scroll_width() as f64;

        // Use cursor's parent (chart container) for actual rendered width
        let cursor_el: web_sys::HtmlElement = cursor.dyn_into().unwrap();
        let chart_container = cursor_el.parent_element().unwrap();
        let chart_html: web_sys::HtmlElement = chart_container.dyn_into().unwrap();
        let chart_inner_width = chart_html.client_width() as f64; // content + padding, excludes border
        let chart_rect = chart_html.get_bounding_client_rect();
        let _border_left = (chart_rect.width() - chart_inner_width) / 2.0;

        let max_translate = (track_width - chart_inner_width).max(0.0);

        // 3-phase scroll:
        // Phase 1: cursor moves from left (0) to center (chart_width/2), track stays still
        // Phase 2: cursor stays at center, track slides by max_translate
        // Phase 3: cursor moves from center to right (chart_inner_width), track stays at max
        let half_chart = chart_inner_width / 2.0;
        let total_virtual = half_chart + max_translate + half_chart;

        let virtual_pos = scroll_progress * total_virtual;

        let (cursor_x, translate_x);
        if virtual_pos <= half_chart {
            // Phase 1: cursor moves left to center
            cursor_x = virtual_pos;
            translate_x = 0.0;
        } else if virtual_pos <= half_chart + max_translate {
            // Phase 2: cursor at center, track slides
            cursor_x = half_chart;
            translate_x = virtual_pos - half_chart;
        } else {
            // Phase 3: cursor moves center to right
            cursor_x = half_chart + (virtual_pos - half_chart - max_translate);
            translate_x = max_translate;
        }

        // Apply track translation
        track_el
            .style()
            .set_property("transform", &format!("translateX(-{}px)", translate_x))
            .unwrap();

        // Apply cursor position
        cursor_el
            .style()
            .set_property("left", &format!("{}px", cursor_x))
            .unwrap();

        // Auto-select: when cursor intersects multiple bars, pick the one
        // whose left edge the cursor most recently crossed (latest start).
        // Use the cursor's actual rendered position to avoid any offset calculation errors.
        let cursor_rect = cursor_el.get_bounding_client_rect();
        let cursor_viewport_x = cursor_rect.left();
        let bars = document.query_selector_all(".timeline-bar").unwrap();
        let mut best_index: Option<String> = None;
        let mut best_left = f64::NEG_INFINITY;
        let mut best_dist = f64::MAX;
        let mut found_intersection = false;

        for i in 0..bars.length() {
            if let Some(bar) = bars.get(i) {
                let el: web_sys::Element = bar.dyn_into().unwrap();
                let bar_rect = el.get_bounding_client_rect();
                let bar_left = bar_rect.left();
                let bar_right = bar_rect.right();
                let intersects = cursor_viewport_x >= bar_left && cursor_viewport_x <= bar_right;

                if intersects {
                    // Among intersecting bars, prefer the one with the rightmost left edge
                    // (the most recently entered bar as cursor moves left-to-right)
                    if !found_intersection || bar_left > best_left {
                        found_intersection = true;
                        best_left = bar_left;
                        best_index = el.get_attribute("data-bar-index");
                    }
                } else if !found_intersection {
                    let bar_center = (bar_left + bar_right) / 2.0;
                    let dist = (bar_center - cursor_viewport_x).abs();
                    if dist < best_dist {
                        best_dist = dist;
                        best_index = el.get_attribute("data-bar-index");
                    }
                }
            }
        }

        // Store the focused bar index on the container element
        if let Some(ref idx) = best_index {
            let mut last = last_focused.borrow_mut();
            if last.as_ref() != Some(idx) {
                *last = Some(idx.clone());
                if let Some(cont) = document.get_element_by_id(&container_id) {
                    cont.set_attribute("data-focused", idx).unwrap_or(());
                    if let Ok(evt) = web_sys::CustomEvent::new("barfocus") {
                        cont.dispatch_event(&evt).unwrap_or(false);
                    }
                }
            }
        }
    });

    window
        .add_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref())
        .unwrap();
    cb.forget();
}

fn scroll_to_track_position(container_id: &str, track_id: &str, target_track_x: f64) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let container = match document.get_element_by_id(container_id) {
        Some(el) => el,
        None => return,
    };
    let track = match document.get_element_by_id(track_id) {
        Some(el) => el,
        None => return,
    };
    let cursor = match document.get_element_by_id("timeline-cursor") {
        Some(el) => el,
        None => return,
    };

    let track_el: web_sys::HtmlElement = track.dyn_into().unwrap();
    let track_width = track_el.scroll_width() as f64;

    let cursor_el: web_sys::HtmlElement = cursor.dyn_into().unwrap();
    let chart = cursor_el.parent_element().unwrap();
    let chart_html: web_sys::HtmlElement = chart.dyn_into().unwrap();
    let chart_inner_width = chart_html.client_width() as f64;

    let max_translate = (track_width - chart_inner_width).max(0.0);
    let half_chart = chart_inner_width / 2.0;
    let total_virtual = half_chart + max_translate + half_chart;

    // virtual_pos == cursor_track_x in all phases
    let desired_virtual = target_track_x.clamp(0.0, total_virtual);
    let desired_progress = desired_virtual / total_virtual;

    let container_rect = container.get_bounding_client_rect();
    let container_height = container_rect.height();
    let viewport_height = window.inner_height().unwrap().as_f64().unwrap();

    if container_height <= viewport_height {
        return;
    }

    let container_offset_top = container_rect.top() + window.scroll_y().unwrap_or(0.0);
    let target_scroll = container_offset_top + desired_progress * (container_height - viewport_height);

    let opts = web_sys::ScrollToOptions::new();
    opts.set_top(target_scroll);
    opts.set_behavior(web_sys::ScrollBehavior::Smooth);
    window.scroll_to_with_scroll_to_options(&opts);
}

fn get_current_translate_x(track_id: &str) -> f64 {
    let document = web_sys::window().unwrap().document().unwrap();
    if let Some(track) = document.get_element_by_id(track_id) {
        let track_el: web_sys::HtmlElement = track.dyn_into().unwrap();
        let transform = track_el.style().get_property_value("transform").unwrap_or_default();
        // Parse "translateX(-500px)" → 500.0
        if let Some(start) = transform.find("translateX(") {
            let rest = &transform[start + 11..];
            if let Some(end) = rest.find("px") {
                if let Ok(val) = rest[..end].parse::<f64>() {
                    return val.abs();
                }
            }
        }
    }
    0.0
}

fn setup_chart_click(container_id: &str, chart_id: &str, track_id: &str) {
    let document = web_sys::window().unwrap().document().unwrap();

    let chart = match document.get_element_by_id(chart_id) {
        Some(el) => el,
        None => return,
    };

    let container_id = container_id.to_string();
    let track_id = track_id.to_string();
    let is_dragging = Rc::new(RefCell::new(false));

    // Click handler — move cursor to clicked position
    {
        let container_id = container_id.clone();
        let track_id = track_id.clone();
        let chart_id_str = chart_id.to_string();

        let click_cb = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            let document = web_sys::window().unwrap().document().unwrap();
            let chart = match document.get_element_by_id(&chart_id_str) {
                Some(el) => el,
                None => return,
            };
            let chart_html: web_sys::HtmlElement = chart.dyn_into().unwrap();
            let chart_rect = chart_html.get_bounding_client_rect();
            let chart_inner_width = chart_html.client_width() as f64;
            let border_left = (chart_rect.width() - chart_inner_width) / 2.0;

            let click_x = (e.client_x() as f64 - chart_rect.left() - border_left).clamp(0.0, chart_inner_width);
            let translate_x = get_current_translate_x(&track_id);
            let target_track_x = click_x + translate_x;

            scroll_to_track_position(&container_id, &track_id, target_track_x);
        });

        chart
            .add_event_listener_with_callback("click", click_cb.as_ref().unchecked_ref())
            .unwrap();
        click_cb.forget();
    }

    // Drag handlers — mousedown starts drag, mousemove updates, mouseup ends
    {
        let is_dragging_down = is_dragging.clone();
        let mousedown_cb = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_e: web_sys::MouseEvent| {
            *is_dragging_down.borrow_mut() = true;
        });
        chart
            .add_event_listener_with_callback("mousedown", mousedown_cb.as_ref().unchecked_ref())
            .unwrap();
        mousedown_cb.forget();
    }

    {
        let is_dragging_up = is_dragging.clone();
        let mouseup_cb = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |_e: web_sys::MouseEvent| {
            *is_dragging_up.borrow_mut() = false;
        });
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("mouseup", mouseup_cb.as_ref().unchecked_ref())
            .unwrap();
        mouseup_cb.forget();
    }

    {
        let is_dragging_move = is_dragging;
        let container_id = container_id.clone();
        let track_id = track_id.clone();
        let chart_id_str = chart_id.to_string();

        let mousemove_cb = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            if !*is_dragging_move.borrow() {
                return;
            }
            let document = web_sys::window().unwrap().document().unwrap();
            let chart = match document.get_element_by_id(&chart_id_str) {
                Some(el) => el,
                None => return,
            };
            let chart_html: web_sys::HtmlElement = chart.dyn_into().unwrap();
            let chart_rect = chart_html.get_bounding_client_rect();
            let chart_inner_width = chart_html.client_width() as f64;
            let border_left = (chart_rect.width() - chart_inner_width) / 2.0;

            let click_x = (e.client_x() as f64 - chart_rect.left() - border_left).clamp(0.0, chart_inner_width);
            let translate_x = get_current_translate_x(&track_id);
            let target_track_x = click_x + translate_x;

            // Use instant scroll for dragging (no smooth animation lag)
            let window = web_sys::window().unwrap();
            let container = match document.get_element_by_id(&container_id) {
                Some(el) => el,
                None => return,
            };
            let track = match document.get_element_by_id(&track_id) {
                Some(el) => el,
                None => return,
            };
            let cursor = match document.get_element_by_id("timeline-cursor") {
                Some(el) => el,
                None => return,
            };

            let track_el: web_sys::HtmlElement = track.dyn_into().unwrap();
            let track_width = track_el.scroll_width() as f64;
            let cursor_el: web_sys::HtmlElement = cursor.dyn_into().unwrap();
            let chart2 = cursor_el.parent_element().unwrap();
            let chart2_html: web_sys::HtmlElement = chart2.dyn_into().unwrap();
            let ciw = chart2_html.client_width() as f64;

            let max_translate = (track_width - ciw).max(0.0);
            let half_chart = ciw / 2.0;
            let total_virtual = half_chart + max_translate + half_chart;
            let desired_virtual = target_track_x.clamp(0.0, total_virtual);
            let desired_progress = desired_virtual / total_virtual;

            let container_rect = container.get_bounding_client_rect();
            let container_height = container_rect.height();
            let viewport_height = window.inner_height().unwrap().as_f64().unwrap();
            if container_height <= viewport_height { return; }

            let container_offset_top = container_rect.top() + window.scroll_y().unwrap_or(0.0);
            let target_scroll = container_offset_top + desired_progress * (container_height - viewport_height);

            let opts = web_sys::ScrollToOptions::new();
            opts.set_top(target_scroll);
            opts.set_behavior(web_sys::ScrollBehavior::Instant);
            window.scroll_to_with_scroll_to_options(&opts);
        });

        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("mousemove", mousemove_cb.as_ref().unchecked_ref())
            .unwrap();
        mousemove_cb.forget();
    }
}
