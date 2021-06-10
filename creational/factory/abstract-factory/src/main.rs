use abstract_factory::*;

fn main() {
    let ny_store = NYPizzaStore;
    let chicago_store = ChicagoPizzaStore;

    let pizza = ny_store.order_pizza("cheese");
    println!("Ethan ordered a {}", pizza);
    println!("------------");
    let pizza = chicago_store.order_pizza("cheese");
    println!("Joel ordered a {}", pizza);
    println!("------------");
    let pizza = ny_store.order_pizza("veggie");
    println!("Ethan ordered a {}", pizza);
    println!("------------");
    let pizza = chicago_store.order_pizza("veggie");
    println!("Joel ordered a {}", pizza);
}
