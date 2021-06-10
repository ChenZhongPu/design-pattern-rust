use std::fmt;

pub trait Cheese: fmt::Display {}

pub struct MozzarellaCheese;
impl Cheese for MozzarellaCheese {}
impl fmt::Display for MozzarellaCheese {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Shredded Mozzarella")
    }
}

pub struct ParmesanCheese;
impl Cheese for ParmesanCheese {}
impl fmt::Display for ParmesanCheese {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Shredded Parmesan")
    }
}

pub struct ReggianoCheese;
impl Cheese for ReggianoCheese {}
impl fmt::Display for ReggianoCheese {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Reggiano Cheese")
    }
}
