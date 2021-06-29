pub trait ICar {
    fn drive(&self);
}

pub struct Car;

impl ICar for Car {
    fn drive(&self) {
        println!("Car has been driven!");
    }
}

pub struct ProxyCar<'a> {
    real_car: &'a dyn ICar,
    driver_age: i32,
}

impl<'a> ICar for ProxyCar<'a> {
    fn drive(&self) {
        if self.driver_age > 16 {
            self.real_car.drive();
        } else {
            println!("Sorry, the driver is too young to drive.");
        }
    }
}

impl<'a> ProxyCar<'a> {
    pub fn new(driver_age: i32, other_car: &'a dyn ICar) -> Self{
        ProxyCar {
            real_car: other_car,
            driver_age
        }
    }
}