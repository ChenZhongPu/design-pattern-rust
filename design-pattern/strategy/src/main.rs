use strategy::{MallardDuck, Duck, FlyWithWings, FlyRocketPowered, Squeak};

fn main() {
    let duck = MallardDuck;
    duck.swim();
    duck.perform_fly(FlyWithWings);
    duck.perform_fly(FlyRocketPowered);
    duck.display();
    duck.perform_quack(Squeak);
}