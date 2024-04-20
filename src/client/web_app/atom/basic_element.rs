use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use crate::client::web_app::helper::helper::format_css;

#[derive(PartialEq, Clone, Props)]
pub struct BasicElementProps {
    script: Option<String>,
    on_load: Option<String>,
    style: Option<String>,
    style_scope_selector: Option<String>,
    children: Element,
}

pub fn BasicElement(props: BasicElementProps) -> Element {
    let mut java_script_code = String::from("");
    let mut css_code = String::from("");
    let is_some_javascript = props.script.is_some() || props.on_load.is_some();
    let is_some_css_code = props.style.is_some();
    if is_some_javascript {
        let script = if props.script.is_some() { props.script.unwrap().clone() } else { String::from("") };
        let is_some_onload_script = props.on_load.is_some();
        let mut on_load = if is_some_onload_script { props.on_load.unwrap().clone() } else { String::from("") };
        if is_some_onload_script {
            on_load = format!(";window.addEventListener('load', () => {{ {} }});", on_load);
        }
        java_script_code = format!("{}{}", script, on_load);
    }
    if is_some_css_code {
        let css_code_pros = props.style.unwrap();
        let selector = props.style_scope_selector.unwrap();
        css_code = format!("@scope ({}) {{ {} }}", selector, format_css(css_code_pros));
    }
    rsx! {
        if is_some_javascript {
            script {
                dangerous_inner_html: java_script_code
            }
        },
        if is_some_css_code {
            style {
                dangerous_inner_html: css_code
            }
        },
        { props.children }
    }
}
