use dioxus::prelude::*;

#[component]
pub fn Papers() -> Element {
    rsx! {
        div { class: "page-papers",
            h1 { "Papers" }
        }
    }
}
