use crate::pizza_ingredient_factory::PizzaIngredientFactory;
use crate::{Cheese, Clams, Dough, Sauce, Veggies};
use std::fmt;

pub trait Pizza: fmt::Display {
    fn prepare(&mut self);
    fn bake(&self) {
        println!("Bake for 25 minutes at 350");
    }
    fn cut(&self) {
        println!("Cutting the pizza into diagonal slices");
    }
    fn boxes(&self) {
        println!("Place pizza in official PizzaStore box");
    }
    fn name(&self) -> &str;
}

pub struct CheesePizza {
    name: String,
    dough: Option<Box<dyn Dough>>,
    sauce: Option<Box<dyn Sauce>>,
    cheese: Option<Box<dyn Cheese>>,
    factory: Box<dyn PizzaIngredientFactory>,
}

impl CheesePizza {
    pub fn new(factory: Box<dyn PizzaIngredientFactory>, name: &str) -> Self {
        CheesePizza {
            name: String::from(name),
            factory,
            dough: None,
            sauce: None,
            cheese: None,
        }
    }
}

impl Pizza for CheesePizza {
    fn prepare(&mut self) {
        println!("Preparing {}", self.name);
        self.dough = Some(self.factory.create_dough());
        self.sauce = Some(self.factory.create_sauce());
        self.cheese = Some(self.factory.create_cheese());
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for CheesePizza {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "name: {} \n dough: {} \n sauce: {} \n cheese: {}",
            self.name,
            self.dough.as_ref().unwrap(),
            self.sauce.as_ref().unwrap(),
            self.cheese.as_ref().unwrap()
        )
    }
}

pub struct ClamPizza {
    name: String,
    dough: Option<Box<dyn Dough>>,
    sauce: Option<Box<dyn Sauce>>,
    cheese: Option<Box<dyn Cheese>>,
    clam: Option<Box<dyn Clams>>,
    factory: Box<dyn PizzaIngredientFactory>,
}
impl ClamPizza {
    pub fn new(factory: Box<dyn PizzaIngredientFactory>, name: &str) -> Self {
        ClamPizza {
            name: String::from(name),
            factory,
            dough: None,
            sauce: None,
            cheese: None,
            clam: None,
        }
    }
}
impl Pizza for ClamPizza {
    fn prepare(&mut self) {
        println!("Preparing {}", self.name);
        self.dough = Some(self.factory.create_dough());
        self.sauce = Some(self.factory.create_sauce());
        self.cheese = Some(self.factory.create_cheese());
        self.clam = Some(self.factory.create_clam());
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for ClamPizza {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "name: {} \n dough: {} \n sauce: {} \n cheese: {} \n clam: {}",
            self.name,
            self.dough.as_ref().unwrap(),
            self.sauce.as_ref().unwrap(),
            self.cheese.as_ref().unwrap(),
            self.clam.as_ref().unwrap()
        )
    }
}

pub struct VeggiePizza {
    name: String,
    dough: Option<Box<dyn Dough>>,
    sauce: Option<Box<dyn Sauce>>,
    cheese: Option<Box<dyn Cheese>>,
    veggies: Option<Vec<Box<dyn Veggies>>>,
    factory: Box<dyn PizzaIngredientFactory>,
}
impl VeggiePizza {
    pub fn new(factory: Box<dyn PizzaIngredientFactory>, name: &str) -> Self {
        VeggiePizza {
            name: String::from(name),
            factory,
            dough: None,
            sauce: None,
            cheese: None,
            veggies: None,
        }
    }
}
impl Pizza for VeggiePizza {
    fn prepare(&mut self) {
        println!("Preparing {}", self.name);
        self.dough = Some(self.factory.create_dough());
        self.sauce = Some(self.factory.create_sauce());
        self.cheese = Some(self.factory.create_cheese());
        self.veggies = Some(self.factory.create_veggies());
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for VeggiePizza {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "name: {} \n dough: {} \n sauce: {} \n cheese: {} \n veggies num: {}",
            self.name,
            self.dough.as_ref().unwrap(),
            self.sauce.as_ref().unwrap(),
            self.cheese.as_ref().unwrap(),
            self.veggies.as_ref().unwrap().len()
        )
    }
}
