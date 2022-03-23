pub trait Beverage {
    fn cost(&self) -> f64;
}

pub struct Espresso;
impl Beverage for Espresso {
    fn cost(&self) -> f64 {
        1.99
    }
}

pub struct HouseBlend;
impl Beverage for HouseBlend {
    fn cost(&self) -> f64 {
        0.89
    }
}

pub trait CondimentDecorator: Beverage {
    fn new(beverage: Box<dyn Beverage>) -> Self;
}

pub struct Mocha {
    beverage: Box<dyn Beverage>,
}

impl CondimentDecorator for Mocha {
    fn new(beverage: Box<dyn Beverage>) -> Mocha {
        Mocha { beverage }
    }
}

impl Beverage for Mocha {
    fn cost(&self) -> f64 {
        self.beverage.cost() + 0.2
    }
}

pub struct Whip {
    beverage: Box<dyn Beverage>,
}

impl CondimentDecorator for Whip {
    fn new(beverage: Box<dyn Beverage>) -> Self {
        Whip { beverage }
    }
}

impl Beverage for Whip {
    fn cost(&self) -> f64 {
        self.beverage.cost() + 0.1
    }
}
