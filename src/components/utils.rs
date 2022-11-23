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
/// pub fn Component(cx: Scope) -> Element {
///     let routes = create_router(
///         &cx,
///         vec![
///             ("Add Employee", rsx!(AddEmployee {})),
///             ("Page2", rsx!(Add2 {})),
///         ],
///     );
///     cx.render(rsx! {
///         SelectRoute { routes: routes }
///     })
/// }
/// ```
pub fn create_router<'a, P>(
    cx: &Scope<'a, P>,
    routes: Vec<(&str, LazyNodes<'a, 'a>)>,
) -> Vec<(String, Element<'a>)> {
    routes
        .into_iter()
        .map(|(n, r)| (n.to_owned(), lazy_node_to_element(cx, r)))
        .collect::<Vec<_>>()
}

/// Convent LazyNodes to Element(Option<VNode>) 
pub fn lazy_node_to_element<'a, P>(
    cx: &Scope<'a, P>,
    lazy_node: LazyNodes<'a,'a>
) -> Element<'a> {
    let fac = NodeFactory::new(cx);
    Some(lazy_node.call(fac))
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
                    style: "padding:5px",
                    div {
                        class: "button is-tag is-light is-small",
                        onclick: move |_| index.set(idx),
                        div {
                            "{s}"
                        }
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
                label { class: "label", "{name}" }
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
