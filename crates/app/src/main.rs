use dioxus::prelude::*;

mod canvas;
mod components;
mod data;
mod pages;
mod styles;

use pages::home::Home;
use pages::projects::Projects;
use pages::project_detail::ProjectDetail;
use pages::papers::Papers;
use pages::paper_detail::PaperDetail;
use pages::chat::Chat;

#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
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
        style { "{styles::global::global_css()}" }
        components::navbar::Navbar {}
        div {
            style: "padding-top: 60px;",
            Router::<Route> {}
        }
    }
}
