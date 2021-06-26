use state::*;

fn main() {
    let mut machine = GumballMachine::new(2);

    machine.insert_quarter();
    machine.turn_crank();
    println!("----------");
    machine.insert_quarter();
    machine.turn_crank();
    println!("----------");
    machine.insert_quarter();
    machine.turn_crank();
}