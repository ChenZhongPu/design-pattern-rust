use std::fmt;

pub trait Veggies: fmt::Display {}

pub struct Onion;
impl Veggies for Onion {}
impl fmt::Display for Onion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Onion")
    }
}

pub struct Garlic;
impl Veggies for Garlic {}
impl fmt::Display for Garlic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Onion")
    }
}

pub struct MushRoom;
impl Veggies for MushRoom {}
impl fmt::Display for MushRoom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Mushrooms")
    }
}

pub struct Eggplant;
impl Veggies for Eggplant {}
impl fmt::Display for Eggplant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Eggplant")
    }
}

pub struct RedPepper;
impl Veggies for RedPepper {}
impl fmt::Display for RedPepper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Red Pepper")
    }
}
