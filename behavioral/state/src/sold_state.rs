use crate::State;
use crate::GumballMachine;
use crate::NoQuarterState;
use crate::SoldOutState;

pub struct SoldState;

impl State for SoldState {
    fn insert_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("Please wait, we're already giving you a gumball");
        self
    }

    fn eject_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("Sorry, you already turned the crank");
        self
    }

    fn turn_crank(self: Box<Self>) -> Box<dyn State> {
        println!("Turning twice doesn't get you another gumball!");
        self 
    }

    fn dispense<'a>(self: Box<Self>, machine: &'a mut GumballMachine) -> Box<dyn State> {
        machine.release_ball();
        if machine.count() > 0 {
            println!("Released...");
            Box::new(NoQuarterState)
        } else {
            println!("Oops, out of gumballs!");
            Box::new(SoldOutState)
        }
    }
}