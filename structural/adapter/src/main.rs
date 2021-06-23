use adapter::*;

fn test_duck<T: Duck>(duck: &T) {
    duck.quack();
    duck.fly();
}
fn main() {
    let duck = MallardDuck;

    let turkey = WildTurkey;
    let turkey_adapter = TurkeyAdapter::new(Box::new(turkey));

    println!("The Duck says...");
    test_duck(&duck);

    println!("\nThe Turkey says...");
    test_duck(&turkey_adapter);
}