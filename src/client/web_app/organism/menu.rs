use dioxus::prelude::*;
use dioxus::prelude::{rsx, Props};
use crate::client::path::{PAGE_1, PAGE_2};
use crate::client::web_app::helper::macro_web_app::add_style;

add_style!("menu.css");

#[derive(PartialEq, Clone)]
pub enum MenuNavigation {
    Page1,
    Page2,
}

#[derive(PartialEq, Props, Clone)]
pub struct MenuProps {
    page: MenuNavigation,
}

pub fn Menu(props: MenuProps) -> Element {
    rsx! {
        nav {
            "aria-label": "Main navigation",
            style {
                dangerous_inner_html: add_style()
            },
            ul {
                li {
                    "--data-current-page": props.page.eq(&MenuNavigation::Page1),
                    a {
                        href: PAGE_1,
                        "A page"
                    }
                },
                li {
                    "--data-current-page": props.page.eq(&MenuNavigation::Page2),
                    a {
                        href: PAGE_2,
                        "Another page"
                    }
                }
            }
        }
    }
}
