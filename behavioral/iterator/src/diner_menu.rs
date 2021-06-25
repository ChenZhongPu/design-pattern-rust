use crate::{MenuItem, Menu};

const MAX_ITEMS: usize = 6;

pub struct DinerMenu {
    number: usize,
    menu_items: [MenuItem; MAX_ITEMS],
}

impl DinerMenu {
    pub fn new() -> Self {
        DinerMenu {
            number: 3,
            menu_items: [MenuItem::new("Vegetarian BLT",
                                       "(Fakin') Bacon with lettuce & tomato on whole wheat", true, 2.99),
                MenuItem::new("BLT",
                              "Bacon with lettuce & tomato on whole wheat", false, 2.99),
                MenuItem::new("Soup of the day",
                "Soup of the day, with a side of potato salad", false, 3.29),
                MenuItem::empty(), MenuItem::empty(), MenuItem::empty()],
        }
    }

    pub fn add_item(&mut self, name: &str, description: &str, vegetarian: bool, price: f64) {
        if self.number >= MAX_ITEMS {
            panic!("Sorry, menu is full!  Can't add item to menu");
        } else {
            self.menu_items[self.number] = MenuItem::new(name, description, vegetarian, price);
            self.number += 1;
        }
    }
}

impl Menu for DinerMenu {
    fn iter(&self) -> std::slice::Iter<'_, MenuItem> {
        self.menu_items[..self.number].iter()
    }
}