use template::*;

fn main() {
    let tea = Tea;
    let coffee = Coffee;

    println!("\nMaking tea...");
    tea.prepare_recipe();

    println!("\nMaking coffee...");
    coffee.prepare_recipe();
}