use crate::fly_behavior::FlyBehavior;
use crate::quack_behavior::QuackBehavior;

pub trait Duck {
    fn display(&self);
    fn perform_fly<T: FlyBehavior>(&self, fly_behavior: T) {
        fly_behavior.fly();
    }
    fn perform_quack<T: QuackBehavior>(&self, quack_behavior: T) {
       quack_behavior.quack();
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