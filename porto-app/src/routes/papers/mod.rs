pub mod _constants;
pub mod detail;

use self::_constants::{BURN_DURATION_MS, PAPER_POS, PHOTO_POS, PHOTO_URLS, STAMP_LABELS};
use crate::data;
use crate::libs::{on_escape, sleep_ms};
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn Papers() -> Element {
    let papers = data::load_papers();
    let mut expanded: Signal<Option<usize>> = use_signal(|| None);
    let mut burning: Signal<Option<(usize, f64, f64)>> = use_signal(|| None);
    let mut burned: Signal<Vec<usize>> = use_signal(Vec::new);

    let _escape_listener = use_hook(|| Rc::new(on_escape(move || expanded.set(None))));

    let current_expanded = *expanded.read();

    rsx! {
        div { class: "board-page",
            h1 { class: "board-title", "PAPERS" }
            p { class: "board-subtitle", "RESEARCH BOARD" }

            div { class: "board-scene",

                for (i, url) in PHOTO_URLS.iter().enumerate() {
                    {
                        let (top, left, rot) = PHOTO_POS[i];
                        let style = format!("top: {}; left: {}; transform: rotate({});", top, left, rot);
                        let burn_origin = burning
                            .read()
                            .filter(|(idx, _, _)| *idx == i);
                        let is_burning = burn_origin.is_some();
                        let is_burned = burned.read().contains(&i);
                        let wrap_class = if is_burning { "board-photo-wrap burning" } else { "board-photo-wrap" };
                        if is_burned { return rsx! {} }

                        rsx! {
                            div {
                                class: "{wrap_class}",
                                style: "{style}",
                                key: "photo-{i}",
                                onclick: move |evt| {
                                    if burning.read().is_some() { return; }
                                    let coords = evt.element_coordinates();
                                    let x_pct = (coords.x / 128.0 * 100.0).clamp(0.0, 100.0);
                                    let y_pct = (coords.y / 168.0 * 100.0).clamp(0.0, 100.0);
                                    burning.set(Some((i, x_pct, y_pct)));

                                    spawn(async move {
                                        sleep_ms(BURN_DURATION_MS).await;
                                        burning.set(None);
                                        burned.write().push(i);
                                    });
                                },
                                div { class: "board-pin board-pin-red" }
                                img { class: "board-photo-img", src: "{url}", alt: "" }
                                if let Some((_, bx, by)) = burn_origin {
                                    {
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
