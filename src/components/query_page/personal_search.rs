use backend::{
    display_employee_change_information, display_salary_record_information,
    prelude::{Attendance, PersonalChange, SalaryRecord},
    search_attendance_information,
};
use dioxus::prelude::*;

use crate::components::{create_router, SelectRoute};
pub fn PersonalSearch(cx: Scope) -> Element {
    let routes = create_router(
        &cx,
        vec![
            ("Show Salary", rsx!(ShowSalary {})),
            ("Show Employee Change", rsx!(ShowEmployeeChange {})),
            ("Show Attendance", rsx!(ShowAttendance {})),
        ],
    );

    cx.render(rsx! {
        div {
            SelectRoute {routes: routes}
        }
    })
}

pub fn ShowSalary(cx: Scope) -> Element {
    let id = use_state(&cx, || String::from(""));
    let none_vec = Vec::<SalaryRecord>::new();
    let f = use_future(&cx, (), |_| {
        let id = id.clone();
        async move {
            display_salary_record_information(id.get().parse::<usize>().unwrap_or_default())
                .await
                .unwrap()
        }
    });
    let table = f.value().unwrap_or(&none_vec);

    let table = table.iter().map(|s| {
        let id = s.id.unwrap();
        let salary = s
            .salary
            .clone()
            .unwrap()
            .to_string()
            .trim_matches(|c| !char::is_numeric(c))
            .to_string();
        let basic_salary = s
            .basic_salary
            .clone()
            .unwrap()
            .to_string()
            .trim_matches(|c| !char::is_numeric(c))
            .to_string();
        let bonus = s
            .bonus
            .clone()
            .unwrap()
            .to_string()
            .trim_matches(|c| !char::is_numeric(c))
            .to_string();
        let fine = s
            .fine
            .clone()
            .unwrap()
            .to_string()
            .trim_matches(|c| !char::is_numeric(c))
            .to_string();
        let start_time = s.starting_time.clone().unwrap();
        let cut_of_time = s.cut_of_time.clone().unwrap();

        rsx! {
            tr {
                td {"{id}"}
                td {"{salary}"}
                td {"{basic_salary}"}
                td {"{bonus}"}
                td {"{fine}"}
                td {"{start_time}"}
                td {"{cut_of_time}"}
            }
        }
    });
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
                        th {"id"}
                        th {"salary"}
                        th {"basic_salary"}
                        th {"bonus"}
                        th {"fine"}
                        th {"start_time"}
                        th {"cut_of_time"}
                    }
                }
                tbody {
                    table
                }
            }
        }
    })
}
pub fn ShowEmployeeChange(cx: Scope) -> Element {
    let id = use_state(&cx, || String::from(""));
    let none_vec = Vec::<PersonalChange>::new();
    let f = use_future(&cx, (), |_| {
        let id = id.clone();
        async move {
            display_employee_change_information(id.get().parse::<usize>().unwrap_or_default())
                .await
                .unwrap()
        }
    });
    let table = f.value().unwrap_or(&none_vec);

    let table = table.iter().map(|s| {
        let id = s.id.unwrap();
        let change_time = s.change_time.clone().unwrap();
        let department_name = s.department_name.clone().unwrap();
        let post_name = s.post_name.clone().unwrap();

        rsx! {
            tr {
                td {"{id}"}
                td {"{change_time}"}
                td {"{department_name}"}
                td {"{post_name}"}
            }
        }
    });
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
                        th {"id"}
                        th {"change_time"}
                        th {"department_name"}
                        th {"post_name"}
                    }
                }
                tbody {
                    table
                }
            }
        }
    })
}
pub fn ShowAttendance(cx: Scope) -> Element {
    let id = use_state(&cx, || String::from(""));
    let none_vec = Vec::<Attendance>::new();
    let f = use_future(&cx, (), |_| {
        let id = id.clone();
        async move {
            search_attendance_information(
                id.get().parse::<usize>().unwrap_or_default(),
                String::from("2000-07-05"),
            )
            .await
            .unwrap()
        }
    });
    let table = f.value().unwrap_or(&none_vec);

    let table = table.iter().map(|s| {
        let name = s.name.clone().unwrap();
        let attendance_name = s.attendance_name.clone().unwrap();
        let base_fine_or_bonus = s
            .base_fine_or_bonus
            .clone()
            .unwrap()
            .to_string()
            .trim_matches(|c| !char::is_numeric(c))
            .to_string();
        let rate_fine_or_bonus = s
            .rate_fine_or_bonus
            .clone()
            .unwrap()
            .to_string()
            .trim_matches(|c| !char::is_numeric(c))
            .to_string();
        let attendance_time = s
            .attendance_time
            .clone()
            .unwrap()
            .to_string()
            .trim_matches(|c| !char::is_numeric(c))
            .to_string();

        let record_time = s.record_time.clone().unwrap();
        rsx! {
            tr {
                td {"{name}"}
                td {"{attendance_name}"}
                td {"{base_fine_or_bonus}"}
                td {"{rate_fine_or_bonus}"}
                td {"{attendance_time}"}
                td {"{record_time}"}
            }
        }
    });
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
                        td {"name"}
                        td {"attendance_name"}
                        td {"base_fine_or_bonus"}
                        td {"rate_fine_or_bonus"}
                        td {"attendance_time"}
                        td {"record_time"}
                    }
                }
                tbody {
                    table
                }
            }
        }
    })
}
