#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use crate::components::navbar::Navbar;

mod components;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(DarkMode(false)));
    // const (theme, set_theme) = (useStat)
    let dark_mode = use_context::<Signal<DarkMode>>();
    let theme = if dark_mode().0 { "dark" } else { "" };
    rsx! {
        link {
            rel: "stylesheet",
            href: "main.css",
        }
        div {
            class: "container {theme}",
            div { Navbar {} }
        }
    }
}

#[derive(Clone, Copy)]
struct DarkMode(bool);
