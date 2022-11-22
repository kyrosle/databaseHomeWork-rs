use backend::display_department_information;
use dioxus::{events::FormEvent, prelude::*};

pub(crate) fn AddRegulations(cx: Scope) -> Element {
    let choices = vec!["Department", "Post", "Employee Change", "Political"]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let choices_show = choices.iter().map(|s| {
        rsx! {
            option { "{s}" }
        }
    });
    let show_table = use_state(&cx, || vec![]);

    let show_button = move |e: FormEvent| {
        cx.spawn(async move {
            match &e.values["show"] {
                s if s == "Department" => {
                    let department_vec = display_department_information()
                        .await
                        .unwrap()
                        .into_iter()
                        .map(|d| d.name.unwrap())
                        .collect::<Vec<_>>();
                    // println!("{:#?}", department_vec);  -has value-
                    show_table.set(department_vec);
                }
                s if s == "Post" => {}
                s if s == "Employee Change" => {}
                s if s == "Salary" => {}

                _ => {}
            }
        })
    };
    cx.render(rsx! {
        div {
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
        }
    })
}
