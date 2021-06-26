use crate::State;
use crate::HasQuarterState;
use crate::GumballMachine;

pub struct NoQuarterState;

impl State for NoQuarterState {
    fn insert_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("You inserted a quarter");
        Box::new(HasQuarterState)
    }

    fn eject_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("You haven't inserted a quarter");
        self
    }

    fn turn_crank(self: Box<Self>) -> Box<dyn State> {
        println!("You turned, but there's no quarter");
        self 
    }

    fn dispense<'a>(self: Box<Self>, _machine: &'a mut GumballMachine) -> Box<dyn State> {
        println!("You need to pay first");
        self
    }
}