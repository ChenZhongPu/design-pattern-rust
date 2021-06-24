use crate::CaffeineBeverage;

pub struct Tea;

impl CaffeineBeverage for Tea {
    fn brew(&self) {
        println!("Steeping the tea");
    }

    fn add_condiment(&self) {
        println!("Adding Lemon");
    }
}