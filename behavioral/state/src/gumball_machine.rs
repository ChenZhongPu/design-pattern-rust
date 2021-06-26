use crate::State;
use crate::SoldOutState;
use crate::NoQuarterState;

pub struct GumballMachine {
    state: Option<Box<dyn State>>,
    count: usize,
}

impl GumballMachine {
    pub fn new(count: usize) -> Self {
        if count == 0 {
            GumballMachine {
                state: Some(Box::new(SoldOutState)),
                count
            }
        } else {
            GumballMachine {
                state: Some(Box::new(NoQuarterState)),
                count
            }
        }
    }

    pub fn insert_quarter(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.insert_quarter());
        }
    }

    pub fn eject_quarter(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.eject_quarter());
        }
    }

    pub fn turn_crank(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.turn_crank());
        }
        if let Some(s) = self.state.take() {
            self.state = Some(s.dispense(self));
        }
    }

    pub fn release_ball(&mut self) {
        if self.count > 0 {
            self.count -= 1;
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}