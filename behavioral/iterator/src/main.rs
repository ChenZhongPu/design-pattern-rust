use iterator::{DinerMenu, PancakeHouseMenu, Waitress};

fn main() {
    let mut diner_menu = DinerMenu::new();
    diner_menu.add_item("Hotdog",
                        "A hot dog, with sauerkraut, relish, onions, topped with cheese",
                        false, 3.05);

    let mut pancake_menu = PancakeHouseMenu::new();
    pancake_menu.add_item("Blueberry Pancakes",
                          "Pancakes made with fresh blueberries and blueberry syrup",
                          true, 3.49);

    let waitress = Waitress::new(vec![
        Box::new(diner_menu),
        Box::new(pancake_menu)]);

    waitress.print_menu();
}