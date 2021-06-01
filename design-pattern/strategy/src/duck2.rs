use crate::{FlyBehavior, QuackBehavior};

pub struct Duck;

impl Duck {
    pub fn perform_fly<T: FlyBehavior>(&self, fly_behavior: T) {
       fly_behavior.fly();
    }
    pub fn perform_quack<T: QuackBehavior>(&self, quack_behavior: T) {
        quack_behavior.quack();
    }
}