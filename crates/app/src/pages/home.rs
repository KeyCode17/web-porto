use dioxus::prelude::*;
use crate::canvas::{force_graph, particles, scroll_reveal};
use crate::components::timeline::Timeline;
use crate::data;
use crate::styles::theme;

#[component]
pub fn Home() -> Element {
    let about = data::load_about();

    use_effect(move || {
        particles::start_particles("hero-canvas");
        let skills = data::load_skills();
        let skills_json = serde_json::to_string(&skills).unwrap();
        force_graph::start_force_graph("skills-canvas", &skills_json);
        scroll_reveal::init_scroll_reveals();
    });

    rsx! {
        section { id: "hero",
            style: "position: relative; height: 100vh; display: flex; align-items: center; justify-content: center; overflow: hidden;",
            canvas {
                id: "hero-canvas",
                style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%;",
            }
            div {
                style: "position: relative; z-index: 10; text-align: center;",
                h1 {
                    style: "font-size: 8rem; font-weight: 700; color: {theme::DEEP_NAVY}; text-transform: uppercase; line-height: 0.9;",
                    "{about.name}"
                }
                p {
                    style: "font-size: 2rem; color: {theme::DARK_BROWN}; margin-top: 1rem; font-family: {theme::FONT_MONO};",
                    "{about.title}"
                }
            }
        }
        section { id: "about",
            "data-reveal": "true",
            style: "padding: 6rem 2rem; background-color: {theme::WARM_BEIGE};",
            div { style: "max-width: 1200px; margin: 0 auto;",
                h2 {
                    style: "font-size: 5rem; font-weight: 700; color: {theme::DEEP_NAVY}; text-transform: uppercase; margin-bottom: 2rem;",
                    "ABOUT"
                }
                p {
                    style: "font-size: 1.3rem; line-height: 1.8; color: {theme::DEEP_NAVY}; max-width: 700px; margin-bottom: 3rem;",
                    "{about.narrative}"
                }
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 2rem;",
                    for fact in about.facts.iter() {
                        div {
                            style: "border: 3px solid {theme::DEEP_NAVY}; padding: 1.5rem;",
                            p {
                                style: "font-family: {theme::FONT_MONO}; font-size: 0.9rem; color: {theme::MUTED_TEAL}; text-transform: uppercase;",
                                "{fact.label}"
                            }
                            p {
                                style: "font-size: 1.8rem; font-weight: 700; color: {theme::DEEP_NAVY}; margin-top: 0.5rem;",
                                "{fact.value}"
                            }
                        }
                    }
                }
            }
        }
        section { id: "skills",
            "data-reveal": "true",
            style: "padding: 6rem 2rem; min-height: 80vh;",
            div { style: "max-width: 1200px; margin: 0 auto;",
                h2 {
                    style: "font-size: 5rem; font-weight: 700; color: {theme::DEEP_NAVY}; text-transform: uppercase; margin-bottom: 2rem;",
                    "SKILLS"
                }
                canvas {
                    id: "skills-canvas",
                    style: "width: 100%; height: 600px; border: 3px solid {theme::DEEP_NAVY};",
                }
            }
        }
        Timeline {}
        section { id: "contact",
            "data-reveal": "true",
            style: "padding: 6rem 2rem; background-color: {theme::DEEP_NAVY}; color: {theme::MINT_WHITE};",
            div { style: "max-width: 1200px; margin: 0 auto;",
                h2 {
                    style: "font-size: 5rem; font-weight: 700; text-transform: uppercase; margin-bottom: 2rem;",
                    "CONTACT"
                }
                div {
                    style: "display: flex; gap: 3rem; flex-wrap: wrap;",
                    for link in about.social_links.iter() {
                        a {
                            href: "{link.url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            style: "font-size: 1.5rem; font-weight: 700; color: {theme::MINT_WHITE}; text-transform: uppercase; border: 3px solid {theme::MINT_WHITE}; padding: 1rem 2rem; transition: all 0.2s;",
                            "{link.platform}"
                        }
                    }
                }
            }
        }
    }
}
