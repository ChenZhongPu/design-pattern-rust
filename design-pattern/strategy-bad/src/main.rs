use strategy_bad::{ModelDuck, Duck, FlyRocketPowered};

fn main() {
    let mut duck = ModelDuck::new();
    duck.perform_fly();
    duck.set_fly_behavior(Box::new(FlyRocketPowered));
    duck.perform_fly();

    duck.display();
    duck.swim();
}