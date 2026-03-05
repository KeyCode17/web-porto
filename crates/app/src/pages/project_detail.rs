use dioxus::prelude::*;

#[component]
pub fn ProjectDetail(slug: String) -> Element {
    rsx! {
        div { class: "page-project-detail",
            h1 { "Project: {slug}" }
        }
    }
}
