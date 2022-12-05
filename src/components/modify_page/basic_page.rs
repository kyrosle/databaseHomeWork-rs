use backend::update_employee_information;
use dioxus::prelude::*;
pub fn BasicInformationModify(cx: Scope) -> Element {
    let id = use_state(&cx, || String::from(""));
    let name = use_state(&cx, || String::from(""));
    let health = use_state(&cx, || String::from(""));
    let introduce = use_state(&cx, || String::from(""));
    let birth = use_state(&cx, || String::from(""));

    let change_name = move |_| {
        let name = name.get().clone();
        let id = id.clone().parse::<usize>().unwrap();
        cx.spawn(async move {
            update_employee_information("name".to_string(), name, id)
                .await
                .unwrap()
        });
    };
    let change_health = move |_| {
        let health = health.get().clone();
        let id = id.clone().parse::<usize>().unwrap();
        cx.spawn(async move {
            update_employee_information("health_status".to_string(), health, id)
                .await
                .unwrap()
        });
    };
    let change_birth = move |_| {
        let birth = birth.get().clone();
        let id = id.clone().parse::<usize>().unwrap();
        cx.spawn(async move {
            update_employee_information("birth".to_string(), birth, id)
                .await
                .unwrap()
        });
    };
    let change_introduce = move |_| {
        let introduce = introduce.get().clone();
        let id = id.clone().parse::<usize>().unwrap();
        cx.spawn(async move {
            update_employee_information("postscript".to_string(), introduce, id)
                .await
                .unwrap()
        });
    };

    cx.render(rsx!{
        div {
            class: "box",
            div {
                div {
                    label {class: "label", "Employee Id : "}
                    input { class: "input", value: "{id}", oninput: |e| id.set(e.value.clone()), r#type: "text" }
                }
                div {
                    label {class: "label", "Name : "}
                    input { class: "input", value: "{name}", oninput: |e| name.set(e.value.clone()), r#type: "text" }
                }
                br {}
                div { class: "control",  button {class: "button is-link", onclick: change_name, "Change Name"} }
                br {}
                div {
                    label {class: "label", "Health Status : "}
                    input { class: "input", value: "{health}", oninput: |e| health.set(e.value.clone()), r#type: "text" }
                }
                br {}
                div { class: "control",  button {class: "button is-link", onclick: change_health, "Change Health Status"} }
                br {}
                div {
                    label { class: "label", "Birth : " }
                    input { class: "input", value: "{birth}", oninput: |e| birth.set(e.value.clone()), r#type: "date" }
                }
                br {}
                div { class: "control",  button {class: "button is-link", onclick: change_birth, "Change Birth Data"} }
                br {}
                div {
                    label {class: "label", "Employee Introduce : "}
                    input { class: "input", value: "{introduce}", oninput: |e| introduce.set(e.value.clone()), r#type: "text" }
                }
                br {}
                div { class: "control",  button {class: "button is-link", onclick: change_introduce, "Change Introduce"} }
                br {}
            }
        }
    })
}

pub fn ImportantInformationModify(cx: Scope) -> Element {
    let id = use_state(&cx, || String::from(""));
    let department = use_state(&cx, || String::from(""));
    let post = use_state(&cx, || String::from(""));
    let salary = use_state(&cx, || String::from(""));

    let change_department = move |_| {
        let id = id.clone().parse::<usize>().unwrap();
        let department = department.get().clone();
        cx.spawn(async move {
            update_employee_information("department_id".to_string(), department, id)
                .await
                .unwrap()
        })
    };
    let change_post = move |_| {
        let id = id.clone().parse::<usize>().unwrap();
        let post = post.get().clone();
        cx.spawn(async move {
            update_employee_information("post_id".to_string(), post, id)
                .await
                .unwrap()
        })
    };
    let change_salary = move |_| {
        let id = id.clone().parse::<usize>().unwrap();
        let salary = salary.get().clone();
        cx.spawn(async move {
            update_employee_information("salary".to_string(), salary, id)
                .await
                .unwrap()
        })
    };

    cx.render(rsx! {
        div {
            div {
                label {class: "label", "Employee Id : "}
                input { class: "input", value: "{id}", oninput: |e| id.set(e.value.clone()), r#type: "text" }
            }
            div {
                label {class: "label", "Department Id : "}
                input { class: "input", value: "{department}", oninput: |e| department.set(e.value.clone()), r#type: "text" }
            }
            br {}
            div { class: "control",  button {class: "button is-link", onclick: change_department, "Change Name"} }
            br {}
            div {
                label {class: "label", "Post Id : "}
                input { class: "input", value: "{post}", oninput: |e| post.set(e.value.clone()), r#type: "text" }
            }
            br {}
            div { class: "control",  button {class: "button is-link", onclick: change_post, "Change Health Status"} }
            br {}
            div {
                label {class: "label", "Salary : "}
                input { class: "input", value: "{salary}", oninput: |e| salary.set(e.value.clone()), r#type: "text" }
            }
            div { class: "control",  button {class: "button is-link", onclick: change_salary, "Change Birth Data"} }
        }
    })
}
