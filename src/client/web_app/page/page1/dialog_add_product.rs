use dioxus::prelude::*;
use crate::client::path::PRODUCT_API;
use crate::client::web_app::atom::basic_element::BasicElement;
use crate::client::web_app::helper::helper::prevent_default_form_behavior;
use crate::client::web_app::helper::macro_web_app::{add_script, add_style};

add_script!("dialog_add_product.js");
add_style!("dialog_add_product.css");

const FORM_ID: &'static str = "add-product-form";
const NAME_INPUT_ID: &'static str = "name-input-id";
const DETAILS_INPUT_ID: &'static str = "details-input-id";

#[derive(PartialEq, Clone, Props)]
pub struct AddProductDialogProps {
    id: String,
    callback: String,
}

pub fn AddProductDialog(props: AddProductDialogProps) -> Element {
    rsx! {
        BasicElement {
            script: add_script(),
            on_load: prevent_default_form_behavior(FORM_ID),
            style: add_style(),
            style_scope_selector: ".dialog",
            dialog {
                id: props.id.clone(),
                class: "dialog",
                "aria-label" : "Add product",
                div {
                    h2 {
                        "Add product"
                    }
                },
                div {
                    form {
                        id: FORM_ID,
                        "onsubmit": format!("add_product('{}', () => {{ {} }}, '{}', '{}', '{}'); return false;",
                            PRODUCT_API, props.callback.clone(), NAME_INPUT_ID, DETAILS_INPUT_ID, props.id.clone()),
                        div {
                            label {
                                r#for: NAME_INPUT_ID,
                                "Name"
                            },
                            input {
                                id: NAME_INPUT_ID,
                                r#type: "text",
                                required: "true"
                            }
                        },
                        div {
                            label {
                                r#for: DETAILS_INPUT_ID,
                                "Details"
                            },
                            textarea {
                                id: DETAILS_INPUT_ID,
                                placeholder: "product details",
                                rows: "5",
                                required: "true"
                            }
                        },
                        div {
                            button {
                                r#type: "submit",
                                form: FORM_ID,
                                "Add"
                            }
                        }
                    }
                }
            }
        }
    }
}