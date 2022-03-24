use crate::CheesePizza;
use crate::ClamPizza;
use crate::Pizza;
use crate::VeggiePizza;

pub struct SimplePizzaFactory;

impl SimplePizzaFactory {
    pub fn create_pizza(t: &str) -> Box<dyn Pizza> {
        match t {
            "cheese" => Box::new(CheesePizza::new()),
            "clam" => Box::new(ClamPizza::new()),
            "veggie" => Box::new(VeggiePizza::new()),
            _ => panic!("unknown pizza type"),
        }
    }
}
