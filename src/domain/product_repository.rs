use crate::domain::product::Product;

pub trait ProductRepository {
    fn save(&self, product: &Product);
    fn get_all(&self) -> Vec<Product>;
}