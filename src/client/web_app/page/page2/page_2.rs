use dioxus::prelude::*;
use dioxus::core_macro::rsx;
use crate::client::web_app::organism::menu::MenuNavigation;
use crate::client::web_app::organism::page_template::PageTemplate;

pub fn Page2() -> Element {
    rsx! {
        PageTemplate {
            page: MenuNavigation::Page2,
            title: "Page 2".to_string(),
            styles: "".to_string(),
            script: "".to_string(),
            onload: None,
            {
                rsx! {
                    div {
                        h1 {
                            "Page 2"
                        }
                    }
                }
            }
        }
    }
}
