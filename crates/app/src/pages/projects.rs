use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::data;
use crate::styles::theme;

/// Suit defines everything: symbol, suit color, card background
/// Shuffled suit order for 9 cards
const CARD_DEALS: &[(&str, &str, &str)] = &[
    // (suit, suit_color, bg_color)
    // ♠=orange, ♦=teal, ♣=crimson, ♥=navy
    ("\u{2660}", "#E5E5E5", "#D65108"), // ♠ orange
    ("\u{2666}", "#E84040", "#568EA3"), // ♦ teal
    ("\u{2665}", "#E84040", "#02182B"), // ♥ navy
    ("\u{2663}", "#E5E5E5", "#8B1A1A"), // ♣ crimson
    ("\u{2666}", "#E84040", "#568EA3"), // ♦ teal
    ("\u{2660}", "#E5E5E5", "#D65108"), // ♠ orange
    ("\u{2665}", "#E84040", "#02182B"), // ♥ navy
    ("\u{2660}", "#E5E5E5", "#D65108"), // ♠ orange
    ("\u{2665}", "#E84040", "#02182B"), // ♥ navy
];

fn card_deal(index: usize) -> (&'static str, &'static str, &'static str) {
    CARD_DEALS[index % CARD_DEALS.len()]
}

fn category_label(cat: &str) -> &'static str {
    match cat {
        "ai" => "AI / ML",
        "data" => "Data Analysis",
        "web" => "Web Development",
        "systems" => "Systems",
        _ => "Other",
    }
}

#[component]
pub fn Projects() -> Element {
    let projects = data::load_projects();
    let mut phase = use_signal(|| 0u8);
    let mut expanded: Signal<Option<usize>> = use_signal(|| None);
    let mut hovered: Signal<Option<usize>> = use_signal(|| None);

    use_effect(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // Phase 1: shuffle at 200ms
        let cb1 = Closure::<dyn FnMut()>::new(move || {
            phase.set(1);
        });
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                cb1.as_ref().unchecked_ref(),
                200,
            )
            .unwrap();
        cb1.forget();

        // Phase 2: dealing at 800ms
        let cb2 = Closure::<dyn FnMut()>::new(move || {
            phase.set(2);
        });
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                cb2.as_ref().unchecked_ref(),
                800,
            )
            .unwrap();
        cb2.forget();

        // Phase 3: dealt & ready (after deal animation finishes ~800 + 9*150 + 600 = 2750ms)
        let cb3 = Closure::<dyn FnMut()>::new(move || {
            phase.set(3);
        });
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                cb3.as_ref().unchecked_ref(),
                2800,
            )
            .unwrap();
        cb3.forget();

        // Escape key listener
        let cb_key = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
            move |e: web_sys::KeyboardEvent| {
                if e.key() == "Escape" {
                    expanded.set(None);
                }
            },
        );
        document
            .add_event_listener_with_callback("keydown", cb_key.as_ref().unchecked_ref())
            .unwrap();
        cb_key.forget();
    });

    let current_phase = *phase.read();
    let current_expanded = *expanded.read();
    let current_hovered = *hovered.read();
    let n = projects.len();

    rsx! {
        div { style: "padding: 4rem 2rem 1rem; height: 100vh; overflow: hidden;",
            h1 {
                style: "font-size: 6rem; font-weight: 700; color: {theme::DEEP_NAVY}; text-transform: uppercase; text-align: center; margin-bottom: 0.5rem; font-family: {theme::FONT_HEADING};",
                "PROJECTS"
            }
            p {
                style: "font-family: {theme::FONT_MONO}; font-size: 0.9rem; color: {theme::MUTED_TEAL}; text-align: center; margin-bottom: 1rem;",
                "CLICK A CARD TO REVEAL"
            }
            div { class: "poker-container",
                for (i, project) in projects.iter().enumerate() {
                    {
                        let (suit, suit_color, bg_color) = card_deal(i);
                        let angle = if n > 1 {
                            -45.0 + (i as f64) * (95.0 / (n - 1) as f64)
                        } else {
                            0.0
                        };
                        let shuffle_rot = ((i as f64 * 7.3) % 11.0) - 5.5;
                        let shuffle_x = ((i as f64 * 13.7) % 21.0) - 10.5;
                        let phase_class = match current_phase {
                            0 => "",
                            1 => "phase-shuffle",
                            2 => "phase-dealt",
                            _ => "phase-dealt phase-ready",
                        };
                        let blur_class = match current_expanded {
                            Some(idx) if idx != i => "blurred",
                            _ => "",
                        };
                        let hover_class = match current_hovered {
                            Some(idx) if idx == i => "hovered",
                            _ => "",
                        };
                        let card_style = format!(
                            "background: {}; --final-rot: {}deg; --shuffle-rot: {}deg; --shuffle-x: {}px; --deal-delay: {}ms; --suit-color: {}; z-index: {};",
                            bg_color, angle, shuffle_rot, shuffle_x, i * 150, suit_color, i + 1
                        );
                        let card_class = format!("poker-card {} {} {}", phase_class, blur_class, hover_class);
                        let cat_label = category_label(&project.category);
                        let title = project.title.clone();
                        let slug = project.slug.clone();
                        let suit_top = suit;
                        let suit_bottom = suit;

                        rsx! {
                            div {
                                class: "{card_class}",
                                style: "{card_style}",
                                key: "{slug}",
                                onmouseenter: move |_| { hovered.set(Some(i)); },
                                onmouseleave: move |_| {
                                    if *hovered.read() == Some(i) {
                                        hovered.set(None);
                                    }
                                },
                                onclick: move |_| { expanded.set(Some(i)); },
                                span { class: "poker-card-suit", "{suit_top}" }
                                span { class: "poker-card-title", "{title}" }
                                span { class: "poker-card-category", "{cat_label}" }
                                span { class: "poker-card-suit-bottom", "{suit_bottom}" }
                            }
                        }
                    }
                }
            }

            if let Some(idx) = current_expanded {
                {
                    let project = &projects[idx];
                    let (suit, _suit_color, bg_color) = card_deal(idx);
                    let cat_label = category_label(&project.category);
                    let expanded_style = format!("background: {};", bg_color);
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
                                span { class: "poker-expanded-suit", "{suit}" }
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
