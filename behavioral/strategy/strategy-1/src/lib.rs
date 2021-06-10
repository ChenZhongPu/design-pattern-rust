mod duck;
mod fly_behavior;
mod quack_behavior;

pub use fly_behavior::FlyBehavior;
pub use fly_behavior::FlyNoWay;
pub use fly_behavior::FlyRocketPowered;
pub use fly_behavior::FlyWithWings;

pub use quack_behavior::MuteQuack;
pub use quack_behavior::QuackBehavior;
pub use quack_behavior::Squeak;

pub use duck::Duck;
