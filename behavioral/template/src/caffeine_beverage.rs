pub trait CaffeineBeverage {
    fn prepare_recipe(&self) {
        self.boil_water();
        self.brew();
        self.pour_in_cup();
        self.add_condiment();
    }
    fn brew(&self);
    fn add_condiment(&self);
    fn boil_water(&self) {
        println!("Boiling water");
    }
    fn pour_in_cup(&self) {
        println!("Pouring into cup");
    }
}