use super::track::{get_current_translate_x, scroll_to_track_position, track_scroll_target};
use crate::libs::EventListener;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

pub fn setup_chart_click(container_id: &str, chart_id: &str, track_id: &str) -> Vec<EventListener> {
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
