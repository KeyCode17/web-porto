pub mod _canvas;
pub mod _constants;
pub mod detail;

use self::_canvas::{Direction, ShootingRange, start_range};
use self::_constants::{
    BELT_PITCH_PX, BELT_STAGGER_MS, LOAD_DELAY_MS, RANGE_CANVAS_ID, RANGE_CONTAINER_ID,
    RANGE_TAIL_VH, SCROLL_PER_ROUND_VH,
};
use crate::data;
use crate::libs::EventListener;
use crate::libs::{on_escape, sleep_ms};
use crate::styles::theme;
use dioxus::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use web_sys::{ScrollBehavior, ScrollToOptions};

fn category_label(cat: &str) -> &'static str {
    match cat {
        "ai" => "AI / ML",
        "data" => "Data Analysis",
        "web" => "Web Development",
        "systems" => "Systems",
        _ => "Other",
    }
}

// Clicking a round scrolls the page to it instead of jumping the state: the
// scroll handler then fires every round the belt passes on the way, so a click
// on a distant round reads as a burst rather than a teleport.
fn scroll_to_round(index: usize, total: usize) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(container) = window
        .document()
        .and_then(|d| d.get_element_by_id(RANGE_CONTAINER_ID))
    else {
        return;
    };
    let Some(viewport) = window.inner_height().ok().and_then(|v| v.as_f64()) else {
        return;
    };

    let rect = container.get_bounding_client_rect();
    let travel = rect.height() - viewport;
    if travel <= 0.0 || total == 0 {
        return;
    }

    let top = rect.top() + window.scroll_y().unwrap_or(0.0);
    let progress = (index as f64 + 0.5) / total as f64;

    let opts = ScrollToOptions::new();
    opts.set_top(top + progress * travel);
    opts.set_behavior(ScrollBehavior::Smooth);
    window.scroll_to_with_scroll_to_options(&opts);
}

#[component]
pub fn Projects() -> Element {
    let projects = data::load_projects();
    let total = projects.len();
    let mut loaded = use_signal(|| false);
    let mut chambered = use_signal(|| 0usize);
    let mut expanded: Signal<Option<usize>> = use_signal(|| None);
    let mut hovered: Signal<Option<usize>> = use_signal(|| None);

    use_future(move || async move {
        sleep_ms(LOAD_DELAY_MS).await;
        loaded.set(true);
    });

    let _escape_listener = use_hook(|| Rc::new(on_escape(move || expanded.set(None))));

    let range: Rc<RefCell<Option<ShootingRange>>> = use_hook(|| Rc::new(RefCell::new(None)));

    let _scroll = use_hook(|| {
        Rc::new(web_sys::window().and_then(|window| {
            EventListener::new(window.as_ref(), "scroll", move |_| {
                let Some(container) = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.get_element_by_id(RANGE_CONTAINER_ID))
                else {
                    return;
                };
                let Some(viewport) = web_sys::window()
                    .and_then(|w| w.inner_height().ok())
                    .and_then(|v| v.as_f64())
                else {
                    return;
                };

                let rect = container.get_bounding_client_rect();
                let travel = rect.height() - viewport;
                if travel <= 0.0 {
                    return;
                }

                let progress = (-rect.top() / travel).clamp(0.0, 1.0);
                let index = ((progress * total as f64) as usize).min(total - 1);
                if *chambered.peek() != index {
                    chambered.set(index);
                }
            })
        }))
    });

    use_effect({
        let range = range.clone();
        move || {
            if range.borrow().is_none() {
                *range.borrow_mut() = start_range(RANGE_CANVAS_ID);
            }
        }
    });

    // Reading the signal here is what subscribes the effect: every time a round
    // is chambered the effect re-runs and the range fires once.
    let last_round = use_hook(|| Rc::new(Cell::new(0usize)));

    use_effect({
        let range = range.clone();
        let last_round = last_round.clone();
        move || {
            let round = *chambered.read();
            let previous = last_round.replace(round);
            let shots = round.abs_diff(previous);
            let direction = match round >= previous {
                true => Direction::Forward,
                false => Direction::Rewind,
            };
            if let Some(active_range) = range.borrow().as_ref() {
                active_range.queue_shots(shots, direction);
            }
        }
    });

    let is_loaded = *loaded.read();
    let current_expanded = *expanded.read();
    let current = (*chambered.read()).min(projects.len().saturating_sub(1));
    let n = projects.len();
    let shift = -((current as f64) - (n.saturating_sub(1) as f64) / 2.0) * BELT_PITCH_PX;
    let active = &projects[current];
    let active_title = active.title.clone();
    let active_desc = active.short_description.clone();
    let active_cat = category_label(&active.category);
    let active_tech = active.tech_stack.clone();
    let peek = (*hovered.read()).map(|i| {
        let project = &projects[i];
        (
            i,
            project.title.clone(),
            project.tech_stack.join(" \u{00B7} "),
            (i as f64 - current as f64) * BELT_PITCH_PX,
        )
    });

    rsx! {
        div { style: "background: {theme::DEEP_NAVY}; position: relative;",
            div {
                id: RANGE_CONTAINER_ID,
                class: "range-scroll",
                style: "height: {total * SCROLL_PER_ROUND_VH + RANGE_TAIL_VH}vh;",

                div { class: "range-stage",
                    canvas { id: RANGE_CANVAS_ID, class: "range-canvas" }

                    div { class: "range-head",
                        h1 { class: "range-heading", "PROJECTS" }
                        p { class: "range-sub", "SCROLL TO FIRE" }
                    }

                    div { class: "range-hud",
                        span { class: "range-hud-line", "ROUND {current + 1:02} / {n:02}" }
                        span { class: "range-hud-line", "TARGET {active_cat}" }
                    }

                    div { class: "range-readout",
                        h2 { class: "range-title", "{active_title}" }
                        p { class: "range-desc", "{active_desc}" }
                        div { class: "range-tags",
                            for tech in active_tech.iter() {
                                span { class: "range-tag", "{tech}" }
                            }
                        }
                        button {
                            class: "range-open",
                            r#type: "button",
                            onclick: move |_| { expanded.set(Some(current)); },
                            "OPEN DOSSIER"
                        }
                    }

                    if let Some((_, peek_title, peek_tech, peek_offset)) = peek {
                        div {
                            class: "range-peek",
                            style: "transform: translateX(calc(-50% + {peek_offset}px));",
                            span { class: "range-peek-title", "{peek_title}" }
                            span { class: "range-peek-tech", "{peek_tech}" }
                        }
                    }

                    div { class: "belt-rail",
                    div {
                        class: "belt-strip",
                        style: "transform: translateX({shift}px);",
                        for (i, project) in projects.iter().enumerate() {
                            {
                                let slug = project.slug.clone();
                                let title = project.title.clone();
                                let state = match (is_loaded, i == current, i < current) {
                                    (false, _, _) => "belt-round",
                                    (true, true, _) => "belt-round loaded chambered",
                                    (true, false, true) => "belt-round loaded spent",
                                    (true, false, false) => "belt-round loaded",
                                };
                                let style = format!("--load-delay: {}ms;", i * BELT_STAGGER_MS);

                                rsx! {
                                    button {
                                        class: "{state}",
                                        style: "{style}",
                                        key: "{slug}",
                                        r#type: "button",
                                        "aria-label": "{title}",
                                        onmouseenter: move |_| { hovered.set(Some(i)); },
                                        onmouseleave: move |_| {
                                            let leaving = *hovered.read() == Some(i);
                                            if leaving {
                                                hovered.set(None);
                                            }
                                        },
                                        onclick: move |_| { scroll_to_round(i, total); },
                                        svg {
                                            class: "belt-round-shape",
                                            view_box: "0 0 40 130",
                                            preserve_aspect_ratio: "xMidYMax meet",
                                            path {
                                                class: "belt-round-bullet",
                                                d: "M20 2 C15.5 12 14 28 14 47 L26 47 C26 28 24.5 12 20 2 Z",
                                            }
                                            path {
                                                class: "belt-round-case",
                                                d: "M14 45 L26 45 L26 53 L31 61 L31 110 L29 112 L29 117 L32 119 L32 127 L8 127 L8 119 L11 117 L11 112 L9 110 L9 61 L14 53 Z",
                                            }
                                            path {
                                                class: "belt-round-shine",
                                                d: "M12.5 62 L16 62 L16 110 L12.5 110 Z",
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

            if let Some(idx) = current_expanded {
                {
                    let project = &projects[idx];
                    let cat_label = category_label(&project.category);
                    let expanded_style = format!("background: {};", theme::MUTED_TEAL);
                    let repo_url = project.repo_url.clone();
                    let demo_url = project.demo_url.clone();
                    let long_desc = project.long_description.clone();
                    let title = project.title.clone();
                    let tech_stack = project.tech_stack.clone();
                    let repo_link_style = format!(
                        "color: {}; background: {}; border: 2px solid {};",
                        theme::MINT_WHITE, theme::DEEP_NAVY, theme::DEEP_NAVY
                    );
                    let demo_link_style = format!(
                        "color: {}; border: 2px solid {}; background: transparent;",
                        theme::DEEP_NAVY, theme::DEEP_NAVY
                    );

                    rsx! {
                        div {
                            class: "poker-overlay",
                            onclick: move |_| { expanded.set(None); },
                        }
                        div {
                            class: "poker-card-expanded",
                            style: "{expanded_style}",
                            button {
                                class: "poker-close-btn",
                                onclick: move |_| { expanded.set(None); },
                                "X"
                            }
                            div { class: "poker-expanded-left",
                                h2 { class: "poker-expanded-title", "{title}" }
                                span { class: "poker-expanded-category-label", "{cat_label}" }
                            }
                            div { class: "poker-expanded-right",
                                p { class: "poker-expanded-desc", "{long_desc}" }
                                div { class: "poker-expanded-tags",
                                    for tech in tech_stack.iter() {
                                        span { class: "poker-expanded-tag", "{tech}" }
                                    }
                                }
                                div { class: "poker-expanded-links",
                                    if !repo_url.is_empty() {
                                        a {
                                            class: "poker-expanded-link",
                                            style: "{repo_link_style}",
                                            href: "{repo_url}",
                                            target: "_blank",
                                            "VIEW REPO"
                                        }
                                    }
                                    if !demo_url.is_empty() {
                                        a {
                                            class: "poker-expanded-link",
                                            style: "{demo_link_style}",
                                            href: "{demo_url}",
                                            target: "_blank",
                                            "LIVE DEMO"
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
