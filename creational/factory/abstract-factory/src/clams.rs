use std::fmt;

pub trait Clams: fmt::Display {}

pub struct FreshClams;
impl Clams for FreshClams {}
impl fmt::Display for FreshClams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Fresh Clams from Long Island Sound")
    }
}

pub struct FrozenClams;
impl Clams for FrozenClams {}
impl fmt::Display for FrozenClams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Frozen Clams from Chesapeake Bay")
    }
}
