use std::cell::RefCell;
use crate::domain::product::Product;
use crate::domain::product_repository::ProductRepository;

pub struct ProductRepositoryAdapterInMemory {
    store: RefCell<Vec<Product>>,
}

impl ProductRepository for ProductRepositoryAdapterInMemory {
    fn save(&self, product: &Product) {
        self.store.borrow_mut().push(
            Self::clone_product(product)
        );
    }

    fn get_all(&self) -> Vec<Product> {
        let store_ref = self.store.borrow();
        store_ref.iter().map(|product| Self::clone_product(product)).collect::<Vec<Product>>()
    }
}

impl ProductRepositoryAdapterInMemory {
    pub fn new() -> Self {
        Self { store: RefCell::new(Vec::new()) }
    }
    fn clone_product(product: &Product) -> Product {
        Product::reconstitute(product.id(), product.name().to_string(), product.details().to_string())
    }
}