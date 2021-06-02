use crate::FlyBehavior;
use crate::QuackBehavior;

pub trait Duck {
    fn display(&self);
    fn perform_fly<T: FlyBehavior>(&self, fly: &T) {
        fly.fly();
    }
    fn perform_quack<T: QuackBehavior>(&self, quack: &T) {
        quack.quack();
    }
    fn swim(&self) {
        println!("All ducks float, even decoys!");
    }
}

pub struct MallardDuck;
impl Duck for MallardDuck {
    fn display(&self) {
        println!("I'm a real Mallard duck");
    }
}

pub struct ModelDuck;
impl Duck for ModelDuck {
    fn display(&self) {
        println!("I'm a model duck");
    }
}