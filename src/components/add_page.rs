#![allow(non_snake_case)]
use crate::components::*;
use dioxus::prelude::*;

pub fn AddPage(cx: Scope) -> Element {
    let fac = NodeFactory::new(&cx);

    //     let route1 = Some(rsx!(Add1 {}).call(fac));
    //     let route2 = Some(rsx!(Add2 {}).call(fac));
    //     let routes = vec![("Page1".to_owned(), route1), ("Page2".to_owned(), route2)];

    let routes = RouteProps::create_router(
        fac,
        vec![("Page1", rsx!(Add1 {})), ("Page2", rsx!(Add2 {}))],
    );

    cx.render(rsx! {
        SelectRoute { routes: routes }
    })
}

fn Add1(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            "Add1"
        }
    })
}

fn Add2(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            "Add2"
        }
    })
}

#[derive(Props, PartialEq, Default)]
struct EmployeeProps {
    name: String,
    health: String,
    salary: String,
    department_id: usize,
    post_id: usize,
    practical_type: usize,
    birth: String,
    information: String,
}

fn AddEmployee(cx: Scope) -> Element {
    let employee = use_state(&cx, EmployeeProps::default);
    cx.render(rsx! {
        h1 {"From"}
        // form {
            // onsubmit: move |_| {},
            // oninput: move |_| {},
            // input { r#type: "text", name: "姓名: " }
            // input { r#type: "text", name: "健康情况: " }
            // input { r#type: "text", name: "月薪资: " }
        // }
    })
}
