use dioxus::prelude::*;
use crate::data;
use crate::styles::theme;
use crate::Route;

#[component]
pub fn ProjectDetail(slug: String) -> Element {
    let projects = data::load_projects();
    let project = projects.iter().find(|p| p.slug == slug);

    match project {
        Some(project) => rsx! {
            div { style: "padding: 6rem 2rem; min-height: 100vh;",
                div { style: "max-width: 900px; margin: 0 auto;",
                    // Back link
                    Link {
                        to: Route::Projects {},
                        span {
                            style: "font-family: {theme::FONT_MONO}; color: {theme::MUTED_TEAL}; font-size: 0.9rem; text-transform: uppercase;",
                            "← BACK TO PROJECTS"
                        }
                    }
                    // Category
                    p {
                        style: "font-family: {theme::FONT_MONO}; font-size: 0.9rem; color: {theme::MUTED_TEAL}; text-transform: uppercase; margin-top: 2rem;",
                        "{project.category}"
                    }
                    // Title
                    h1 {
                        style: "font-size: 4rem; font-weight: 700; color: {theme::DEEP_NAVY}; text-transform: uppercase; margin: 0.5rem 0 2rem;",
                        "{project.title}"
                    }
                    // Tech stack
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 0.5rem; margin-bottom: 2rem;",
                        for tech in project.tech_stack.iter() {
                            span {
                                style: "font-family: {theme::FONT_MONO}; font-size: 0.8rem; border: 2px solid {theme::DEEP_NAVY}; padding: 0.3rem 0.6rem;",
                                "{tech}"
                            }
                        }
                    }
                    // Description
                    p {
                        style: "font-size: 1.2rem; line-height: 1.8; color: {theme::DEEP_NAVY}; margin-bottom: 2rem;",
                        "{project.long_description}"
                    }
                    // Links
                    div {
                        style: "display: flex; gap: 1.5rem;",
                        if !project.repo_url.is_empty() {
                            a {
                                href: "{project.repo_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                style: "font-weight: 700; color: {theme::MINT_WHITE}; background: {theme::DEEP_NAVY}; padding: 0.8rem 1.5rem; text-transform: uppercase; font-size: 0.9rem;",
                                "VIEW REPO"
                            }
                        }
                        if !project.demo_url.is_empty() {
                            a {
                                href: "{project.demo_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                style: "font-weight: 700; color: {theme::DEEP_NAVY}; border: 3px solid {theme::DEEP_NAVY}; padding: 0.8rem 1.5rem; text-transform: uppercase; font-size: 0.9rem;",
                                "LIVE DEMO"
                            }
                        }
                    }
                }
            }
        },
        None => rsx! {
            div { style: "padding: 6rem 2rem; min-height: 100vh;",
                h1 {
                    style: "font-size: 3rem; color: {theme::DEEP_NAVY};",
                    "Project not found"
                }
                Link {
                    to: Route::Projects {},
                    span { style: "color: {theme::MUTED_TEAL};", "← Back to projects" }
                }
            }
        }
    }
}
