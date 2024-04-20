use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::App;
use crate::usecases::get_all_products::ProductSchema;

pub async fn add_product_api(State(app): State<Arc<App>>, Json(payload): Json<AddProductPayload>) -> Result<Json<ProductIdSchema>, StatusCode> {
    let id = app.add_product_to_store.execute(payload.name, payload.details);
    Ok(Json(ProductIdSchema { id }))
}

pub async fn get_all_products_api(State(app): State<Arc<App>>) -> Result<Json<Vec<ProductSchema>>, StatusCode> {
    Ok(Json(app.get_all_products.execute()))
}

#[derive(Deserialize)]
pub struct AddProductPayload {
    name: String,
    details: String
}

#[derive(Serialize)]
pub struct ProductIdSchema {
    id: String
}