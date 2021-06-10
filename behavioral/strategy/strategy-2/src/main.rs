use strategy_2::Duck;
use strategy_2::FlyRocketPowered;
use strategy_2::FlyWithWings;
use strategy_2::MallardDuck;

fn main() {
    let fly_wing = FlyWithWings;
    let fly_rocket = FlyRocketPowered;
    let duck = MallardDuck;
    duck.swim();
    duck.display();
    duck.perform_fly(&fly_wing);
    duck.perform_fly(&fly_rocket);
}
