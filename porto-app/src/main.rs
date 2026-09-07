use dioxus::prelude::*;

mod components;
mod data;
mod libs;
mod routes;
mod styles;

use routes::chat::Chat;
use routes::home::Home;
use routes::papers::Papers;
use routes::papers::detail::PaperDetail;
use routes::projects::Projects;
use routes::projects::detail::ProjectDetail;

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
