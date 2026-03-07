use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::data;
use crate::styles::theme;

const STAMP_LABELS: &[&str] = &["PUBLISHED", "PEER-REVIEWED", "PUBLISHED"];

/// Paper positions: (top%, left%, rotation)
const PAPER_POS: &[(&str, &str, &str)] = &[
    ("35%", "2%", "-3deg"),
    ("5%", "35%", "2.5deg"),
    ("40%", "62%", "-1.5deg"),
];

/// Photo positions: (top%, left%, rotation)
const PHOTO_POS: &[(&str, &str, &str)] = &[
    ("2%", "8%", "-12deg"),
    ("55%", "42%", "7deg"),
    ("8%", "75%", "-5deg"),
];

const PHOTO_URLS: &[&str] = &[
    "/photos/thumb/Photo%201.JPG",
    "/photos/thumb/Photo%202.JPG",
    "/photos/thumb/Photo%203.JPG",
];


#[component]
pub fn Papers() -> Element {
    let papers = data::load_papers();
    let mut expanded: Signal<Option<usize>> = use_signal(|| None);
    let mut burning: Signal<Option<(usize, f64, f64)>> = use_signal(|| None);
    let mut burned: Signal<Vec<usize>> = use_signal(|| Vec::new());

    use_effect(move || {
        let document = web_sys::window().unwrap().document().unwrap();
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

    let current_expanded = *expanded.read();

    rsx! {
        div { class: "board-page",
            h1 { class: "board-title", "PAPERS" }
            p { class: "board-subtitle", "RESEARCH BOARD" }

            div { class: "board-scene",

                // Pinned photos
                for (i, url) in PHOTO_URLS.iter().enumerate() {
                    {
                        let (top, left, rot) = PHOTO_POS[i];
                        let style = format!("top: {}; left: {}; transform: rotate({});", top, left, rot);
                        let current_burn = *burning.read();
                        let is_burning = matches!(current_burn, Some((idx, _, _)) if idx == i);
                        let is_burned = burned.read().contains(&i);
                        let wrap_class = if is_burning { "board-photo-wrap burning" } else { "board-photo-wrap" };
                        if is_burned { return rsx! {} }

                        rsx! {
                            div {
                                class: "{wrap_class}",
                                style: "{style}",
                                key: "photo-{i}",
                                onclick: move |evt| {
                                    // Block if any photo is currently burning
                                    if burning.read().is_some() { return; }
                                    let coords = evt.element_coordinates();
                                    let x_pct = (coords.x / 128.0 * 100.0).clamp(0.0, 100.0);
                                    let y_pct = (coords.y / 168.0 * 100.0).clamp(0.0, 100.0);
                                    burning.set(Some((i, x_pct, y_pct)));

                                    let window = web_sys::window().unwrap();
                                    let cb = Closure::<dyn FnMut()>::new(move || {
                                        burning.set(None);
                                        burned.write().push(i);
                                    });
                                    window.set_timeout_with_callback_and_timeout_and_arguments_0(
                                        cb.as_ref().unchecked_ref(), 3500
                                    ).unwrap();
                                    cb.forget();
                                },
                                div { class: "board-pin board-pin-red" }
                                img { class: "board-photo-img", src: "{url}", alt: "" }
                                if is_burning {
                                    {
                                        let (_, bx, by) = current_burn.unwrap();
                                        let burn_style = format!("--burn-x: {:.1}%; --burn-y: {:.1}%;", bx, by);
                                        rsx! {
                                            div {
                                                class: "board-photo-burn",
                                                style: "{burn_style}",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Paper documents
                for (i, paper) in papers.iter().enumerate() {
                    {
                        let (top, left, rot) = PAPER_POS[i];
                        let stamp = STAMP_LABELS[i % STAMP_LABELS.len()];
                        let venue_short = paper.venue.split('(').nth(1)
                            .and_then(|s| s.split(')').next())
                            .unwrap_or(&paper.venue);
                        let title = paper.title.clone();
                        let authors = paper.authors.join(", ");
                        let tags = paper.tags.clone();
                        let doc_style = format!(
                            "top: {}; left: {}; transform: rotate({});",
                            top, left, rot
                        );
                        let blur = match current_expanded {
                            Some(idx) if idx != i => "board-doc-blurred",
                            _ => "",
                        };

                        rsx! {
                            div {
                                class: "board-doc {blur}",
                                style: "{doc_style}",
                                key: "{paper.slug}",
                                onclick: move |_| { expanded.set(Some(i)); },
                                div { class: "board-pin" }
                                div { class: "board-stamp", "{stamp}" }
                                p { class: "board-venue", "{venue_short}" }
                                h2 { class: "board-doc-title", "{title}" }
                                p { class: "board-doc-author", "{authors}" }
                                div { class: "board-doc-tags",
                                    for tag in tags.iter() {
                                        span { class: "board-doc-tag", "{tag}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Expanded paper overlay
            if let Some(idx) = current_expanded {
                {
                    let paper = &papers[idx];
                    let title = paper.title.clone();
                    let authors = paper.authors.join(", ");
                    let venue = paper.venue.clone();
                    let abstract_text = paper.r#abstract.clone();
                    let tags = paper.tags.clone();
                    let has_url = !paper.url.is_empty();
                    let has_pdf = !paper.pdf_file.is_empty();
                    let url = paper.url.clone();
                    let pdf_url = format!("/papers/{}", paper.pdf_file);

                    rsx! {
                        div {
                            class: "board-overlay",
                            onclick: move |_| { expanded.set(None); },
                        }
                        div { class: "board-expanded",
                            button {
                                class: "board-close-btn",
                                onclick: move |_| { expanded.set(None); },
                                "X"
                            }
                            div { class: "board-expanded-scroll",
                                p { class: "board-expanded-venue", "{venue}" }
                                h2 { class: "board-expanded-title", "{title}" }
                                p { class: "board-expanded-author", "{authors}" }
                                div { class: "board-expanded-tags",
                                    for tag in tags.iter() {
                                        span { class: "board-expanded-tag", "{tag}" }
                                    }
                                }
                                div { class: "board-expanded-abstract",
                                    h3 { "ABSTRACT" }
                                    p { "{abstract_text}" }
                                }
                                div { class: "board-expanded-links",
                                    if has_url {
                                        a {
                                            class: "board-expanded-link board-link-primary",
                                            href: "{url}",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            "VIEW ON JOURNAL \u{2192}"
                                        }
                                    }
                                    if has_pdf {
                                        a {
                                            class: "board-expanded-link board-link-secondary",
                                            href: "{pdf_url}",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            "VIEW PDF \u{2192}"
                                        }
                                    }
                                }
                                if has_pdf {
                                    div { class: "board-expanded-pdf",
                                        object {
                                            data: "{pdf_url}",
                                            r#type: "application/pdf",
                                            style: "width: 100%; height: 70vh; border: none;",
                                            p {
                                                "Unable to display PDF. "
                                                a {
                                                    href: "{pdf_url}",
                                                    target: "_blank",
                                                    "Download PDF"
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
