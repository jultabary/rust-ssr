use minify_js::{minify, Session, TopLevelMode};
use regex::Regex;

pub fn format_css(style: String) -> String {
    let re = Regex::new(r"\s{2,}").unwrap();
    let style = re.replace_all(style.as_str(), "");
    style.replace("\n", "")
}

pub fn format_css_from_bytes(bytes: &[u8]) -> String {
    format_css(String::from_utf8(Vec::from(bytes)).unwrap())
}

pub fn minify_javascript(code: String) -> String {
    let session = Session::new();
    let mut code_minifié = Vec::new();
    minify(&session, TopLevelMode::Global, code.as_bytes(), &mut code_minifié).unwrap();
    std::str::from_utf8(&code_minifié).unwrap().to_string()
}

pub fn minify_javascript_from_bytes(bytes: &[u8]) -> String {
    minify_javascript(String::from_utf8(Vec::from(bytes)).unwrap())
}

pub fn get_by_id() -> String {
    "function getById(id) { return document.getElementById(id); };".to_string()
}

pub fn on_load() -> String {
    "function onLoad(fonction) { window.addEventListener('load', () => { fonction() }) };".to_string()
}

pub fn prevent_default_form_behavior(form_id: &str) -> String {
    format!(
        r#"getById('{}').addEventListener('submit', async function(e) {{ e.preventDefault(); }});"#,
        form_id.to_string()
    )
}