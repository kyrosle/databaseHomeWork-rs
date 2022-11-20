#![allow(non_snake_case)]
use crate::components::*;
use dioxus::prelude::*;

pub fn AddPage(cx: Scope) -> Element {
    let fac = NodeFactory::new(&cx);
    let routes = create_router(
        fac,
        vec![
            ("Add Employee", rsx!(AddEmployee {})),
            ("Page2", rsx!(Add2 {})),
        ],
    );
    cx.render(rsx! {
        SelectRoute { routes: routes}
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
        //     onsubmit: move |_| {},
        //     oninput: move |_| {},
        //     input { r#type: "text", name: "姓名: " }
        //     input { r#type: "text", name: "健康情况: " }
        //     input { r#type: "text", name: "月薪资: " }
        // }
    })
}
