use super::constants::{BAR_FOCUS_EVENT, FOCUSED_ATTRIBUTE, TIMELINE_CURSOR_ID};
use crate::libs::EventListener;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

pub fn setup_scroll_hijack(container_id: &str, track_id: &str) -> Option<EventListener> {
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
