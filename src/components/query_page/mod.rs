mod department_search;
mod personal_search;
mod type_search;

use dioxus::prelude::*;

use crate::components::SelectRoute;
use crate::components::query_page::department_search::DepartmentSearch;

use self::personal_search::PersonalSearch;
use self::type_search::TypeSearch;

use super::create_router;

pub fn QueryPage(cx: Scope) -> Element {
    let routes = create_router(
        &cx,
        vec![
            ("Type Search", rsx!(TypeSearch {})),
            ("Personal Search", rsx!(PersonalSearch {})),
            ("Department Search", rsx!(DepartmentSearch {})),
        ],
    );
    cx.render(rsx! {
        div {
            SelectRoute { routes: routes }
        }
    })
}
