pub trait Duck {
    fn quack(&self);
    fn fly(&self);
}

pub struct MallardDuck;
impl Duck for MallardDuck {
    fn quack(&self) {
        println!("Quack");
    }

    fn fly(&self) {
        println!("I'm flying");
    }
}