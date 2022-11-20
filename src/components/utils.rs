#![allow(non_snake_case)]
use dioxus::prelude::*;

/// Nested Router alternatives
/// # Props
/// ## Parameters:
/// `routes` : Vec(String, Element<'a>)
///
#[derive(Props)]
pub struct RouteProps<'a> {
    pub routes: Vec<(String, Element<'a>)>,
}

/// build a [`NodeFactory`] and a vec![] contains (Router name, Components)
/// which is (String name, rsx!{ xxx })
/// and then used in the `cx.render`
///
/// In a Page site:  
/// ```
/// let fac = NodeFactory::new(&cx);
/// let routes = RouteProps::create_router(
///     fac,
///     vec![
///         ("Add Employee", rsx!(AddEmployee {})),
///         ("Page2", rsx!(Add2 {})),
///     ],
/// );
/// cx.render(rsx! {
///     SelectRoute { routes: routes}
/// })
/// ```
pub fn create_router<'a>(
    fac: NodeFactory<'a>,
    routes: Vec<(&str, LazyNodes<'a, 'a>)>,
) -> Vec<(String, Element<'a>)> {
    routes
        .into_iter()
        .map(|(n, r)| (n.to_owned(), Some(r.call(fac))))
        .collect::<Vec<_>>()
}

/// Nested Router alternatives
/// # Node
/// It will render the router from the `RouteProps` value
pub fn SelectRoute<'a>(cx: Scope<'a, RouteProps<'a>>) -> Element<'a> {
    let index = use_state(&cx, || 0_usize);

    let routes_name = cx.props.routes.iter().map(|(name, _)| name.to_string());

    let routes_name = routes_name
        .enumerate()
        .map(|(idx, s)| {
            rsx! {
                div {
                    onclick: move |_| index.set(idx),
                    div {
                        class: "button is-text is-light is-small",
                        "{s}"
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let route = cx
        .props
        .routes
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx == *index.get())
        .map(|(_, value)| {
            let (name, node) = value;
            rsx! {
                br {}
                "{name}"
                node
            }
        });

    cx.render(rsx! {
        div {
            br{}
            div {
                style: "display:flex;flex-direction: row",
                routes_name
            }
            route
        }
    })
}
