use dioxus::prelude::*;
use crate::data;
use crate::styles::theme;
use crate::Route;

#[component]
pub fn Papers() -> Element {
    let papers = data::load_papers();

    rsx! {
        div { style: "padding: 6rem 2rem; min-height: 100vh;",
            div { style: "max-width: 1200px; margin: 0 auto;",
                h1 {
                    style: "font-size: 6rem; font-weight: 700; color: {theme::DEEP_NAVY}; text-transform: uppercase; margin-bottom: 3rem;",
                    "PAPERS"
                }
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(350px, 1fr)); gap: 2rem;",
                    for paper in papers.iter() {
                        {
                            let authors_str = paper.authors.join(", ");
                            rsx! {
                                Link {
                                    to: Route::PaperDetail { slug: paper.slug.clone() },
                                    div {
                                        style: "border: 3px solid {theme::DEEP_NAVY}; padding: 2rem; background: {theme::MINT_WHITE}; cursor: pointer;",
                                        p {
                                            style: "font-family: {theme::FONT_MONO}; font-size: 0.8rem; color: {theme::MUTED_TEAL}; text-transform: uppercase;",
                                            "{paper.venue}"
                                        }
                                        h2 {
                                            style: "font-size: 1.5rem; font-weight: 700; color: {theme::DEEP_NAVY}; margin: 0.5rem 0;",
                                            "{paper.title}"
                                        }
                                        p {
                                            style: "font-size: 0.9rem; color: {theme::DARK_BROWN}; margin-bottom: 0.5rem;",
                                            "{authors_str}"
                                        }
                                        p {
                                            style: "font-size: 0.9rem; color: {theme::DEEP_NAVY}; line-height: 1.5;",
                                            "{paper.r#abstract}"
                                        }
                                        div {
                                            style: "display: flex; flex-wrap: wrap; gap: 0.5rem; margin-top: 1rem;",
                                            for tag in paper.tags.iter() {
                                                span {
                                                    style: "font-family: {theme::FONT_MONO}; font-size: 0.7rem; border: 2px solid {theme::DEEP_NAVY}; padding: 0.2rem 0.5rem;",
                                                    "{tag}"
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
