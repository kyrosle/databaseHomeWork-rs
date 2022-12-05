use backend::{
    display_attendance_type_information, display_department_information,
    display_political_information, display_post_information, insert_attendance_type_information,
    insert_department_information, insert_political, insert_post,
};
use dioxus::{events::FormEvent, prelude::*};

macro_rules! display_vec {
    ($func: ident, $show_table: expr) => {
        let vec = $func()
            .await
            .unwrap()
            .into_iter()
            .map(|v| v.name.unwrap())
            .collect::<Vec<_>>();
        $show_table.set(vec);
    };
}

pub(crate) fn AddRegulations(cx: Scope) -> Element {
    let choices = vec!["Department", "Post", "Attendance Type", "Political Type"]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let choices_show = choices.iter().map(|s| {
        rsx! {
            option { "{s}" }
        }
    });
    let show_table = use_state(&cx, std::vec::Vec::new);

    let change_form_index = use_state(&cx, || 0_usize);
    let change_form = vec![
        rsx!(h1{"Change Form"}),
        rsx!(AddDepartment {}),
        rsx!(AddPost {}),
        rsx!(AddAttendance {}),
        rsx!(AddPolitical {}),
    ]
    .into_iter()
    .enumerate()
    .filter(|(idx, _)| idx == change_form_index.get())
    .map(|(_, ln)| ln);

    let show_button = move |e: FormEvent| {
        cx.spawn({
            let show_table = show_table.clone();
            let change_form_index = change_form_index.clone();

            async move {
                match &e.values["show"] {
                    s if s == "Department" => {
                        display_vec!(display_department_information, show_table);
                        change_form_index.set(1);
                    }
                    s if s == "Post" => {
                        display_vec!(display_post_information, show_table);
                        change_form_index.set(2);
                    }
                    s if s == "Attendance Type" => {
                        let attendance_vec = display_attendance_type_information()
                            .await
                            .unwrap()
                            .into_iter()
                            .map(|v| v.attendance_name.unwrap())
                            .collect::<Vec<_>>();
                        show_table.set(attendance_vec);
                        change_form_index.set(3);
                    }
                    s if s == "Political Type" => {
                        display_vec!(display_political_information, show_table);
                        change_form_index.set(4);
                    }
                    _ => {}
                };
            }
        })
    };

    cx.render(rsx! {
        div {
            style: "display: flex;flex-row: row nowrap;height: 500px",
            div {
                class: "box",
                style: "padding: 5px",
                form {
                    onsubmit: show_button,
                    prevent_default: "onsubmit",
                    class: "field",

                    div {
                        class: "field",
                        label { class: "label", "Select : " }
                        div {
                            class: "control",
                            div {
                                class: "select",
                                select {
                                    name: "show",
                                    choices_show
                                }
                            }
                        }
                    }
                    div {
                        class: "field is-grouped",
                        div {
                            class: "control",
                            button {
                                class: "button is-link",
                                "Show"
                            }
                        }
                    }
                }
                table {
                    class: "table",
                    thead {
                        tr { th { title: "Name", "Name" } }
                    }
                    tbody {
                        show_table.get().iter().map(|s| rsx!(tr { th {"{s}"} }))
                    }
                }
            }
            div {
                style: "flex: 1; padding: 5px",
                div {
                    change_form
                }
            }
        }
    })
}

fn AddDepartment(cx: Scope) -> Element {
    let submit_event = move |e: FormEvent| {
        let name = e.values["name"].clone();
        cx.spawn(async move {
            insert_department_information(name).await.unwrap();
        })
        // println!("{:#?}", e);
    };

    cx.render(rsx! {
        div {
            style: "padding: 5px",
            form {
                onsubmit: submit_event,
                prevent_default: "onsubmit",
                div {
                    label { class: "label", "Department Name : " }
                    input { class: "input", name: "name", r#type: "text", placeholder: "Text input" }
                }
                br {}
                div {
                    class: "field is-grouped",
                    div {
                        class: "control",
                        button {
                            class: "button is-link",
                            "Add"
                        }
                    }
                }
            }
        }
    })
}

fn AddPost(cx: Scope) -> Element {
    let submit_event = move |e: FormEvent| {
        let name = e.values["name"].clone();
        let salary_line = e.values["salary"].clone().parse::<f32>().unwrap();
        let salary_id = e.values["salary_id"].clone().parse::<usize>().unwrap();
        cx.spawn(async move {
            insert_post(name, salary_id, salary_line).await.unwrap();
        })
        // println!("{:#?}", e);
    };

    let content = use_state(&cx, || String::from("Message"));

    cx.render(rsx! {
        div {
            style: "padding: 5px",
            // message box
            div {
                class: "box",
                style: "height: 100px",
                label { class: "label", "Message label" }
                label { "{content}" }
            }
            form {
                onsubmit: submit_event,
                prevent_default: "onsubmit",
                div {
                    label { class: "label", "Post Name : " }
                    input { class: "input", name: "name", r#type: "text", placeholder: "Text input" }
                }
                br {}
                div {
                    oninput: move |e| {
                        if e.value.parse::<f32>().is_err() {
                            content.set(format!("Invalid Value: {}", e.value.clone()));
                        } else {
                            content.set(String::from(""));
                        }
                    },
                    label { class: "label", "Level Salary Line : "}
                    input { class: "input", name: "salary", r#type: "text", placeholder: "Text input" }
                }
                div {
                    class: "field",
                    label { class: "label", "Level : " }
                    div {
                        class: "control",
                        div {
                            class: "select",
                            select {
                                name: "salary_id",
                                vec!["1","2","3","4","5"].into_iter().map(|v| rsx!(option{"{v}"}))
                            }
                        }
                    }
                }
                div {
                    class: "field is-grouped",
                    div {
                        class: "control",
                        button {
                            class: "button is-link",
                            "Add"
                        }
                    }
                }
            }
        }
    })
}

fn AddAttendance(cx: Scope) -> Element {
    let submit_event = move |e: FormEvent| {
        let name = e.values["name"].clone();
        let rp_value = e.values["rp_value"].clone().parse::<f32>().unwrap();
        let rc_value = e.values["rc_value"].clone().parse::<f32>().unwrap();
        cx.spawn(async move {
            insert_attendance_type_information(name, rp_value, rc_value)
                .await
                .unwrap();
        })
        // println!("{:#?}", e);
    };

    let content = use_state(&cx, || String::from("Message"));

    cx.render(rsx! {
        div {
            style: "padding: 5px",
            div {
                class: "box",
                style: "height: 100px",
                label { class: "label", "Message label" }
                label { "{content}" }
            }
            form {
                onsubmit: submit_event,
                prevent_default: "onsubmit",
                div {
                    label { class: "label", "Attendance Name : " }
                    input { class: "input", name: "name", r#type: "text", placeholder: "Text input" }
                }
                br {}

                div {
                    oninput: move |e| {
                        if e.value.parse::<f32>().is_err() {
                            content.set(format!("Invalid Value: {}", e.value.clone()));
                        } else {
                            content.set(String::from(""));
                        }
                    },
                    label { class: "label", "Rewards And Punishments : "}
                    input { class: "input", name: "rp_value", r#type: "text", placeholder: "Text input" }
                }
                br {}

                div {
                    oninput: move |e| {
                        if e.value.parse::<f32>().is_err() {
                            content.set(format!("Invalid Value: {}", e.value.clone()));
                        } else {
                            content.set(String::from(""));
                        }
                    },
                    label { class: "label", "Reward Coefficient : "}
                    input { class: "input", name: "rc_value", r#type: "text", placeholder: "Text input" }
                }
                br {}

                div {
                    class: "field is-grouped",
                    div {
                        class: "control",
                        button {
                            class: "button is-link",
                            "Add"
                        }
                    }
                }
            }
        }
    })
}

fn AddPolitical(cx: Scope) -> Element {
    let submit_event = move |e: FormEvent| {
        let name = e.values["name"].clone();
        cx.spawn(async move {
            insert_political(name).await.unwrap();
        })
        // println!("{:#?}", e);
    };

    cx.render(rsx! {
        div {
            style: "padding: 5px",
            form {
                onsubmit: submit_event,
                prevent_default: "onsubmit",
                div {
                    label { class: "label", "Political Name : " }
                    input { class: "input", name: "name", r#type: "text", placeholder: "Text input" }
                }
                br {}
                div {
                    class: "field is-grouped",
                    div {
                        class: "control",
                        button {
                            class: "button is-link",
                            "Add"
                        }
                    }
                }
            }
        }
    })
}
