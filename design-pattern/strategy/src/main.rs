use strategy::{MallardDuck, Duck, FlyWithWings, FlyRocketPowered, Squeak, Duck2};

fn main() {
    let duck = MallardDuck;
    duck.swim();
    duck.perform_fly(FlyWithWings);
    duck.perform_fly(FlyRocketPowered);
    duck.display();
    duck.perform_quack(Squeak);

    println!("------a new duck-------");
    let duck = Duck2;
    duck.perform_fly(FlyWithWings);
    duck.perform_fly(FlyRocketPowered);
}