use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            Router {
                ul {
                    Link {to: "/", li{"Go Home"}}
                    Link {to: "/hello", li{"Hello"}}
                }
                Route { to: "/" ,Home {} }
                Route { to: "/hello" , Hello {}}
            }
        }
    ))
}

fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {"Home"}
    })
}

fn Hello(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {"Hello"}
    })
}
