use simple_factory::*;

fn main() {
    let store = PizzaStore;
    let pizza = store.order_pizza("cheese");
    println!("We ordered a {}", pizza.get_name());

    let pizza = store.order_pizza("veggie");
    println!("We ordered a {}", pizza.get_name());
}
