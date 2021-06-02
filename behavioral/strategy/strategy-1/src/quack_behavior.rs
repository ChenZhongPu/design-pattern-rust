pub trait QuackBehavior {
    fn quack(&self);
}

pub struct Quack;
impl QuackBehavior for Quack {
    fn quack(&self) {
        println!("Quack");
    }
}

pub struct MuteQuack;
impl QuackBehavior for MuteQuack {
    fn quack(&self) {
        println!("<< Silence >>");
    }
}

pub struct Squeak;
impl QuackBehavior for Squeak {
    fn quack(&self) {
        println!("Squeak");
    }
}
