use composite::{Menu, MenuComponent, MenuItem, Waitress};

fn main() {
    let mut pancake_menu = Menu::new("PANCAKE HOUSE MENU", "Breakfast");
    let mut diner_menu = Menu::new("DINER MENU", "Lunch");
    let mut dessert_menu = Menu::new("DESSERT MENU", "Dessert of course!");

    let mut all_menus = Menu::new("ALL MENUS", "ALl menus combined");

    pancake_menu.add(Box::new(MenuItem::new("K&B's Pancake Breakfast",
                                            "Pancakes with scrambled eggs and toast",
                                            true,
                                            2.99)));
    pancake_menu.add(Box::new(MenuItem::new("Waffles",
                                                        "Waffles with your choice of blueberries or strawberries",
                                                        true,
                                                        3.59)));

    diner_menu.add(Box::new(MenuItem::new("Vegetarian BLT",
                                          "(Fakin') Bacon with lettuce & tomato on whole wheat",
                                          true,
                                          2.99)));
    diner_menu.add(Box::new(MenuItem::new("Hot Dog",
                                          "A hot dog, with saurkraut, relish, onions, topped with cheese",
                                          false,
                                          3.05)));

    dessert_menu.add(Box::new(MenuItem::new("Apple Pie",
                                            "Apple pie with a flakey crust, topped with vanilla icecream",
                                            true,
                                            1.59)));
    dessert_menu.add(Box::new(MenuItem::new("Cheesecake",
                                            "Creamy New York cheesecake, with a chocolate graham crust",
                                            true,
                                            1.99)));
    // dessert menu is a sub-menu
    diner_menu.add(Box::new(dessert_menu));

    all_menus.add(Box::new(pancake_menu));
    all_menus.add(Box::new(diner_menu));

    let waitress = Waitress::new(Box::new(all_menus));
    waitress.print_menu();
}