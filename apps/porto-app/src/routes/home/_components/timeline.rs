use crate::data;
use crate::libs::EventListener;
use crate::styles::theme;
use dioxus::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

const TIMELINE_CONTAINER_ID: &str = "timeline-scroll-container";
const TIMELINE_TRACK_ID: &str = "timeline-track";
const TIMELINE_CHART_ID: &str = "timeline-chart";
const TIMELINE_CURSOR_ID: &str = "timeline-cursor";
const BAR_FOCUS_EVENT: &str = "barfocus";
const FOCUSED_ATTRIBUTE: &str = "data-focused";

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
        const EDU_COLORS: &[&str] = &["#2D6A4F", "#1a5276", "#4A6741"];
        return EDU_COLORS[index % EDU_COLORS.len()];
    }
    const COLORS: &[&str] = &[
        "#02182B", "#D65108", "#2D6A4F", "#6B4C8A", "#B85C38", "#1a5276",
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
            "01" => "Jan",
            "02" => "Feb",
            "03" => "Mar",
            "04" => "Apr",
            "05" => "May",
            "06" => "Jun",
            "07" => "Jul",
            "08" => "Aug",
            "09" => "Sep",
            "10" => "Oct",
            "11" => "Nov",
            "12" => "Dec",
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

    let mut min_months = f64::MAX;
    let mut max_months = f64::MIN;
    for exp in experiences.iter() {
        let (sy, sm) = parse_date(&exp.start_date);
        let (ey, em) = parse_date(&exp.end_date);
        let start = date_to_months(sy, sm);
        let end = date_to_months(ey, em);
        if start < min_months {
            min_months = start;
        }
        if end > max_months {
            max_months = end;
        }
    }

    min_months -= 2.0;
    max_months += 2.0;
    let total_months = max_months - min_months;

    let start_year = (min_months / 12.0).floor() as i32;
    let end_year = (max_months / 12.0).ceil() as i32;

    let mut row_ends: Vec<f64> = Vec::new();
    let mut row_assignments: Vec<usize> = Vec::new();

    for exp in experiences.iter() {
        let (sy, sm) = parse_date(&exp.start_date);
        let (ey, em) = parse_date(&exp.end_date);
        let start = date_to_months(sy, sm);
        let end = date_to_months(ey, em);

        let existing_row = row_ends.iter_mut().position(|row_end| {
            let fits = start >= *row_end + 1.0;
            if fits {
                *row_end = end;
            }
            fits
        });

        let assigned_row = match existing_row {
            Some(row) => row,
            None => {
                row_ends.push(end);
                row_ends.len() - 1
            }
        };
        row_assignments.push(assigned_row);
    }

    let num_rows = row_ends.len();
    let row_height: usize = 48;
    let bar_height: usize = 38;
    let axis_height: usize = 36;
    let _chart_height = num_rows * row_height + axis_height + 20;

    let track_width: usize = 2000;
    let hijack_height: usize = track_width * 2;

    let listeners: Rc<RefCell<Vec<EventListener>>> = use_hook(|| Rc::new(RefCell::new(Vec::new())));

    use_effect({
        let listeners = listeners.clone();
        move || {
            let mut owned = listeners.borrow_mut();
            if !owned.is_empty() {
                return;
            }

            owned.extend(setup_scroll_hijack(
                TIMELINE_CONTAINER_ID,
                TIMELINE_TRACK_ID,
            ));
            owned.extend(setup_chart_click(
                TIMELINE_CONTAINER_ID,
                TIMELINE_CHART_ID,
                TIMELINE_TRACK_ID,
            ));

            let container = web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| document.get_element_by_id(TIMELINE_CONTAINER_ID));

            if let Some(container) = container {
                owned.extend(EventListener::new(
                    container.as_ref(),
                    BAR_FOCUS_EVENT,
                    move |_| {
                        let focused = web_sys::window()
                            .and_then(|window| window.document())
                            .and_then(|document| document.get_element_by_id(TIMELINE_CONTAINER_ID))
                            .and_then(|cont| cont.get_attribute(FOCUSED_ATTRIBUTE))
                            .and_then(|idx| idx.parse::<usize>().ok());

                        if let Some(idx) = focused {
                            selected.set(Some(idx));
                        }
                    },
                ));
            }
        }
    });

    let mut sorted_indices: Vec<usize> = (0..experiences.len()).collect();
    sorted_indices.sort_by(|&a, &b| {
        let (ay, am) = parse_date(&experiences[b].start_date);
        let (by, bm) = parse_date(&experiences[a].start_date);
        (by, bm).cmp(&(ay, am))
    });

    rsx! {
        section { id: "experience",
            style: "background-color: {theme::DEEP_NAVY};",

            div {
                class: "timeline-desktop",
                id: TIMELINE_CONTAINER_ID,
                style: "height: {hijack_height}px; position: relative;",

                div {
                    style: "position: sticky; top: 0; height: 100vh; overflow: hidden; display: flex; flex-direction: column; background-color: {theme::DEEP_NAVY}; padding: 60px 2rem 2rem 2rem;",

                    h2 {
                        style: "font-size: 4rem; font-weight: 700; color: {theme::DARK_BROWN}; text-transform: uppercase; margin-bottom: 1rem; flex-shrink: 0;",
                        "EXPERIENCE & EDUCATION"
                    }

                    div {
                        id: TIMELINE_CHART_ID,
                        style: "border: 3px solid {theme::DEEP_NAVY}; background: {theme::MINT_WHITE}; overflow: hidden; position: relative; flex: 1 1 auto; min-height: 0;",

                        div {
                            id: TIMELINE_CURSOR_ID,
                            style: "position: absolute; top: 0; bottom: 0; width: 2px; background: {theme::DARK_BROWN}; z-index: 50; left: 0; pointer-events: none; transition: none;",
                            div {
                                style: "position: absolute; bottom: 30px; left: 50%; transform: translateX(-50%); width: 0; height: 0; border-left: 6px solid transparent; border-right: 6px solid transparent; border-top: 8px solid {theme::DARK_BROWN};",
                            }
                        }

                        div {
                            id: TIMELINE_TRACK_ID,
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

fn setup_scroll_hijack(container_id: &str, track_id: &str) -> Option<EventListener> {
    let window = web_sys::window()?;

    let container_id = container_id.to_string();
    let track_id = track_id.to_string();
    let last_focused: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));

    let handler = move |_event: web_sys::Event| {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };

        let container = match document.get_element_by_id(&container_id) {
            Some(el) => el,
            None => return,
        };
        let track = match document.get_element_by_id(&track_id) {
            Some(el) => el,
            None => return,
        };
        let cursor = match document.get_element_by_id(TIMELINE_CURSOR_ID) {
            Some(el) => el,
            None => return,
        };

        let rect = container.get_bounding_client_rect();
        let container_top = rect.top();
        let container_height = rect.height();
        let Some(viewport_height) = window.inner_height().ok().and_then(|value| value.as_f64())
        else {
            return;
        };

        if container_height < 1.0 || container_height <= viewport_height {
            return;
        }

        let scroll_progress = -container_top / (container_height - viewport_height);
        let scroll_progress = scroll_progress.clamp(0.0, 1.0);

        let Ok(track_el) = track.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };
        let track_width = track_el.scroll_width() as f64;

        let Ok(cursor_el) = cursor.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };
        let Some(chart_container) = cursor_el.parent_element() else {
            return;
        };
        let Ok(chart_html) = chart_container.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };
        let chart_inner_width = chart_html.client_width() as f64;
        let chart_rect = chart_html.get_bounding_client_rect();
        let _border_left = (chart_rect.width() - chart_inner_width) / 2.0;

        let max_translate = (track_width - chart_inner_width).max(0.0);

        let half_chart = chart_inner_width / 2.0;
        let total_virtual = half_chart + max_translate + half_chart;

        let virtual_pos = scroll_progress * total_virtual;

        let (cursor_x, translate_x);
        if virtual_pos <= half_chart {
            cursor_x = virtual_pos;
            translate_x = 0.0;
        } else if virtual_pos <= half_chart + max_translate {
            cursor_x = half_chart;
            translate_x = virtual_pos - half_chart;
        } else {
            cursor_x = half_chart + (virtual_pos - half_chart - max_translate);
            translate_x = max_translate;
        }

        let _ = track_el
            .style()
            .set_property("transform", &format!("translateX(-{}px)", translate_x));

        let _ = cursor_el
            .style()
            .set_property("left", &format!("{}px", cursor_x));

        let cursor_rect = cursor_el.get_bounding_client_rect();
        let cursor_viewport_x = cursor_rect.left();
        let Ok(bars) = document.query_selector_all(".timeline-bar") else {
            return;
        };
        let mut best_index: Option<String> = None;
        let mut best_left = f64::NEG_INFINITY;
        let mut best_dist = f64::MAX;
        let mut found_intersection = false;

        for i in 0..bars.length() {
            if let Some(bar) = bars.get(i) {
                let Ok(el) = bar.dyn_into::<web_sys::Element>() else {
                    continue;
                };
                let bar_rect = el.get_bounding_client_rect();
                let bar_left = bar_rect.left();
                let bar_right = bar_rect.right();
                let intersects = cursor_viewport_x >= bar_left && cursor_viewport_x <= bar_right;

                if intersects {
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

        if let Some(idx) = best_index {
            let focus_changed = {
                let mut last = last_focused.borrow_mut();
                let changed = last.as_deref() != Some(idx.as_str());
                if changed {
                    *last = Some(idx.clone());
                }
                changed
            };
            if focus_changed && let Some(cont) = document.get_element_by_id(&container_id) {
                let _ = cont.set_attribute(FOCUSED_ATTRIBUTE, &idx);
                if let Ok(evt) = web_sys::CustomEvent::new(BAR_FOCUS_EVENT) {
                    let _ = cont.dispatch_event(&evt);
                }
            }
        }
    };

    EventListener::new(window.as_ref(), "scroll", handler)
}

fn track_scroll_target(container_id: &str, track_id: &str, target_track_x: f64) -> Option<f64> {
    let window = web_sys::window()?;
    let document = window.document()?;

    let container = document.get_element_by_id(container_id)?;
    let track_el = document
        .get_element_by_id(track_id)?
        .dyn_into::<web_sys::HtmlElement>()
        .ok()?;
    let cursor_el = document
        .get_element_by_id(TIMELINE_CURSOR_ID)?
        .dyn_into::<web_sys::HtmlElement>()
        .ok()?;
    let chart_html = cursor_el
        .parent_element()?
        .dyn_into::<web_sys::HtmlElement>()
        .ok()?;

    let track_width = track_el.scroll_width() as f64;
    let chart_inner_width = chart_html.client_width() as f64;
    let max_translate = (track_width - chart_inner_width).max(0.0);
    let half_chart = chart_inner_width / 2.0;
    let total_virtual = half_chart + max_translate + half_chart;
    if total_virtual <= 0.0 {
        return None;
    }

    let desired_progress = target_track_x.clamp(0.0, total_virtual) / total_virtual;

    let container_rect = container.get_bounding_client_rect();
    let container_height = container_rect.height();
    let viewport_height = window.inner_height().ok()?.as_f64()?;
    if container_height <= viewport_height {
        return None;
    }

    let container_offset_top = container_rect.top() + window.scroll_y().unwrap_or(0.0);
    Some(container_offset_top + desired_progress * (container_height - viewport_height))
}

fn scroll_to_track_position(container_id: &str, track_id: &str, target_track_x: f64) {
    let Some(target_scroll) = track_scroll_target(container_id, track_id, target_track_x) else {
        return;
    };
    let Some(window) = web_sys::window() else {
        return;
    };

    let opts = web_sys::ScrollToOptions::new();
    opts.set_top(target_scroll);
    opts.set_behavior(web_sys::ScrollBehavior::Smooth);
    window.scroll_to_with_scroll_to_options(&opts);
}

fn get_current_translate_x(track_id: &str) -> f64 {
    const TRANSLATE_PREFIX: &str = "translateX(";

    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return 0.0;
    };
    let Some(track_el) = document
        .get_element_by_id(track_id)
        .and_then(|track| track.dyn_into::<web_sys::HtmlElement>().ok())
    else {
        return 0.0;
    };

    let transform = track_el
        .style()
        .get_property_value("transform")
        .unwrap_or_default();

    let Some(start) = transform.find(TRANSLATE_PREFIX) else {
        return 0.0;
    };
    let rest = &transform[start + TRANSLATE_PREFIX.len()..];
    let Some(end) = rest.find("px") else {
        return 0.0;
    };

    rest[..end].parse::<f64>().map(f64::abs).unwrap_or(0.0)
}

fn setup_chart_click(container_id: &str, chart_id: &str, track_id: &str) -> Vec<EventListener> {
    let mut listeners = Vec::new();
    let Some(window) = web_sys::window() else {
        return listeners;
    };
    let Some(document) = window.document() else {
        return listeners;
    };
    let Some(chart) = document.get_element_by_id(chart_id) else {
        return listeners;
    };

    let container_id = container_id.to_string();
    let track_id = track_id.to_string();
    let chart_id = chart_id.to_string();
    let is_dragging = Rc::new(RefCell::new(false));

    {
        let container_id = container_id.clone();
        let track_id = track_id.clone();
        let chart_id = chart_id.clone();

        listeners.extend(EventListener::new(chart.as_ref(), "click", move |event| {
            let Some(mouse_event) = event.dyn_ref::<web_sys::MouseEvent>() else {
                return;
            };
            let Some(target_track_x) = pointer_track_position(&chart_id, &track_id, mouse_event)
            else {
                return;
            };
            scroll_to_track_position(&container_id, &track_id, target_track_x);
        }));
    }

    {
        let is_dragging_down = is_dragging.clone();
        listeners.extend(EventListener::new(
            chart.as_ref(),
            "mousedown",
            move |_event| {
                *is_dragging_down.borrow_mut() = true;
            },
        ));
    }

    {
        let is_dragging_up = is_dragging.clone();
        listeners.extend(EventListener::new(
            window.as_ref(),
            "mouseup",
            move |_event| {
                *is_dragging_up.borrow_mut() = false;
            },
        ));
    }

    {
        let is_dragging_move = is_dragging;
        let container_id = container_id.clone();
        let track_id = track_id.clone();
        let chart_id = chart_id.clone();

        listeners.extend(EventListener::new(
            window.as_ref(),
            "mousemove",
            move |event| {
                if !*is_dragging_move.borrow() {
                    return;
                }
                let Some(mouse_event) = event.dyn_ref::<web_sys::MouseEvent>() else {
                    return;
                };
                let Some(target_track_x) =
                    pointer_track_position(&chart_id, &track_id, mouse_event)
                else {
                    return;
                };
                drag_to_track_position(&container_id, &track_id, target_track_x);
            },
        ));
    }

    listeners
}

fn pointer_track_position(
    chart_id: &str,
    track_id: &str,
    event: &web_sys::MouseEvent,
) -> Option<f64> {
    let document = web_sys::window()?.document()?;
    let chart_html = document
        .get_element_by_id(chart_id)?
        .dyn_into::<web_sys::HtmlElement>()
        .ok()?;

    let chart_rect = chart_html.get_bounding_client_rect();
    let chart_inner_width = chart_html.client_width() as f64;
    let border_left = (chart_rect.width() - chart_inner_width) / 2.0;
    let pointer_x =
        (event.client_x() as f64 - chart_rect.left() - border_left).clamp(0.0, chart_inner_width);

    Some(pointer_x + get_current_translate_x(track_id))
}

fn drag_to_track_position(container_id: &str, track_id: &str, target_track_x: f64) {
    let Some(target_scroll) = track_scroll_target(container_id, track_id, target_track_x) else {
        return;
    };
    let Some(window) = web_sys::window() else {
        return;
    };

    let opts = web_sys::ScrollToOptions::new();
    opts.set_top(target_scroll);
    opts.set_behavior(web_sys::ScrollBehavior::Instant);
    window.scroll_to_with_scroll_to_options(&opts);
}
