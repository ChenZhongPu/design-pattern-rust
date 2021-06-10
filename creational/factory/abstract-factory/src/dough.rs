use std::fmt;

pub trait Dough: fmt::Display {}

pub struct ThickCrustDough;
impl Dough for ThickCrustDough {}
impl fmt::Display for ThickCrustDough {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "ThickCrust style extra thick crust dough")
    }
}

pub struct ThinCrustDough;
impl Dough for ThinCrustDough {}
impl fmt::Display for ThinCrustDough {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Thin Crust Dough")
    }
}
