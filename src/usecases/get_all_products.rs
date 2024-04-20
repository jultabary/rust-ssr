use std::rc::Rc;
use serde::{Serialize};
use crate::domain::product_repository::ProductRepository;

pub struct GetAllProducts {
    product_repository: Rc<dyn ProductRepository>
}

impl GetAllProducts {
    pub fn new(product_repository: Rc<dyn ProductRepository>) -> Self {
        Self { product_repository }
    }
    pub fn execute(&self) -> Vec<ProductSchema> {
        let products = self.product_repository.get_all();
        let response = products.iter()
            .map(|product| ProductSchema::new(product.id().value().to_string(), product.name().to_string(), product.details().to_string()))
            .collect::<Vec<ProductSchema>>();
        response
    }
}

#[derive(Serialize)]
pub struct ProductSchema {
    id: String,
    name: String,
    details: String
}

impl ProductSchema {
    pub fn new(id: String, name: String, details: String) -> Self {
        Self { id, name, details }
    }
}