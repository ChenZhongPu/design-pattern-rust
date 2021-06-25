use crate::{MenuItem, Menu};

pub struct PancakeHouseMenu {
    menu_items: Vec<MenuItem>,
}

impl PancakeHouseMenu {
    pub fn new() -> Self {
       PancakeHouseMenu {
           menu_items: vec![
               MenuItem::new("K&B's Pancake Breakfast",
                             "Pancakes with scrambled eggs and toast",
                             true, 2.99),
               MenuItem::new("Regular Pancake Breakfast",
                             "Pancakes with fried eggs, sausage",
                             false, 2.99),
               MenuItem::new("Waffles",
                             "Waffles with your choice of blueberries or strawberries",
                             true, 3.59)
           ],
       }
    }

    pub fn add_item(&mut self, name: &str, description: &str,
                    vegetarian: bool, price: f64) {
        self.menu_items.push(MenuItem::new(name, description, vegetarian, price));
    }
}

impl Menu for PancakeHouseMenu {
    fn iter(&self) -> std::slice::Iter<'_, MenuItem> {
        self.menu_items.iter()
    }
}