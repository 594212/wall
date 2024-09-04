use dioxus::prelude::*;

use crate::DarkMode;

#[component]
pub fn Navbar() -> Element {
    let mut is_theme_dark = use_context::<Signal<DarkMode>>();

    let mut _logo = "./logo.png";
    let mut logo = "./logo_black.png";
    let mut toggle = "./night.png";
    let mut search_icon = "./search-w.png";
    if is_theme_dark().0 {
        logo = "./logo_white.png";
        toggle = "./day.png";
        search_icon = "./search-b.png";
    }

    rsx! {
        link {
            rel: "stylesheet",
            href: "navbar.css",
        }
        div { class : "navbar",
            img { src: "{logo}", alt: "", class: "logo" }
            h1 { "King Bar" }

            ul {
                li { "Home" }
                li { "Catalog" }
            }
            div { class: "search-box",
                input { "type": "text", placeholder : "Search" }, img { src: "{search_icon}", alt: ""} }
            img { src: "{toggle}", alt: "", class: "toggle-icon",  onclick:
                move |_event| {
                    is_theme_dark.write().0 = !is_theme_dark().0;
                },
            }
        }
    }
}
