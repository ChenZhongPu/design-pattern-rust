use crate::Menu;

pub struct Waitress {
    menus: Vec<Box<dyn Menu>>,
}

impl Waitress {
    pub fn new(menus: Vec<Box<dyn Menu>>) -> Self {
       Waitress { menus }
    }

    pub fn print_menu(&self) {
       for menu in self.menus.iter() {
           for item in menu.iter() {
               println!("{:#?}", item);
           }
       }
    }
}