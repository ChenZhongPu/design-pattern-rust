use crate::State;
use crate::NoQuarterState;
use crate::GumballMachine;
use crate::SoldState;

pub struct HasQuarterState;

impl State for HasQuarterState {
    fn insert_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("You can't insert another quarter");
        self
    }

    fn eject_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("Quarter returned");
        Box::new(NoQuarterState)
    }

    fn turn_crank(self: Box<Self>) -> Box<dyn State> {
        println!("You turned...");
        Box::new(SoldState)
    }

    fn dispense<'a>(self: Box<Self>, _machine: &'a mut GumballMachine) -> Box<dyn State> {
        println!("No gumball dispensed");
        self
    }
}
