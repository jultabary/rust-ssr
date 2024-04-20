use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use crate::client::web_app::helper::helper::{get_by_id, on_load};
use crate::client::web_app::helper::macro_web_app::add_style;
use crate::client::web_app::organism::menu::{Menu, MenuNavigation};
use crate::client::web_app::style::global::add_global_style;

add_style!("page_templace.css");

#[derive(PartialEq, Clone, Props)]
pub struct PageTemplateProps {
    title: String,
    page: MenuNavigation,
    styles: String,
    script: String,
    onload: Option<String>,
    children: Element,
}

pub fn PageTemplate(props: PageTemplateProps) -> Element {
    let styles = format!("{}{}{}", add_global_style(), props.styles, add_style());
    let mut java_script = format!("{}{}{};", on_load(), get_by_id(), props.script);
    let mut on_load= String::from("");
    if props.onload.is_some() {
        on_load = format!(";onLoad(() => {{ {} }});", props.onload.unwrap());
    }
    java_script += on_load.as_str();
    rsx! {
        head {
            title {
                "{props.title.as_str()}"
            },
            style {
                "{styles.as_str()}"
            }
        },
        body {
            script {
                dangerous_inner_html: "{java_script.as_str()}",
            },
            div {
                class: "menu-container",
                Menu {
                    page: props.page
                }
            },
            div {
                class: "main-container",
                main {
                    { props.children }
                }
            }
        }
    }
}

