use dioxus::prelude::*;
use crate::data;
use crate::styles::theme;
use crate::Route;

#[component]
pub fn PaperDetail(slug: String) -> Element {
    let papers = data::load_papers();
    let paper = papers.iter().find(|p| p.slug == slug);

    match paper {
        Some(paper) => {
            let authors_str = paper.authors.join(", ");
            let pdf_url = format!("/static/papers/{}", paper.pdf_file);

            rsx! {
                div { style: "padding: 6rem 2rem; min-height: 100vh;",
                    div { style: "max-width: 1200px; margin: 0 auto;",
                        Link {
                            to: Route::Papers {},
                            span {
                                style: "font-family: {theme::FONT_MONO}; color: {theme::MUTED_TEAL}; font-size: 0.9rem; text-transform: uppercase;",
                                "← BACK TO PAPERS"
                            }
                        }
                        p {
                            style: "font-family: {theme::FONT_MONO}; font-size: 0.9rem; color: {theme::MUTED_TEAL}; text-transform: uppercase; margin-top: 2rem;",
                            "{paper.venue}"
                        }
                        h1 {
                            style: "font-size: 3rem; font-weight: 700; color: {theme::DEEP_NAVY}; margin: 0.5rem 0 1rem;",
                            "{paper.title}"
                        }
                        p {
                            style: "font-size: 1rem; color: {theme::DARK_BROWN}; margin-bottom: 1rem;",
                            "{authors_str}"
                        }
                        // Tags
                        div {
                            style: "display: flex; flex-wrap: wrap; gap: 0.5rem; margin-bottom: 2rem;",
                            for tag in paper.tags.iter() {
                                span {
                                    style: "font-family: {theme::FONT_MONO}; font-size: 0.75rem; border: 2px solid {theme::DEEP_NAVY}; padding: 0.2rem 0.5rem;",
                                    "{tag}"
                                }
                            }
                        }
                        // Abstract
                        div {
                            style: "border: 3px solid {theme::DEEP_NAVY}; padding: 2rem; margin-bottom: 2rem; background: {theme::WARM_BEIGE};",
                            h3 {
                                style: "font-size: 1.2rem; font-weight: 700; color: {theme::DEEP_NAVY}; text-transform: uppercase; margin-bottom: 1rem;",
                                "ABSTRACT"
                            }
                            p {
                                style: "line-height: 1.8; color: {theme::DEEP_NAVY};",
                                "{paper.r#abstract}"
                            }
                        }
                        // PDF Viewer (iframe for now, WASM viewer later)
                        div {
                            style: "border: 3px solid {theme::DEEP_NAVY};",
                            iframe {
                                src: "{pdf_url}",
                                style: "width: 100%; height: 80vh; border: none;",
                                title: "{paper.title}",
                            }
                        }
                    }
                }
            }
        },
        None => rsx! {
            div { style: "padding: 6rem 2rem;",
                h1 { style: "font-size: 3rem; color: {theme::DEEP_NAVY};", "Paper not found" }
                Link {
                    to: Route::Papers {},
                    span { style: "color: {theme::MUTED_TEAL};", "← Back to papers" }
                }
            }
        }
    }
}
