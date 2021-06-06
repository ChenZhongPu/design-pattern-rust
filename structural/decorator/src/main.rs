use decorator::*;

fn main() {
    let beverage = Espresso;
    println!("Espresso: {}", beverage.cost());
    let mut mocha = Mocha::new(Box::new(beverage));
    mocha = Mocha::new(Box::new(mocha));
    println!("Espresso with 2 mocha: {}", mocha.cost());
}