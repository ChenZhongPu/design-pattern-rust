use crate::fly_behavior::FlyBehavior;
use crate::quack_behavior::{QuackBehavior, Quack};
use crate::FlyNoWay;

pub trait Duck {
    fn get_fly_behavior(&self) -> &dyn FlyBehavior;
    fn set_fly_behavior(&mut self, fly_behavior: Box<dyn FlyBehavior>);

    fn get_quack_behavior(&self) -> &dyn QuackBehavior;
    fn set_quack_behavior(&mut self, quack_behavior: Box<dyn QuackBehavior>);

    fn display(&self);

    fn perform_fly(&self) {
        self.get_fly_behavior().fly();
    }

    fn perform_quack(&self) {
        self.get_quack_behavior().quack();
    }

    fn swim(&self) {
        println!("All ducks float, even decoys!");
    }
}

pub struct MallardDuck {
    fly_behavior: Box<dyn FlyBehavior>,
    quack_behavior: Box<dyn QuackBehavior>,
}
impl Duck for MallardDuck {
    fn get_fly_behavior(&self) -> &dyn FlyBehavior {
        &(*self.fly_behavior)
    }

    fn set_fly_behavior(&mut self, fly_behavior: Box<dyn FlyBehavior>) {
        self.fly_behavior = fly_behavior;
    }

    fn get_quack_behavior(&self) -> &dyn QuackBehavior {
        &(*self.quack_behavior)
    }

    fn set_quack_behavior(&mut self, quack_behavior: Box<dyn QuackBehavior>) {
        self.quack_behavior = quack_behavior
    }

    fn display(&self) {
        println!("I'm a real Mallard duck");
    }
}

pub struct ModelDuck {
    fly_behavior: Box<dyn FlyBehavior>,
    quack_behavior: Box<dyn QuackBehavior>,
}
impl Duck for ModelDuck {
    fn get_fly_behavior(&self) -> &dyn FlyBehavior {
        &(*self.fly_behavior)
    }

    fn set_fly_behavior(&mut self, fly_behavior: Box<dyn FlyBehavior>) {
        self.fly_behavior = fly_behavior;
    }

    fn get_quack_behavior(&self) -> &dyn QuackBehavior {
        &(*self.quack_behavior)
    }

    fn set_quack_behavior(&mut self, quack_behavior: Box<dyn QuackBehavior>) {
        self.quack_behavior = quack_behavior
    }

    fn display(&self) {
        println!("I'm a model duck");
    }
}

impl ModelDuck {
    pub fn new() -> Self {
        ModelDuck {
            fly_behavior: Box::new(FlyNoWay),
            quack_behavior: Box::new(Quack)
        }
    }
}