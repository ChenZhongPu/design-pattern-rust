use crate::Pizza;

pub trait PizzaStore {
    fn create_pizza(&self, t: &str) -> Box<dyn Pizza>;
    fn order_pizza(&self, t: &str) -> Box<dyn Pizza> {
        let pizza = self.create_pizza(t);
        println!("--- Making a {} ---", pizza.name());
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.boxes();
        pizza
    }
}
