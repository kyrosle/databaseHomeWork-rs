use backend::{
    display_attendance_type_information, display_department_information,
    display_political_information, display_post_information, insert_department_information,
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
    let change_form = vec![rsx!(h1{"Change Form"}), rsx!(AddDepartment {})]
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
                    }
                    s if s == "Attendance Type" => {
                        let attendance_vec = display_attendance_type_information()
                            .await
                            .unwrap()
                            .into_iter()
                            .map(|v| v.attendance_name.unwrap())
                            .collect::<Vec<_>>();
                        show_table.set(attendance_vec);
                    }
                    s if s == "Political Type" => {
                        display_vec!(display_political_information, show_table);
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
    };

    cx.render(rsx! {
        div {
            form {
                onsubmit: submit_event,
                prevent_default: "onsubmit",

                div {
                    label { class: "label", "Department Name : " }
                    input { class: "input", name: "name", r#type: "text", placeholder: "Text input" }
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
        }
    })
}
