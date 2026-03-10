use dioxus::prelude::*;
use crate::canvas::{force_graph, hero_zoom, particles, scroll_reveal};
use crate::components::timeline::Timeline;
use crate::data;
use crate::styles::theme;

#[component]
pub fn Home() -> Element {
    let about = data::load_about();

    use_effect(move || {
        particles::start_particles("hero-canvas");
        hero_zoom::init_hero_zoom();
        let skills = data::load_skills();
        let skills_json = serde_json::to_string(&skills).unwrap();
        force_graph::start_force_graph("skills-canvas", &skills_json);
        scroll_reveal::init_scroll_reveals();
    });

    rsx! {
        // Combined hero + about scroll container
        div {
            id: "hero-zoom-container",
            style: "height: 300vh; position: relative;",

            // Sticky viewport — hero AND about share this space
            div {
                style: "position: sticky; top: 0; height: 100vh; overflow: hidden; background-color: {theme::MINT_WHITE};",

                // Particle canvas
                canvas {
                    id: "hero-canvas",
                    style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; z-index: 1;",
                }

                // Dark overlay for about transition (Deep Focus Blue)
                div {
                    id: "hero-bg-overlay",
                    style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; background-color: {theme::DEEP_NAVY}; opacity: 0; z-index: 2; pointer-events: none;",
                }

                // Hero name + subtitle (centered)
                div {
                    id: "hero-text-wrapper",
                    style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; z-index: 10; pointer-events: none;",
                    div {
                        style: "text-align: center;",
                        h1 {
                            id: "hero-name",
                            style: "font-size: 8rem; font-weight: 700; color: {theme::DEEP_NAVY}; text-transform: uppercase; line-height: 0.9; will-change: transform; transform-origin: center center;",
                            "{about.name}"
                        }
                        p {
                            id: "hero-subtitle",
                            style: "font-size: 2rem; color: {theme::DARK_BROWN}; margin-top: 1rem; font-family: {theme::FONT_MONO}; will-change: opacity;",
                            "{about.title}"
                        }
                    }
                }

                // About content — already on screen, just invisible (same color as bg)
                div {
                    id: "about-content",
                    style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; z-index: 5; pointer-events: none; opacity: 0; overflow-y: auto; padding: 6rem 2rem;",
                    div {
                        style: "max-width: 1200px; width: 100%; margin: 0 auto;",
                        h2 {
                            id: "about-heading",
                            style: "font-size: 5rem; font-weight: 700; color: {theme::DARK_BROWN}; text-transform: uppercase; margin-bottom: 2rem;",
                            "ABOUT"
                        }
                        p {
                            id: "about-narrative",
                            style: "font-size: 1.3rem; line-height: 1.8; color: {theme::MINT_WHITE}; max-width: 700px; margin-bottom: 3rem;",
                            "{about.narrative}"
                        }
                        div {
                            style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap: 1rem;",
                            for fact in about.facts.iter() {
                                div {
                                    class: "about-fact-card",
                                    style: "background: {theme::DEEP_NAVY}; padding: 1.5rem; border-left: 4px solid {theme::MUTED_TEAL};",
                                    p {
                                        class: "about-fact-label",
                                        style: "font-family: {theme::FONT_MONO}; font-size: 0.9rem; color: {theme::MINT_WHITE}; text-transform: uppercase;",
                                        "{fact.label}"
                                    }
                                    p {
                                        class: "about-fact-value",
                                        style: "font-size: 1.8rem; font-weight: 700; color: {theme::DARK_BROWN}; margin-top: 0.5rem;",
                                        "{fact.value}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        section { id: "skills",
            "data-reveal": "true",
            style: "padding: 6rem 2rem; min-height: 80vh; background-color: {theme::MINT_WHITE};",
            div { style: "max-width: 1200px; margin: 0 auto;",
                h2 {
                    style: "font-size: 5rem; font-weight: 700; color: {theme::DARK_BROWN}; text-transform: uppercase; margin-bottom: 2rem;",
                    "SKILLS"
                }
                canvas {
                    id: "skills-canvas",
                    style: "width: 100%; height: 600px; border: 3px solid {theme::MUTED_TEAL};",
                }
            }
        }
        Timeline {}
        section { id: "contact",
            style: "padding: 6rem 2rem; background-color: {theme::DEEP_NAVY}; color: {theme::MINT_WHITE};",
            div { style: "max-width: 1200px; margin: 0 auto;",
                h2 {
                    style: "font-size: 5rem; font-weight: 700; color: {theme::DARK_BROWN}; text-transform: uppercase; margin-bottom: 2rem;",
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
