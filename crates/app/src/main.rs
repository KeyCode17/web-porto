use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        div {
            style: "background-color: #F1FFFA; color: #273B76; min-height: 100vh; font-family: 'Space Grotesk', sans-serif;",
            h1 { style: "font-size: 4rem; padding: 2rem;", "web-porto" }
            p { "Rust + WASM Portfolio — Coming Soon" }
        }
    }
}
