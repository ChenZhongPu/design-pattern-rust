use std::fmt;

pub trait Pepperoni: fmt::Display {}

pub struct SlicedPepperoni;
impl Pepperoni for SlicedPepperoni {}
impl fmt::Display for SlicedPepperoni {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Sliced Pepperoni")
    }
}
