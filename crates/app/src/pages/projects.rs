use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div { class: "page-projects",
            h1 { "Projects Gallery" }
        }
    }
}
