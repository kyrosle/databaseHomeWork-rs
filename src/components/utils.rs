#![allow(non_snake_case)]
use dioxus::prelude::*;

/// Nested Router alternatives
/// # Props
/// ## Parameters:
/// `routes` : Vec(String, Element<'a>)
///
/// route path name, and the route render page(VNode)
#[derive(Props)]
pub struct RouteProps<'a> {
    pub routes: Vec<(String, Option<VNode<'a>>)>,
}

impl<'a> RouteProps<'a> {
    pub fn create_router<'r>(
        fac: NodeFactory<'r>,
        routes: Vec<(&str, LazyNodes<'r, 'r>)>,
    ) -> Vec<(String, Element<'r>)> {
        // let fac = NodeFactory::new(&cx);
        routes
            .into_iter()
            .map(|(n, r)| (n.to_owned(), Some(r.call(fac))))
            .collect::<Vec<_>>()
    }
}

/// Nested Router alternatives
/// # Node
/// It will render the router from the `RouteProps` value
pub fn SelectRoute<'a>(cx: Scope<'a, RouteProps<'a>>) -> Element<'a> {
    let routes = cx
        .props
        .routes
        .iter()
        .map(|(n, r)| {
            rsx! {
                div{
                    h1 {"{n}"}
                    r
                }
            }
        })
        .collect::<Vec<_>>();
    cx.render(rsx! {
        div {
            routes
        }
    })
}
