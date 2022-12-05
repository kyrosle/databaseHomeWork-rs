use backend::{
    delete_attendance_type_information, delete_department_information, delete_employee_information,
    delete_political, delete_post_by_id,
};
use dioxus::{events::FormEvent, prelude::*};
macro_rules! match_type {
    ($($val: ident,)*; $option_vec: ident) => {
        enum MatchType {
            $($val,)*
            None,
        }
        $option_vec = vec![$(stringify!($val),)*];
        impl From<&str> for MatchType {
            fn from(s: &str) -> MatchType {
                match s {
                    $(stringify!($val) => MatchType::$val,)*
                    _ => MatchType::None,
                }
            }
        }
    };
}

pub fn DeletePage(cx: Scope) -> Element {
    let option_vec: Vec<&str>;
    match_type![AttendanceType, Employee, Department, Post, PoliticalType,;option_vec];

    let on_submit = move |e: FormEvent| {
        let select_type = e.values["select"].clone();
        let id = e.values["id"].clone().parse::<usize>().unwrap_or_default();

        cx.spawn(async move {
            match MatchType::from(&select_type[..]) {
                MatchType::AttendanceType => delete_attendance_type_information(id).await.unwrap(),
                MatchType::Employee => delete_employee_information(id).await.unwrap(),
                MatchType::Department => delete_department_information(id).await.unwrap(),
                MatchType::Post => delete_post_by_id(id).await.unwrap(),
                MatchType::PoliticalType => delete_political(id).await.unwrap(),
                MatchType::None => println!("Id is None"),
            }
        });
    };
    let show_type = option_vec.iter().map(|v| rsx!(option{"{v}"}));
    cx.render(rsx! {
        div {
            class: "box",
            style: "padding: 5px",
            form {
                onsubmit: on_submit,
                prevent_default: "onsubmit",
                div { class: "label", "Select Delete Object" }
                div {
                    class: "select",
                    select {
                        name: "select",
                        show_type
                    }
                }
                label { class: "label", "Input the correspond Id : " }
                div {
                    style: "width: 200px",
                    input {
                        class: "input",
                        name: "id",
                        r#type: "text",
                        placeholder: "the correspond Id"
                    }
                }
                br {}
                div {
                    class: "field is-grouped",
                    div {
                        class: "control",
                        button {
                            class: "button is-link",
                            "Delete"
                        }
                    }
                }
            }
        }
    })
}
