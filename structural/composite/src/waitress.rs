use crate::MenuComponent;

pub struct Waitress {
    menus: Box<dyn MenuComponent>,
}

impl Waitress {
    pub fn new(menus: Box<dyn MenuComponent>) -> Self {
       Waitress { menus }
    }

    pub fn print_menu(&self) {
        self.menus.print();
    }
}