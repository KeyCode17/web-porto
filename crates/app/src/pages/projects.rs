use dioxus::prelude::*;
use crate::data;
use crate::styles::theme;
use crate::Route;

#[component]
pub fn Projects() -> Element {
    let projects = data::load_projects();

    rsx! {
        div { style: "padding: 6rem 2rem; min-height: 100vh;",
            div { style: "max-width: 1200px; margin: 0 auto;",
                h1 {
                    style: "font-size: 6rem; font-weight: 700; color: {theme::DEEP_NAVY}; text-transform: uppercase; margin-bottom: 3rem;",
                    "PROJECTS"
                }
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(350px, 1fr)); gap: 2rem;",
                    for project in projects.iter() {
                        Link {
                            to: Route::ProjectDetail { slug: project.slug.clone() },
                            div {
                                style: "border: 3px solid {theme::DEEP_NAVY}; padding: 2rem; background: {theme::MINT_WHITE}; cursor: pointer; transition: transform 0.1s;",
                                p {
                                    style: "font-family: {theme::FONT_MONO}; font-size: 0.8rem; color: {theme::MUTED_TEAL}; text-transform: uppercase;",
                                    "{project.category}"
                                }
                                h2 {
                                    style: "font-size: 2rem; font-weight: 700; color: {theme::DEEP_NAVY}; margin: 0.5rem 0;",
                                    "{project.title}"
                                }
                                p {
                                    style: "color: {theme::DARK_BROWN};",
                                    "{project.short_description}"
                                }
                                div {
                                    style: "display: flex; flex-wrap: wrap; gap: 0.5rem; margin-top: 1rem;",
                                    for tech in project.tech_stack.iter() {
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
        }
    }
}
