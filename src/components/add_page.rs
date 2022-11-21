#![allow(non_snake_case)]
use crate::components::*;
use dioxus::prelude::*;
use backend::prelude::*;

pub fn AddPage(cx: Scope) -> Element {
    let routes = create_router(
        &cx,
        vec![
            ("   Add Employee  ", rsx!(AddEmployee {})),
            (" Add Regulations ", rsx!(AddRegulations {})),
            (" Add Attendance  ", rsx!(AddAttendance {})),
            ("Salary Settlement", rsx!(SalarySettlement {})),
        ],
    );
    cx.render(rsx! {
        SelectRoute { routes: routes }
    })
}

macro_rules! display_list {
    ($cx: expr, $name: pat, $func: ident) => {
        let $name = match use_future(&$cx, (),  |_| async move {
            $func()
            .await
            .unwrap()
        }).value() {
            Some(value)  => {
                value.iter()
                .map(|v| {
                    let name = v.name.clone().unwrap();
                    rsx!{ 
                        option{ "{name}" } 
                    }
                }).collect::<Vec<_>>()
            }
            None => Vec::<LazyNodes>::new() 
        };
    };
}

fn AddEmployee(cx: Scope) -> Element {
    display_list!(cx, department, display_department_information);
    display_list!(cx, post, display_post_information);
    display_list!(cx, political, display_political_information);

    let content = use_state(&cx, || {
        String::from("Message")
    });
    let submit = move |evt| {
        // cx.spawn(async move {
        // }) 
        println!("{:?}", evt);
    };
    cx.render(rsx! {
        br {}
        // message box
        div {
            class: "box",
            style: "height: 100px",
            label { class: "label", "Message label" }
            label { "{content}" }
        }

        form {
            // events
            onsubmit: submit,
            prevent_default: "onsubmit",
            class: "field",

            // name input
            div {
                label { class: "label", "Name : " }
                input { class: "input", name: "user_name", r#type: "text", placeholder: "Text input" }
            }

            // health status input
            div {
                label { class: "label", "Health Status : "}
                input { class: "input", name: "health_status", r#type: "text", placeholder: "Text input" }
            }

            // salary input
            div {
                oninput: move |e| { 
                    if e.value.parse::<isize>().is_err() {
                        content.set(format!("Invalid Value: {}", e.value.clone()));
                    } else {
                        content.set(String::from(""));
                    }
                },
                label { class: "label", "Salary : "}
                input { class: "input", name: "salary", r#type: "text", placeholder: "Text input" }
            }

            // department choice
            div {
                class: "field",
                label { class: "label", "Deportment : " }
                div {
                    class: "control",
                    div {
                        class: "select",
                        select {
                            name: "department",
                            department
                        }
                    }
                }
            }

            // post choice
            div {
                class: "field",
                label { class: "label", "Post : " }
                div {
                    class: "control",
                    div {
                        class: "select",
                        select {
                            name: "post",
                            post
                        }
                    }
                }
            }

            // political choice
            div {
                class: "field",
                label { class: "label", "Political : " }
                div {
                    class: "control",
                    div {
                        class: "select",
                        select {
                            name: "political",
                            political
                        }
                    }
                }
            }

            // birth input
            div {
                label { class: "label", "Birth : " }
                input {
                    class: "date",
                    name: "birth",
                    r#type: "date",
                }
            }
            br {}

            // Employee status input
            div {
                label { class: "label", "Employee Status : "}
                textarea {
                    class: "textarea",
                    placeholder: "e.g. I am a employee...",
                    "rows": "5",
                }
            }
            br {}

            // Submit Button
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
    })
}











fn AddRegulations(cx: Scope) -> Element {
    cx.render(rsx! {
        div {

        }
    })
}

fn AddAttendance(cx: Scope) -> Element {
    cx.render(rsx! {
        div {

        }
    })
}

fn SalarySettlement(cx: Scope) -> Element {
    cx.render(rsx! {
        div {

        }
    })
}
