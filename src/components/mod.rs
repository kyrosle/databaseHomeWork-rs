#![allow(non_snake_case)]
mod utils;

mod add_page;
mod delete_page;
mod modify_page;
mod query_page;

use dioxus::prelude::*;

pub use add_page::*;
pub use delete_page::*;
pub use modify_page::*;
pub use query_page::*;
pub use utils::*;

pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "box",
            Router {
                ul {
                    Link {class: "button is-info is-light is-small" to: "/",  li { "      HomePage      " } }
                    Link {class: "button is-info is-light is-small" to: "/1", li { "Information Addition" } }
                    Link {class: "button is-info is-light is-small" to: "/2", li { " Information Delete " } }
                    Link {class: "button is-info is-light is-small" to: "/3", li { " Information Change " } }
                    Link {class: "button is-info is-light is-small" to: "/4", li { " Information Search " } }
                }
                Route { to: "/", div { "HomePage" } } 
                Route { to: "/1", div { AddPage { } } }
                Route { to: "/2", div { DeletePage { } } }
                Route { to: "/3", div { ModifyPage { } } }
                Route { to: "/4", div { QueryPage { } } }
                Route { to: "" h1 { "404 Not Found" } }
            }
        }
    })
}
