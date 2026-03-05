use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::data;
use crate::styles::theme;

#[component]
pub fn Timeline() -> Element {
    let experiences = data::load_experience();
    let item_count = experiences.len();
    let total_width = item_count * 450 + 200;

    use_effect(move || {
        setup_scroll_hijack("timeline-scroll-container", "timeline-track");
    });

    rsx! {
        section { id: "experience",
            div {
                id: "timeline-scroll-container",
                style: "height: {total_width}px; position: relative;",
                div {
                    style: "position: sticky; top: 0; height: 100vh; overflow: hidden; display: flex; align-items: center;",
                    h2 {
                        style: "position: absolute; top: 2rem; left: 2rem; font-size: 5rem; font-weight: 700; color: {theme::DEEP_NAVY}; text-transform: uppercase; z-index: 2;",
                        "EXPERIENCE"
                    }
                    div {
                        id: "timeline-track",
                        style: "display: flex; gap: 50px; padding: 0 4rem; padding-top: 4rem; will-change: transform;",
                        for exp in experiences.iter() {
                            TimelineCard { experience: exp.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TimelineCard(experience: shared::Experience) -> Element {
    let mut expanded = use_signal(|| false);
    let end_display = if experience.end_date.is_empty() {
        "Present".to_string()
    } else {
        experience.end_date.clone()
    };

    rsx! {
        div {
            onclick: move |_| expanded.toggle(),
            style: "min-width: 400px; max-width: 400px; border: 3px solid {theme::DEEP_NAVY}; padding: 2rem; background: {theme::MINT_WHITE}; cursor: pointer; transition: transform 0.2s;",
            p {
                style: "font-family: {theme::FONT_MONO}; font-size: 0.9rem; color: {theme::MUTED_TEAL};",
                "{experience.start_date} — {end_display}"
            }
            h3 {
                style: "font-size: 1.8rem; font-weight: 700; color: {theme::DEEP_NAVY}; margin: 0.5rem 0;",
                "{experience.role}"
            }
            p {
                style: "font-size: 1.2rem; color: {theme::DARK_BROWN}; font-weight: 700;",
                "{experience.company}"
            }
            p {
                style: "margin-top: 1rem; color: {theme::DEEP_NAVY};",
                "{experience.summary}"
            }
            if *expanded.read() {
                div {
                    style: "margin-top: 1rem; padding-top: 1rem; border-top: 2px solid {theme::MUTED_TEAL};",
                    p {
                        style: "color: {theme::DEEP_NAVY}; line-height: 1.6;",
                        "{experience.details}"
                    }
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 0.5rem; margin-top: 1rem;",
                        for tech in experience.tech.iter() {
                            span {
                                style: "font-family: {theme::FONT_MONO}; font-size: 0.75rem; border: 2px solid {theme::DEEP_NAVY}; padding: 0.2rem 0.5rem;",
                                "{tech}"
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

        let rect = container.get_bounding_client_rect();
        let container_top = rect.top();
        let container_height = rect.height();
        let viewport_height = window.inner_height().unwrap().as_f64().unwrap();

        let scroll_progress = -container_top / (container_height - viewport_height);
        let scroll_progress = scroll_progress.clamp(0.0, 1.0);

        let track_el: web_sys::HtmlElement = track.dyn_into().unwrap();
        let track_width = track_el.scroll_width() as f64;
        let viewport_width = window.inner_width().unwrap().as_f64().unwrap();

        let max_translate = (track_width - viewport_width + 80.0).max(0.0);
        let translate_x = scroll_progress * max_translate;

        track_el
            .style()
            .set_property("transform", &format!("translateX(-{}px)", translate_x))
            .unwrap();
    });

    window
        .add_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref())
        .unwrap();
    cb.forget();
}
