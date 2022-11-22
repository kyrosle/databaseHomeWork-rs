#![allow(non_snake_case)]
use crate::components::*;
use dioxus::prelude::*;

mod add_attendance;
mod add_employee;
mod add_regulations;
mod add_salary;

use add_attendance::AddAttendance;
use add_employee::AddEmployee;
use add_regulations::AddRegulations;
use add_salary::SalarySettlement;

pub fn AddPage(cx: Scope) -> Element {
    let routes = create_router(
        &cx,
        vec![
            ("   Add Employee  ", rsx!(AddEmployee {})),
            (" Add Regulations ", rsx!(AddRegulations {})),
            (" Add Attendance  ", rsx!(AddAttendance {})),
            ("Salary Settlement", rsx!(SalarySettlement {})),
        ],
    );
    cx.render(rsx! {
        SelectRoute { routes: routes }
    })
}

#[macro_export]
macro_rules! display_list {
    ($cx: expr, $name: pat, $func: ident) => {
        let $name = match use_future(&$cx, (), |_| async move { $func().await.unwrap() }).value() {
            Some(value) => value
                .iter()
                .map(|v| {
                    let name = v.name.clone().unwrap();
                    rsx! {
                        option{ "{name}" }
                    }
                })
                .collect::<Vec<_>>(),
            None => Vec::<LazyNodes>::new(),
        };
    };
}
