use crate::Pizza;
use crate::SimplePizzaFactory;

pub struct PizzaStore;

impl PizzaStore {
    pub fn order_pizza(&self, t: &str) -> Box<dyn Pizza> {
        let pizza = SimplePizzaFactory::create_pizza(t);
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.boxes();
        pizza
    }
}
