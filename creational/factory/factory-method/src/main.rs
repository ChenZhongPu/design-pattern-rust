use factory_method::*;

fn main() {
    let ny_store = NYPizzaStore;
    let chicago_store = ChicagoPizzaStore;

    let pizza = ny_store.order_pizza("cheese");
    println!("Ethan ordered a {}", pizza.name());

    let pizza = chicago_store.order_pizza("cheese");
    println!("Joel ordered a {}", pizza.name());
}
