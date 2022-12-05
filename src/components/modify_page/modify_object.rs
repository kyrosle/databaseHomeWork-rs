use backend::update_department_manager;
use dioxus::{events::FormEvent, prelude::*};

use crate::components::{
    create_router,
    modify_page::basic_page::{BasicInformationModify, ImportantInformationModify},
    SelectRoute,
};

pub fn ModifyPage(cx: Scope) -> Element {
    let routes = create_router(
        &cx,
        vec![
            ("EmployeeModify", rsx!(EmployeeModify {})),
            ("EmployeeChangeModify", rsx!(EmployeeChangeModify {})),
        ],
    );
    cx.render(rsx! {
        div {
            SelectRoute { routes: routes }
        }
    })
}

pub fn EmployeeModify(cx: Scope) -> Element {
    let routes = create_router(
        &cx,
        vec![
            ("Basic Information Modify", rsx!(BasicInformationModify {})),
            (
                "Important Information Modify",
                rsx!(ImportantInformationModify {}),
            ),
        ],
    );
    cx.render(rsx! {
        div {
            SelectRoute { routes: routes }
        }
    })
}

pub fn EmployeeChangeModify(cx: Scope) -> Element {
    let submit = move |e: FormEvent| {
        let dp_id = e.values["dp_id"]
            .clone()
            .parse::<usize>()
            .unwrap_or_default();
        let manager_id = e.values["manager_id"]
            .clone()
            .parse::<usize>()
            .unwrap_or_default();
        cx.spawn(async move {
            update_department_manager(manager_id, dp_id).await.unwrap();
        })
    };
    cx.render(rsx! {
        div {
            form {
                onsubmit: submit,
                prevent_default: "onsubmit",
                class: "field",
                div {
                    label { class: "label", "Department Id : " }
                    input { class: "input", name: "dp_id", r#type: "text", placeholder: "Text input" }
                }
                div {
                    label { class: "label", "Manager Id : " }
                    input { class: "input", name: "manager_id", r#type: "text", placeholder: "Text input" }
                }
                br {}
                div {
                    class: "field is-grouped",
                    div {
                        class: "control",
                        button {
                            class: "button is-link",
                            "Submit"
                        }
                    }
                }
            }
        }
    })
}
