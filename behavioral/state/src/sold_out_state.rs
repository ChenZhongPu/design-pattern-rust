use crate::State;
use crate::GumballMachine;

pub struct SoldOutState;

impl State for SoldOutState {
    fn insert_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("You can't insert a quarter, the machine is sold out");
        self
    }

    fn eject_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("You haven't inserted a quarter");
        self
    }

    fn turn_crank(self: Box<Self>) -> Box<dyn State> {
        println!("You turned, but there are no gumballs");
        self 
    }

    fn dispense<'a>(self: Box<Self>, _machine: &'a mut GumballMachine) -> Box<dyn State> {
        println!("No gumball dispensed");
        self
    }    
}