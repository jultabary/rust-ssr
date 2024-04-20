use std::rc::Rc;

use crate::domain::product::Product;
use crate::domain::product_repository::ProductRepository;

pub struct AddProductToStore {
    product_repository: Rc<dyn ProductRepository>
}

impl AddProductToStore {
    pub fn new(product_repository: Rc<dyn ProductRepository>) -> Self {
        Self { product_repository }
    }
    pub fn execute(&self, name: String, details: String) -> String {
        let product = Product::new(name, details);
        self.product_repository.save(&product);
        product.id().value().to_string()
    }
}
