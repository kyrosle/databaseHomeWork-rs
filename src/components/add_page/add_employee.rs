use backend::prelude::*;
use dioxus::events::FormEvent;
use dioxus::prelude::*;

use crate::display_list;

pub(crate) fn AddEmployee(cx: Scope) -> Element {
    display_list!(cx, department, display_department_information);
    display_list!(cx, post, display_post_information);
    display_list!(cx, political, display_political_information);

    let content = use_state(&cx, || String::from("Message"));
    let submit = move |evt: FormEvent| {
        println!("{:#?}", evt);
        cx.spawn(async move {
            let value = &evt.values;
            let post_id = query_post_by_name(value["post"].clone()).await.unwrap();
            let department_id = query_department_id(value["department"].clone())
                .await
                .unwrap();
            let political_id = query_political_id(value["political"].clone())
                .await
                .unwrap();

            let employee_id = insert_employee_information(
                post_id,
                department_id,
                value["name"].clone(),
                value["birth"].clone(),
                political_id,
                value["health"].clone(),
                value["salary"].parse::<f32>().unwrap(),
                value["postscript"].clone(),
            )
            .await
            .unwrap();
            println!("{}", employee_id);
            let now_time = chrono::Local::now().date_naive();
            insert_salary_record_information(employee_id, 0.0, 0.0, 0.0, 0.0, now_time.to_string())
                .await
                .unwrap();
        })
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
                input { class: "input", name: "name", r#type: "text", placeholder: "Text input" }
            }

            // health status input
            div {
                label { class: "label", "Health Status : "}
                input { class: "input", name: "health", r#type: "text", placeholder: "Text input" }
            }

            // salary input
            div {
                oninput: move |e| {
                    if e.value.parse::<f32>().is_err() {
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
                    name: "postscript",
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
