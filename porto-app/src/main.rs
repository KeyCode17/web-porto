use dioxus::prelude::*;

mod canvas;
mod components;
mod data;
mod faq_engine;
mod pages;
mod styles;
mod utils;

use pages::chat::Chat;
use pages::home::Home;
use pages::paper_detail::PaperDetail;
use pages::papers::Papers;
use pages::project_detail::ProjectDetail;
use pages::projects::Projects;

#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},

    #[route("/projects")]
    Projects {},

    #[route("/projects/:slug")]
    ProjectDetail { slug: String },

    #[route("/papers")]
    Papers {},

    #[route("/papers/:slug")]
    PaperDetail { slug: String },

    #[route("/chat")]
    Chat {},
}

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        document::Link { rel: "icon", r#type: "image/svg+xml", href: "/favicon.svg" }
        Router::<Route> {}
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        style { "{styles::global::global_css()}" }
        components::cursor::CustomCursor {}
        components::navbar::Navbar {}
        div {
            Outlet::<Route> {}
        }
    }
}
