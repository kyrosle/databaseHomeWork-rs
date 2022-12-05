use std::ops::Sub;

use backend::{
    query_employee_information, query_employee_salary, search_attendance_information,
    search_max_time_from_salary_record,
};
use dioxus::prelude::*;
pub(crate) fn SalarySettlement(cx: Scope) -> Element {
    let check_id = use_state(&cx, || 0_usize);

    let name = use_state(&cx, || String::from("Man"));

    let content = use_state(&cx, || String::from("Message"));

    let content2 = use_state(&cx, || String::from("Message"));

    let is_show_attendance = use_state(&cx, || true);

    let attendance_info = use_state(&cx, std::vec::Vec::new);

    let show_pass_record = move |_| {
        let attendance_info = attendance_info.clone();
        let check_id = check_id.clone();
        let content = content.clone();
        let name = name.clone();
        cx.spawn({
            async move {
                let vec = search_attendance_information(
                    *check_id.get(),
                    String::from("2000-01-01 00:00:00"),
                )
                .await
                .unwrap();

                if vec.is_empty() {
                    content.set("Not Found!!!".to_owned());
                }

                // println!("{:#?}", &vec);
                attendance_info.set(vec);

                if let Ok(e_name) = query_employee_information(*check_id.get()).await {
                    name.set(e_name.name.unwrap());
                };
            }
        })
    };

    let count_salary = move |_| {
        cx.spawn({
            let check_id = check_id.clone();
            let content2 = content2.clone();
            let content = content.clone();
            async move {
                let id = *check_id.get();
                if let Ok(time) = search_max_time_from_salary_record(id).await {
                    let attendance_vec =
                        search_attendance_information(id, String::from("2000-01-01"))
                            .await
                            .unwrap();

                    let time = chrono::naive::NaiveDate::parse_from_str(time.as_str(), "%Y-%m-%d")
                        .unwrap();
                    let now_time = chrono::Local::now().date_naive();

                    let base_salary = query_employee_salary(id)
                        .await
                        .unwrap()
                        .parse::<f32>()
                        .unwrap()
                        / 30.0;
                    let diff_day = chrono::naive::NaiveDate::sub(time, now_time)
                        .num_days()
                        .abs();
                    let pay_salary = base_salary * diff_day as f32;
                    let mut fine_salary = 0.0;
                    let mut bonus_salary = 0.0;

                    println!("{:#?}", attendance_vec);
                    for at in attendance_vec.iter() {
                        let tmp = at.get_salary();
                        if tmp < 0.0 {
                            fine_salary -= tmp;
                        } else {
                            bonus_salary += tmp;
                        }
                    }
                    // println!("{}", base_salary);
                    // println!("{}", diff_day);
                    // println!("{}", pay_salary);
                    // println!("{}", bonus_salary);
                    // println!("{}", fine_salary);
                    let resp = format!(
                        "You need to pay {}",
                        pay_salary + bonus_salary - fine_salary,
                    );
                    content2.set(resp);
                } else {
                    content.set("can't not count".to_string());
                }
            }
        })
    };

    cx.render(rsx! {
        div {

            div {
                style: "display: flex;",
                // message box
                div {
                    class: "box",
                    style: "height:100px;width:200px;flex:left;padding:10px",
                    label { class: "label", "Message label" }
                    label { "{content}" }
                }
                div {
                    class: "box",
                    style: "height:100px;width:200px;flex:left;padding:10px",
                    label { class: "label", "Message label" }
                    label { "{content2}" }
                }
                div {
                    class: "box",
                    style: "padding: 10px; display:flex;height:100px;",
                    label {
                        class: "label",
                        "Input Employee ID : "
                    }
                    div {
                        style: "width: 150px; padding: 10px",
                        input {
                            class: "input is-tag is-light",
                            oninput: move |e| {
                                if e.value.parse::<usize>().is_err() {
                                    content.set(format!("Invalid Value: {}", e.value.clone()));
                                } else {
                                    content.set(String::from(""));
                                    let id = e.value.clone().parse::<usize>().unwrap();
                                    check_id.set(id);
                                }
                            },
                            placeholder: "usize value",
                        }
                    }

                    div {
                        style: "padding: 10px",
                        button {
                            class: "button is-tag is-light",
                            onclick: show_pass_record,
                            "Show Pass Record"
                        }
                    }
                    div {
                        style: "padding: 10px",
                        button {
                            class: "button is-tag is-light",
                            onclick: count_salary,
                            "Count Salary"
                        }
                    }
                }
            }
            cx.render(
                if *is_show_attendance.get() {
                    rsx!{
                        table {
                            class: "table",
                            thead {
                                tr {
                                    th {"name"}
                                    th {"attendance_name"}
                                    th {"attendance_time"}
                                    th {"base_fine_or_bonus"}
                                    th {"rate_fine_or_bonus"}
                                    th {"record_time"}
                                }
                            }
                            tbody {
                                attendance_info.iter().map(|at| {
                                    let attendance_name = at.attendance_name.clone().unwrap();
                                    let attendance_time = at.attendance_time
                                        .clone()
                                        .unwrap()
                                        .to_string()
                                        .trim_matches(|c| !char::is_numeric(c))
                                        .to_string();

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
                                    let record_time = at.record_time.clone().unwrap();
                                    rsx! {
                                        tr {
                                            td { "{name}" }
                                            td { "{attendance_name}" }
                                            td { "{attendance_time}" }
                                            td { "{b}" }
                                            td { "{r}" }
                                            td { "{record_time}" }
                                        }
                                    }
                                })
                            }
                        }
                    }
                } else {
                    rsx!{
                        div {"Salary"}
                    }
                }
            )
        }
    })
}
