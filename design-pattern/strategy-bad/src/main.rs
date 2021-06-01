use strategy_bad::{ModelDuck, Duck, FlyRocketPowered, Duck2, FlyNoWay, MuteQuack, FlyWithWings};

fn main() {
    let mut duck = ModelDuck::new();
    duck.perform_fly();
    duck.set_fly_behavior(Box::new(FlyRocketPowered));
    duck.perform_fly();

    duck.display();
    duck.swim();

    println!("-------a new duck----------");

    let mut duck2 = Duck2::new(Box::new(FlyNoWay), Box::new(MuteQuack));
    duck2.perform_fly();
    duck2.set_fly_behavior(Box::new(FlyWithWings));
    duck2.perform_fly();
}