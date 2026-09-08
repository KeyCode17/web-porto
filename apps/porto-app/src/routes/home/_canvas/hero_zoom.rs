use crate::libs::EventListener;
use crate::routes::home::_constants::{
    ABOUT_CONTENT_ID, HERO_NAME_ID, HERO_OVERLAY_ID, HERO_SUBTITLE_ID, HERO_TEXT_WRAPPER_ID,
    HERO_ZOOM_CONTAINER_ID,
};
use wasm_bindgen::JsCast;

const MAX_NAME_SCALE: f64 = 19.0;
const MAX_LAYER_VIEWPORT_MULTIPLE: f64 = 4.0;
const MIN_NAME_SCALE_CAP: f64 = 2.0;
const SUBTITLE_FADE_RATE: f64 = 4.0;
const OVERLAY_FADE_SPAN: f64 = 0.6;
const HERO_FADE_START: f64 = 0.7;
const HERO_FADE_SPAN: f64 = 0.3;
const ABOUT_FADE_START: f64 = 0.4;
const ABOUT_FADE_SPAN: f64 = 0.4;
const ABOUT_INTERACTIVE_THRESHOLD: f64 = 0.5;

pub fn init_hero_zoom() -> Option<EventListener> {
    let window = web_sys::window()?;
    apply_hero_zoom();
    EventListener::new(window.as_ref(), "scroll", |_| apply_hero_zoom())
}

// hero-name carries will-change: transform, so it is promoted to its own
// composited layer and rasterised at its full scaled size. Past the GPU's
// MAX_TEXTURE_SIZE the raster fails, and sibling layers - including the fixed
// navbar - can stop painting while the DOM still looks correct. offsetWidth is
// the untransformed layout width, so capping against it keeps the layer within
// a few viewports, which is far below any texture limit and visually identical:
// the text has already left the screen by ~2x.
fn name_scale_cap(document: &web_sys::Document, viewport_width: f64) -> f64 {
    let base_width = document
        .get_element_by_id(HERO_NAME_ID)
        .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
        .map(|el| el.offset_width() as f64)
        .filter(|width| *width > 0.0);

    match base_width {
        Some(width) => {
            (viewport_width * MAX_LAYER_VIEWPORT_MULTIPLE / width).max(MIN_NAME_SCALE_CAP)
        }
        None => MIN_NAME_SCALE_CAP,
    }
}

fn set_style(document: &web_sys::Document, id: &str, property: &str, value: &str) {
    let element = document
        .get_element_by_id(id)
        .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok());

    if let Some(element) = element {
        let _ = element.style().set_property(property, value);
    }
}

fn apply_hero_zoom() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(container) = document.get_element_by_id(HERO_ZOOM_CONTAINER_ID) else {
        return;
    };
    let Some(viewport_height) = window.inner_height().ok().and_then(|value| value.as_f64()) else {
        return;
    };
    let Some(viewport_width) = window.inner_width().ok().and_then(|value| value.as_f64()) else {
        return;
    };

    let rect = container.get_bounding_client_rect();
    let scrollable_height = rect.height() - viewport_height;
    if scrollable_height <= 0.0 {
        return;
    }

    let progress = (-rect.top() / scrollable_height).clamp(0.0, 1.0);
    let hero_progress = (progress / 0.5).clamp(0.0, 1.0);
    let eased = hero_progress * hero_progress * hero_progress;

    let scale = (1.0 + eased * MAX_NAME_SCALE).min(name_scale_cap(&document, viewport_width));
    let subtitle_opacity = (1.0 - hero_progress * SUBTITLE_FADE_RATE).clamp(0.0, 1.0);
    let overlay_opacity = (hero_progress / OVERLAY_FADE_SPAN).clamp(0.0, 1.0);
    let hero_text_opacity =
        (1.0 - (hero_progress - HERO_FADE_START) / HERO_FADE_SPAN).clamp(0.0, 1.0);
    let about_progress = ((progress - ABOUT_FADE_START) / ABOUT_FADE_SPAN).clamp(0.0, 1.0);

    set_style(
        &document,
        HERO_NAME_ID,
        "transform",
        &format!("scale({})", scale),
    );
    set_style(
        &document,
        HERO_SUBTITLE_ID,
        "opacity",
        &subtitle_opacity.to_string(),
    );
    set_style(
        &document,
        HERO_OVERLAY_ID,
        "opacity",
        &overlay_opacity.to_string(),
    );
    set_style(
        &document,
        HERO_TEXT_WRAPPER_ID,
        "opacity",
        &hero_text_opacity.to_string(),
    );
    set_style(
        &document,
        ABOUT_CONTENT_ID,
        "opacity",
        &about_progress.to_string(),
    );

    let pointer_events = match about_progress > ABOUT_INTERACTIVE_THRESHOLD {
        true => "auto",
        false => "none",
    };
    set_style(
        &document,
        ABOUT_CONTENT_ID,
        "pointer-events",
        pointer_events,
    );
}
