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
        Router::<Route> {}
    }
}

/// Layout wrapping all routes — renders navbar, global styles, cursor
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
