mod components;
use components::*;
use dioxus::prelude::*;
use dioxus_desktop::tao::{dpi::LogicalSize, window::WindowBuilder};

fn main() {
    dioxus_desktop::launch_cfg(app, |c| {
        c.with_custom_head("<link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css\">".to_string())
        .with_window(
            |_| WindowBuilder::new()
                .with_title("Human Resource Management System")
                .with_inner_size(LogicalSize::new(700, 600))
                .with_resizable(false)
        )
    });
}

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            style: "padding: 12px",
            class: "box",
            figure {
                class: "image is-32x32",
                img {
                    src: "../asserts/images/atr.jpg"
                }
            }
            div {
                class: "content",
                "Human Resource Management System"
            }
        }
        HomePage {}
    ))
}
