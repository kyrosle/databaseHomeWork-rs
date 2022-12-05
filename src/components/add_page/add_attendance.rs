use backend::{display_attendance_type_information, insert_attendance_information};
use dioxus::{prelude::*, events::FormEvent};
pub(crate) fn AddAttendance(cx: Scope) -> Element {
    let attendance_vec = use_state(&cx, std::vec::Vec::new);
    let query = use_future(&cx, (), |_| {
        let attendance_vec = attendance_vec.clone();
        async move {
            let vec = display_attendance_type_information().await.unwrap();
            attendance_vec.set(vec);
        }
    });

    let submit = move |ev: FormEvent| {
        let employee_id = ev.values["employee_id"].clone().parse::<usize>().unwrap();
        let attendance_type_id = ev.values["attendance_type_id"].clone().parse::<usize>().unwrap();
        let time = ev.values["time"].clone().parse::<f32>().unwrap();

        cx.spawn(async move {
            insert_attendance_information(employee_id, attendance_type_id, time).await.unwrap();
        })
    };

    
    let content = use_state(&cx, || String::from("Message"));
    cx.render(rsx! {
        div {
            style: "display: flex;flex-row: row nowrap;height: 500px",
            div {
                class: "box",
                style: "padding: 20px;margin: 20px;width: 500px",
                label { class: "label", "Attendance Type" }
                button {
                    class: "button",
                    onclick: move |_| {
                        query.restart();
                    },
                    "Refresh"
                }
                br {}
                table {
                    class: "table",
                    thead {
                        tr{
                            th { "Attendance Number" }
                            th { "Attendance Type Name" }
                            th { "Rewards And Punishments" }
                            th { "Reward Coefficient" }
                        }
                    }
                    tbody {
                        attendance_vec.iter().map(|at| {
                            let id = at.attendance_id.unwrap();
                            let name = at.attendance_name.clone().unwrap();

                            let b = at
                                .base_fine_or_bonus
                                .clone()
                                .unwrap()
                                .to_string()
                                .trim_matches(|c| !char::is_numeric(c))
                                .to_string();

                            let r = at
                                .rate_fine_or_bonus
                                .clone()
                                .unwrap()
                                .to_string()
                                .trim_matches(|c| !char::is_numeric(c))
                                .to_string();
                            rsx! {
                                tr {
                                    td { "{id}" }
                                    td { "{name}" }
                                    td { "{b}" }
                                    td { "{r}" }
                                }
                            }
                        })
                    }
                }
            }

            div {
                style: "flex: 1; padding: 5px; ",
                class: "box",

                // message box
                div {
                    class: "box",
                    style: "height: 100px",
                    label { class: "label", "Message label" }
                    label { "{content}" }
                }
            
                form {
                    onsubmit: submit,
                    prevent_default: "onsubmit",
                    div {
                        oninput: move |e| {
                            if e.value.parse::<usize>().is_err() {
                                content.set(format!("Invalid Value: {}", e.value.clone()));
                            } else {
                                content.set(String::from(""));
                            }
                        },
                        label { class: "label", "Employee Id : "}
                        input { class: "input", name: "employee_id", r#type: "text", placeholder: "Text input" }
                    }
                    div {
                        oninput: move |e| {
                            if e.value.parse::<usize>().is_err() {
                                content.set(format!("Invalid Value: {}", e.value.clone()));
                            } else {
                                content.set(String::from(""));
                            }
                        },
                        label { class: "label", "Attendance Type Id : "}
                        input { class: "input", name: "attendance_type_id", r#type: "text", placeholder: "Text input" }
                    }
                    div {
                        oninput: move |e| {
                            if e.value.parse::<f32>().is_err() {
                                content.set(format!("Invalid Value: {}", e.value.clone()));
                            } else {
                                content.set(String::from(""));
                            }
                        },
                        label { class: "label", "Time : "}
                        input { class: "input", name: "time", r#type: "text", placeholder: "Text input" }
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
        }
    })
}
