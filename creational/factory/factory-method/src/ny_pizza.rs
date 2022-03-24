use crate::Pizza;
use crate::PizzaStore;

pub struct NYPizzaStore;
impl PizzaStore for NYPizzaStore {
    fn create_pizza(&self, t: &str) -> Box<dyn Pizza> {
        match t {
            "cheese" => Box::new(NYStyleCheesePizza::new()),
            "veggie" => Box::new(NYStyleVeggiePizza::new()),
            "clam" => Box::new(NYStyleClamPizza::new()),
            _ => panic!("no such type"),
        }
    }
}

pub struct NYStyleCheesePizza {
    name: String,
}

impl NYStyleCheesePizza {
    pub fn new() -> Self {
        NYStyleCheesePizza {
            name: String::from("NY Style Sauce and Cheese Pizza"),
        }
    }
}

impl Pizza for NYStyleCheesePizza {
    fn name(&self) -> &str {
        &self.name
    }
}

pub struct NYStyleClamPizza {
    name: String,
}

impl NYStyleClamPizza {
    pub fn new() -> Self {
        NYStyleClamPizza {
            name: String::from("NY Style Clam Pizza"),
        }
    }
}

impl Pizza for NYStyleClamPizza {
    fn name(&self) -> &str {
        &self.name
    }
}

pub struct NYStyleVeggiePizza {
    name: String,
}

impl NYStyleVeggiePizza {
    pub fn new() -> Self {
        NYStyleVeggiePizza {
            name: String::from("NY Style Veggie Pizza"),
        }
    }
}

impl Pizza for NYStyleVeggiePizza {
    fn name(&self) -> &str {
        &self.name
    }
}
