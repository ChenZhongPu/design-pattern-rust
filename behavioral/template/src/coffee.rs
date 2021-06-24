use crate::CaffeineBeverage;

pub struct Coffee;

impl CaffeineBeverage for Coffee {
    fn brew(&self) {
        println!("Dripping Coffee through filter");
    }

    fn add_condiment(&self) {
       println!("Adding Sugar and Milk");
    }
}