use dioxus::prelude::*;
use crate::Route;
use crate::styles::theme;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            style: "position: fixed; top: 0; left: 0; right: 0; z-index: 100; background-color: {theme::DEEP_NAVY}; color: {theme::MINT_WHITE}; padding: 1rem 2rem; display: flex; justify-content: space-between; align-items: center; font-weight: 700; text-transform: uppercase; letter-spacing: 0.1em;",
            Link {
                to: Route::Home {},
                span { style: "color: {theme::MINT_WHITE}; font-size: 1.2rem;", "PORTO" }
            }
            div {
                class: "nav-links",
                style: "display: flex; gap: 2rem;",
                Link { to: Route::Home {}, span { style: "color: {theme::MINT_WHITE};", "HOME" } }
                Link { to: Route::Projects {}, span { style: "color: {theme::MINT_WHITE};", "PROJECTS" } }
                Link { to: Route::Papers {}, span { style: "color: {theme::MINT_WHITE};", "PAPERS" } }
                Link { to: Route::Chat {}, span { style: "color: {theme::MINT_WHITE};", "CHAT" } }
            }
        }
    }
}
