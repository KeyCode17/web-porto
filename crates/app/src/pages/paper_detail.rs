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
            let has_pdf = !paper.pdf_file.is_empty();
            let has_url = !paper.url.is_empty();
            let pdf_url = format!("/papers/{}", paper.pdf_file);

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
                            style: "border: 3px solid {theme::DEEP_NAVY}; padding: 2rem; margin-bottom: 2rem; background: {theme::DEEP_NAVY};",
                            h3 {
                                style: "font-size: 1.2rem; font-weight: 700; color: {theme::DARK_BROWN}; text-transform: uppercase; margin-bottom: 1rem;",
                                "ABSTRACT"
                            }
                            p {
                                style: "line-height: 1.8; color: {theme::MINT_WHITE};",
                                "{paper.r#abstract}"
                            }
                        }
                        // Buttons
                        div {
                            style: "display: flex; gap: 1rem; flex-wrap: wrap; margin-bottom: 2rem;",
                            if has_url {
                                a {
                                    href: "{paper.url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    style: "display: inline-block; font-family: {theme::FONT_MONO}; font-size: 1rem; font-weight: 700; color: {theme::MINT_WHITE}; background: {theme::DEEP_NAVY}; border: 3px solid {theme::DEEP_NAVY}; padding: 0.8rem 1.5rem; text-transform: uppercase;",
                                    "VIEW ON JOURNAL →"
                                }
                            }
                        }
                        // PDF Viewer
                        if has_pdf {
                            div {
                                style: "border: 3px solid {theme::DEEP_NAVY};",
                                object {
                                    data: "{pdf_url}",
                                    r#type: "application/pdf",
                                    style: "width: 100%; height: 80vh;",
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
