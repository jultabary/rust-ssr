use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct ProducttId {
    valeur: Uuid
}

impl ProducttId {
    pub fn new() -> Self {
        Self { valeur: Uuid::new_v4() }
    }

    pub fn value(&self) -> Uuid {
        self.valeur
    }
}

pub struct Product {
    id: ProducttId,
    name: String,
    details: String
}

impl Product {
    pub fn new(name: String, details: String) -> Self {
        Self { id: ProducttId::new(), name, details }
    }

    pub fn reconstitute(id: ProducttId, name: String, details: String) -> Product {
        Self {
            id, name, details
        }
    }

    pub fn id(&self) -> ProducttId {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn details(&self) -> &str {
        &self.details
    }
}