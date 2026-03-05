use dioxus::prelude::*;

#[component]
pub fn Chat() -> Element {
    rsx! {
        div { class: "page-chat",
            h1 { "Chat" }
        }
    }
}
