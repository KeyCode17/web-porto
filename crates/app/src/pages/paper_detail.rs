use dioxus::prelude::*;

#[component]
pub fn PaperDetail(slug: String) -> Element {
    rsx! {
        div { class: "page-paper-detail",
            h1 { "Paper: {slug}" }
        }
    }
}
