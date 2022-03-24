use crate::Pizza;
use crate::PizzaStore;

pub struct ChicagoPizzaStore;
impl PizzaStore for ChicagoPizzaStore {
    fn create_pizza(&self, t: &str) -> Box<dyn Pizza> {
        match t {
            "cheese" => Box::new(ChicagoStyleCheesePizza::new()),
            "veggie" => Box::new(ChicagoStyleVeggiePizza::new()),
            "clam" => Box::new(ChicagoStyleClamPizza::new()),
            _ => panic!("no such type"),
        }
    }
}

pub struct ChicagoStyleCheesePizza {
    name: String,
}

impl ChicagoStyleCheesePizza {
    pub fn new() -> Self {
        ChicagoStyleCheesePizza {
            name: String::from("Chicago Style Deep Dish Cheese Pizza"),
        }
    }
}

impl Pizza for ChicagoStyleCheesePizza {
    fn name(&self) -> &str {
        &self.name
    }

    fn cut(&self) {
        println!("Cutting the pizza into square slices")
    }
}

pub struct ChicagoStyleClamPizza {
    name: String,
}

impl ChicagoStyleClamPizza {
    pub fn new() -> Self {
        ChicagoStyleClamPizza {
            name: String::from("Chicago Style Clam Pizza"),
        }
    }
}

impl Pizza for ChicagoStyleClamPizza {
    fn name(&self) -> &str {
        &self.name
    }

    fn cut(&self) {
        println!("Cutting the pizza into square slices");
    }
}

pub struct ChicagoStyleVeggiePizza {
    name: String,
}

impl ChicagoStyleVeggiePizza {
    pub fn new() -> Self {
        ChicagoStyleVeggiePizza {
            name: String::from("Chicago Deep Dish Veggie Pizza"),
        }
    }
}

impl Pizza for ChicagoStyleVeggiePizza {
    fn name(&self) -> &str {
        &self.name
    }

    fn cut(&self) {
        println!("Cutting the pizza into square slices");
    }
}
