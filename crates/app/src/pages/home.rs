use dioxus::prelude::*;
use crate::canvas::particles;
use crate::data;
use crate::styles::theme;

#[component]
pub fn Home() -> Element {
    let about = data::load_about();

    use_effect(move || {
        particles::start_particles("hero-canvas");
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
        section { id: "about", style: "padding: 4rem 2rem;", "About Section" }
        section { id: "skills", style: "padding: 4rem 2rem;", "Skills Section" }
        section { id: "experience", style: "padding: 4rem 2rem;", "Experience Section" }
        section { id: "contact", style: "padding: 4rem 2rem;", "Contact Section" }
    }
}
