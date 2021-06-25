#[derive(Debug)]
pub struct MenuItem {
    name: String,
    description: String,
    vegetarian: bool,
    price: f64,
}

impl MenuItem {
    pub fn empty() -> Self {
        MenuItem::new("", "", false, 0.0)
    }

    pub fn new(name: &str, description: &str, vegetarian: bool, price: f64) -> Self {
        MenuItem {
            name: String::from(name),
            description: String::from(description),
            vegetarian, price
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn is_vegetarian(&self) -> bool {
        self.vegetarian
    }

    pub fn price(&self) -> f64 {
        self.price
    }
}