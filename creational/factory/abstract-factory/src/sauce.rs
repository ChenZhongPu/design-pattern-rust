use std::fmt;

pub trait Sauce: fmt::Display {}

pub struct PlumTomatoSauce;
impl Sauce for PlumTomatoSauce {}
impl fmt::Display for PlumTomatoSauce {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Tomato sauce with plum tomatoes")
    }
}

pub struct MarinaraSauce;
impl Sauce for MarinaraSauce {}
impl fmt::Display for MarinaraSauce {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Marinara Sauce")
    }
}
