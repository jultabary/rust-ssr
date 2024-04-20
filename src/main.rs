#![allow(non_snake_case)]

use std::rc::Rc;
use std::sync::Arc;
use axum::http::Method;
use axum::response::{Html, Redirect};
use axum::Router;
use axum::routing::{get, post};
use log::LevelFilter;
use tower_http::cors::{Any, CorsLayer};
use crate::client::api::product_rest_controller::{add_product_api, get_all_products_api};
use crate::client::path::{PAGE_1, PAGE_2, PRODUCT_API};
use crate::client::web_app::page::page1::page_1::Page1;
use crate::client::web_app::page::page2::page_2::Page2;
use crate::infrastructure::product_repository_adapter_in_memory::ProductRepositoryAdapterInMemory;
use crate::logger::SimpleLogger;
use crate::usecases::add_product_to_store::AddProductToStore;
use crate::usecases::get_all_products::GetAllProducts;

pub mod usecases;
pub mod infrastructure;
pub mod domain;
pub mod client;
mod logger;


pub struct App {
    get_all_products: GetAllProducts,
    add_product_to_store: AddProductToStore
}

impl App {
    pub fn new() -> Self {
        let product_repository = Rc::new(ProductRepositoryAdapterInMemory::new());
        Self { get_all_products: GetAllProducts::new(product_repository.clone()), add_product_to_store: AddProductToStore::new(product_repository.clone()) }
    }
}

unsafe impl Sync for App {}
unsafe impl Send for App {}

static LOGGER: SimpleLogger = SimpleLogger {};


#[tokio::main]
async fn main() {
    let _ = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    let app = App::new();
    let app_arc = Arc::new(app);
    axum::serve(
        listener,
        Router::new()
            .route(PRODUCT_API, post(add_product_api))
            .route(PRODUCT_API, get(get_all_products_api))
            .with_state(app_arc)
            .layer(CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any).allow_headers(Any))
            .route("/", get(|| async { Redirect::permanent(PAGE_1) }))
            .route(PAGE_1, get(Html(format!("<!DOCTYPE html><html>{}</html>", dioxus_ssr::render_element(Page1())))))
            .route(PAGE_2, get(Html(format!("<!DOCTYPE html><html>{}</html>", dioxus_ssr::render_element(Page2())))))
            .into_make_service(),
    )
    .await
    .unwrap();
}
