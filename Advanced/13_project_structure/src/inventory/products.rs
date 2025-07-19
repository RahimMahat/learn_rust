#[derive(Debug)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32,
}

impl Item {
    // you can access the names or items of the parent module using super keyword and for a child
    // module to use the parent module's item the item doesn't have to be public.
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        Self {
            name,
            category,
            quantity,
        }
    }
}
