use crate::pizza::{CheesePizza, ClamPizza, Pizza, VeggiePizza};
use crate::pizza_ingredient_factory::{ChicagoPizzaIngredientFactory, NYPizzaIngredientFactory};

pub trait PizzaStore {
    fn create_pizza(&self, t: &str) -> Box<dyn Pizza>;

    fn order_pizza(&self, t: &str) -> Box<dyn Pizza> {
        let mut pizza = self.create_pizza(t);
        println!("--- Making a {} ---", pizza.name());
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.boxes();
        pizza
    }
}

pub struct ChicagoPizzaStore;
impl PizzaStore for ChicagoPizzaStore {
    fn create_pizza(&self, t: &str) -> Box<dyn Pizza> {
        let factory = Box::new(ChicagoPizzaIngredientFactory);
        match t {
            "cheese" => {
                Box::new(CheesePizza::new(factory, "Chicago Style Cheese Pizza"))
            },
            "veggie" => {
                Box::new(VeggiePizza::new(factory, "Chicago Style Veggie Pizza"))
            },
            "clam" => {
                Box::new(ClamPizza::new(factory, "Chicago Style Clam Pizza"))
            },
            _ => panic!("no such type"),
        }
    }
}

pub struct NYPizzaStore;
impl PizzaStore for NYPizzaStore {
    fn create_pizza(&self, t: &str) -> Box<dyn Pizza> {
        let factory = Box::new(NYPizzaIngredientFactory);
        let pizza: Box<dyn Pizza>;
        match t {
            "cheese" => {
                pizza = Box::new(CheesePizza::new(factory, "New York Style Cheese Pizza"));
            }
            "veggie" => {
                pizza = Box::new(VeggiePizza::new(factory, "New York Style Veggie Pizza"));
            }
            "clam" => {
                pizza = Box::new(ClamPizza::new(factory, "New York Style Clam Pizza"));
            }
            _ => panic!("no such type"),
        }
        pizza
    }
}
