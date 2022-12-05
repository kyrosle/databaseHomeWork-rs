use backend::{
    display_attendance_type_information, display_department_information,
    display_employee_information, display_political_information, display_post_information,
    prelude::{AttendanceType, Department, Employee, Political, Post},
};
use dioxus::{events::FormEvent, prelude::*};

pub fn TypeSearch(cx: Scope) -> Element {
    let show_index = use_state(&cx, || 0_usize);
    let show_table = vec![
        rsx!(ShowEmployee {}),
        rsx!(ShowDepartment {}),
        rsx!(ShowPost {}),
        rsx!(ShowPolitical {}),
        rsx!(ShowAttendanceType {}),
    ]
    .into_iter()
    .enumerate()
    .filter(|(idx, _)| idx == show_index.get())
    .map(|(_, ln)| ln);
    let submit = move |e: FormEvent| {
        let index = e.values["show"].clone().parse::<usize>().unwrap();
        show_index.set(index);
    };
    cx.render(rsx! {
        div {
            div {
                form {
                    onsubmit: submit,
                    prevent_default: "onsubmit",
                    div {
                        class: "field",
                        label { class: "label", "Select : " }
                        div {
                            class: "control",
                            div {
                                class: "select",
                                select {
                                    name: "show",
                                    option{ value: "0", "All Employee" }
                                    option{ value: "1", "All Department" }
                                    option{ value: "2", "All Post" }
                                    option{ value: "3", "All Political" }
                                    option{ value: "4", "All Attendance Type" }
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
                                "Search"
                            }
                        }
                    }
                }
            }

            div {
                show_table
            }
        }
    })
}

pub fn ShowEmployee(cx: Scope) -> Element {
    let none_vec = Vec::<Employee>::new();
    let table = use_future(&cx, (), |_| async move {
        display_employee_information().await.unwrap()
    })
    .value()
    .unwrap_or(&none_vec);

    let table = table.iter().map(|e| {
        let id = e.id.unwrap();
        let name = e.name.clone().unwrap();
        let post = e.post_name.clone().unwrap();
        let department = e.department_name.clone().unwrap();
        let political = e.political_name.clone().unwrap();
        let birth = e.birth.clone().unwrap();
        let salary = e.salary.clone().unwrap().0.parse::<f32>().unwrap();
        let postscript = e.postscript.clone().unwrap();
        rsx! {
            tr {
                td {"{id}"}
                td {"{name}"}
                td {"{post}"}
                td {"{department}"}
                td {"{political}"}
                td {"{birth}"}
                td {"{salary}"}
                td {"{postscript}"}
            }
        }
    });
    cx.render(rsx! {
        div {
            table {
                class: "table",
                thead {
                    tr {
                        th {"id"}
                        th {"name"}
                        th {"post"}
                        th {"department"}
                        th {"political"}
                        th {"birth"}
                        th {"salary"}
                        th {"introduce"}
                    }
                }
                tbody {
                    table
                }
            }
        }
    })
}
pub fn ShowDepartment(cx: Scope) -> Element {
    let none_vec = Vec::<Department>::new();
    let table = use_future(&cx, (), |_| async move {
        display_department_information().await.unwrap()
    })
    .value()
    .unwrap_or(&none_vec);

    let table = table.iter().map(|e| {
        let id = e.id.unwrap();
        let name = e.name.clone().unwrap();
        let manager_name = e.manager_name.clone().unwrap_or_default();
        rsx! {
            tr {
                td {"{id}"}
                td {"{name}"}
                td {"{manager_name}"}
            }
        }
    });
    cx.render(rsx! {
        div {
            table {
                class: "table",
                thead {
                    tr {
                        th {"id"}
                        th {"name"}
                        th {"manager_name"}
                    }
                }
                tbody {
                    table
                }
            }
        }
    })
}
pub fn ShowPost(cx: Scope) -> Element {
    let none_vec = Vec::<Post>::new();
    let table = use_future(&cx, (), |_| async move {
        display_post_information().await.unwrap()
    })
    .value()
    .unwrap_or(&none_vec);

    let table = table.iter().map(|e| {
        let id = e.id.unwrap();
        let name = e.name.clone().unwrap();
        let salary = e.salary.clone().unwrap().0.parse::<f32>().unwrap();

        rsx! {
            tr {
                td {"{id}"}
                td {"{name}"}
                td {"{salary}"}
            }
        }
    });
    cx.render(rsx! {
        div {
            table {
                class: "table",
                thead {
                    tr {
                        th {"id"}
                        th {"name"}
                        th {"salary"}
                    }
                }
                tbody {
                    table
                }
            }
        }
    })
}
pub fn ShowPolitical(cx: Scope) -> Element {
    let none_vec = Vec::<Political>::new();
    let table = use_future(&cx, (), |_| async move {
        display_political_information().await.unwrap()
    })
    .value()
    .unwrap_or(&none_vec);

    let table = table.iter().map(|e| {
        let id = e.id.unwrap();
        let name = e.name.clone().unwrap();
        rsx! {
            tr {
                td {"{id}"}
                td {"{name}"}
            }
        }
    });
    cx.render(rsx! {
        div {
            table {
                class: "table",
                thead {
                    tr {
                        th {"id"}
                        th {"name"}
                    }
                }
                tbody {
                    table
                }
            }
        }
    })
}
pub fn ShowAttendanceType(cx: Scope) -> Element {
    let none_vec = Vec::<AttendanceType>::new();
    let table = use_future(&cx, (), |_| async move {
        display_attendance_type_information().await.unwrap()
    })
    .value()
    .unwrap_or(&none_vec);

    let table = table.iter().map(|at| {
        let attendance_id = at.attendance_id.clone().unwrap();
        let attendance_name = at.attendance_name.clone().unwrap();
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
                td {"{attendance_id}"}
                td {"{attendance_name}"}
                td {"{b}"}
                td {"{r}"}
            }
        }
    });
    cx.render(rsx! {
        div {
            table {
                class: "table",
                thead {
                    tr {
                        th {"attendance_id"}
                        th {"attendance_name"}
                        th {"base_fine_or_bonus"}
                        th {"rate_fine_or_bonus"}
                    }
                }
                tbody {
                    table
                }
            }
        }
    })
}
