use dioxus::prelude::*;
use crate::client::path::PRODUCT_API;
use crate::client::web_app::helper::macro_web_app::add_script;
use crate::client::web_app::organism::menu::MenuNavigation;
use crate::client::web_app::organism::page_template::PageTemplate;
use crate::client::web_app::page::page1::dialog_add_product::AddProductDialog;

add_script!("page_1.js");

const PRODUCTS_LIST_ID: &'static str = "products-list";
const ADD_PRODUCT_DIALOG_ID: &'static str = "add-product-dialog";

pub fn Page1() -> Element {
    rsx! {
        PageTemplate {
            page: MenuNavigation::Page1,
            title: "My products".to_string(),
            styles: "".to_string(),
            script: add_script(),
            onload: format!("get_all_products('{}', '{}');", PRODUCT_API, PRODUCTS_LIST_ID)
            {
                rsx! {
                    AddProductDialog {
                        id: ADD_PRODUCT_DIALOG_ID,
                        callback: format!("get_all_products('{}', '{}')", PRODUCT_API, PRODUCTS_LIST_ID)
                    },
                    h1 {
                        "My products"
                    },
                    div {
                        button {
                            "onclick": format!("getById('{}').showModal();", ADD_PRODUCT_DIALOG_ID),
                            "Add a product"
                        }
                    },
                    span {
                        id: "error-message",
                        "aria-live": "polite"
                    },
                    ul {
                        id: PRODUCTS_LIST_ID,
                        class: "catalogue",
                        "aria-live": "polite",
                    }
                }
            }
        }
    }
}
