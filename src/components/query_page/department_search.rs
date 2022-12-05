use backend::{prelude::Department, show_department_information};
use dioxus::prelude::*;

pub fn DepartmentSearch(cx: Scope) -> Element {
    let id = use_state(&cx, || String::from(""));
    let t = Department {
        id: Some(0),
        manager_name: Some(String::from("None")),
        manager_id: Some(0),
        name: Some(String::from("None")),
    };

    let f = use_future(&cx, (), |_| {
        let id = id.clone();
        async move {
            show_department_information(id.get().parse::<usize>().unwrap_or(7))
                .await
                .unwrap()
        }
    });

    let s = f.value().unwrap_or(&t);

    let d_id = s.id.unwrap();
    let name = s.name.clone().unwrap();
    let manager_id = if s.manager_id.is_none() {
        String::from("Null")
    } else {
        s.manager_id.unwrap().to_string()
    };
    let manager_name = s
        .manager_name
        .clone()
        .unwrap_or_else(|| r#"Null"#.to_owned());

    cx.render(rsx! {
        div {
            div {
                class: "display: flex",
                div {
                    class: "flex: 1",
                    input { class: "input", oninput: move |e| {id.set(e.value.clone())},  r#type: "text", placeholder: "Text input" }
                }
                div {
                    class: "flex: 2",
                    button { class: "button is-link", onclick: move |_| {f.restart();}, "Search" }
                }
            }
            table {
                class: "table",
                thead {
                    tr {
                        td {"id"}
                        td {"name"}
                        td {"manager_id"}
                        td {"manager_name"}
                    }
                }
                tbody {
                    td {"{d_id}"}
                    td {"{name}"}
                    td {"{manager_id}"}
                    td {"{manager_name}"}
                }
            }
        }
    })
}
