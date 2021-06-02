use crate::{FlyBehavior, QuackBehavior, FlyNoWay, MuteQuack};

pub struct Duck {
    fly_behavior: Box<dyn FlyBehavior>,
    quack_behavior: Box<dyn QuackBehavior>,
}

impl Duck {
    pub fn new() -> Self {
        Duck { fly_behavior: Box::new(FlyNoWay), quack_behavior: Box::new(MuteQuack) }
    }

    pub fn set_fly_behavior(&mut self, fly_behavior: Box<dyn FlyBehavior>) {
        self.fly_behavior = fly_behavior;
    }

    pub fn set_quack_behavior(&mut self, quack_behavior: Box<dyn QuackBehavior>) {
        self.quack_behavior = quack_behavior;
    }

    pub fn perform_fly(&self) {
        self.fly_behavior.fly();
    }

    pub fn perform_quack(&self) {
        self.quack_behavior.quack();
    }
}