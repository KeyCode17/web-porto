use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "page-home",
            section { id: "hero", "Hero Section" }
            section { id: "about", "About Section" }
            section { id: "skills", "Skills Section" }
            section { id: "experience", "Experience Section" }
            section { id: "contact", "Contact Section" }
        }
    }
}
