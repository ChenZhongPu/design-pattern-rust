use strategy_1::{Duck, FlyWithWings, Squeak};

fn main() {
    let mut duck = Duck::new();
    duck.perform_fly();
    duck.perform_quack();
    println!("----------");
    duck.set_fly_behavior(Box::new(FlyWithWings));
    duck.perform_fly();
    println!("----------");
    duck.set_fly_behavior(Box::new(FlyWithWings));
    duck.set_quack_behavior(Box::new(Squeak));
    duck.perform_quack();
}
