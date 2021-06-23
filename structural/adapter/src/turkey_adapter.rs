use crate::{Turkey, Duck};

pub struct TurkeyAdapter {
    turkey: Box<dyn Turkey>,
}

impl TurkeyAdapter {
    pub fn new(turkey: Box<dyn Turkey>) -> Self {
        TurkeyAdapter { turkey }
    }
}

impl Duck for TurkeyAdapter {
    fn quack(&self) {
        self.turkey.gobble();
    }

    fn fly(&self) {
        for _ in 0..5 {
            self.turkey.fly();
        }
    }
}