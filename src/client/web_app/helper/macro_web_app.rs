macro_rules! add_script {
    ($path:tt) => {
        fn add_script() -> String {
            static CODE: std::sync::OnceLock<String> = std::sync::OnceLock::new();
            return CODE
                .get_or_init(|| {
                    let bytes = include_bytes!($path);
                    return crate::client::web_app::helper::helper::minify_javascript_from_bytes(bytes);
                })
                .clone();
        }
    };
}

pub(crate) use add_script;

macro_rules! add_style {
    ($path:tt) => {
        fn add_style() -> String {
            static STYLE: std::sync::OnceLock<String> = std::sync::OnceLock::new();
            return STYLE
                .get_or_init(|| {
                    let bytes = include_bytes!($path);
                    return crate::client::web_app::helper::helper::format_css_from_bytes(bytes);
                })
                .clone();
        }
    };
}

pub(crate) use add_style;
