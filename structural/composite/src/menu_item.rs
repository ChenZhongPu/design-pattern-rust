use crate::MenuComponent;

pub struct MenuItem {
    name: String,
    description: String,
    vegetarian: bool,
    price: f64,
}

impl MenuItem {
    pub fn new(name: &str, description: &str, vegetarian: bool, price: f64) -> Self {
        MenuItem {
            name: String::from(name),
            description: String::from(description),
            vegetarian, price
        }
    }
}

impl MenuComponent for MenuItem {
    fn name(&self) -> &str {
       &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn price(&self) -> f64 {
        self.price
    }

    fn is_vegetarian(&self) -> bool {
        self.vegetarian
    }

    fn print(&self) {
        print!(" {}", self.name);
        if self.vegetarian {
            println!("(v)");
        }
        println!(", {}", self.price);
        println!("   -- {}", self.description);
    }
}