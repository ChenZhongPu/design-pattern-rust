use crate::GumballMachine;

pub trait State {
    fn insert_quarter(self: Box<Self>) -> Box<dyn State>;
    fn eject_quarter(self: Box<Self>) -> Box<dyn State>;
    fn turn_crank(self: Box<Self>) -> Box<dyn State>;
    fn dispense<'a>(self: Box<Self>, machine: &'a mut GumballMachine) -> Box<dyn State>;
}