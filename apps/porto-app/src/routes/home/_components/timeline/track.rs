use super::constants::TIMELINE_CURSOR_ID;
use wasm_bindgen::JsCast;

pub fn track_scroll_target(container_id: &str, track_id: &str, target_track_x: f64) -> Option<f64> {
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

pub fn scroll_to_track_position(container_id: &str, track_id: &str, target_track_x: f64) {
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

pub fn get_current_translate_x(track_id: &str) -> f64 {
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
