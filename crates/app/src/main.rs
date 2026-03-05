use dioxus::prelude::*;

mod data;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let about = data::load_about();
    rsx! {
        div {
            style: "background-color: #F1FFFA; color: #273B76; min-height: 100vh; font-family: 'Space Grotesk', sans-serif;",
            h1 { style: "font-size: 4rem; padding: 2rem;", "{about.name}" }
            p { "{about.title}" }
            p { "{about.narrative}" }
        }
    }
}
