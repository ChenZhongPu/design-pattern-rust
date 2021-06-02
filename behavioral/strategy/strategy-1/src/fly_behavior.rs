pub trait FlyBehavior {
    fn fly(&self);
}

pub struct FlyWithWings;
impl FlyBehavior for FlyWithWings {
    fn fly(&self) {
        println!("I'm flying!");
    }
}

pub struct FlyNoWay;
impl FlyBehavior for FlyNoWay {
    fn fly(&self) {
        println!("I can't fly");
    }
}

pub struct FlyRocketPowered;
impl FlyBehavior for FlyRocketPowered {
    fn fly(&self) {
        println!("I'm flying with a rocket");
    }
}
