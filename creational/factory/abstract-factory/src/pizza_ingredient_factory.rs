use crate::Sauce;
use crate::{
    Cheese, Clams, Dough, Eggplant, FreshClams, FrozenClams, Garlic, MarinaraSauce,
    MozzarellaCheese, MushRoom, Onion, Pepperoni, PlumTomatoSauce, RedPepper, ReggianoCheese,
    SlicedPepperoni, ThickCrustDough, ThinCrustDough, Veggies,
};

pub trait PizzaIngredientFactory {
    fn create_dough(&self) -> Box<dyn Dough>;
    fn create_sauce(&self) -> Box<dyn Sauce>;
    fn create_cheese(&self) -> Box<dyn Cheese>;
    fn create_veggies(&self) -> Vec<Box<dyn Veggies>>;
    fn create_pepperoni(&self) -> Box<dyn Pepperoni>;
    fn create_clam(&self) -> Box<dyn Clams>;
}

pub struct NYPizzaIngredientFactory;
impl PizzaIngredientFactory for NYPizzaIngredientFactory {
    fn create_dough(&self) -> Box<dyn Dough> {
        Box::new(ThinCrustDough)
    }

    fn create_sauce(&self) -> Box<dyn Sauce> {
        Box::new(MarinaraSauce)
    }

    fn create_cheese(&self) -> Box<dyn Cheese> {
        Box::new(ReggianoCheese)
    }

    fn create_veggies(&self) -> Vec<Box<dyn Veggies>> {
        vec![
            Box::new(Garlic),
            Box::new(Onion),
            Box::new(MushRoom),
            Box::new(RedPepper),
        ]
    }

    fn create_pepperoni(&self) -> Box<dyn Pepperoni> {
        Box::new(SlicedPepperoni)
    }

    fn create_clam(&self) -> Box<dyn Clams> {
        Box::new(FreshClams)
    }
}

pub struct ChicagoPizzaIngredientFactory;
impl PizzaIngredientFactory for ChicagoPizzaIngredientFactory {
    fn create_dough(&self) -> Box<dyn Dough> {
        Box::new(ThickCrustDough)
    }

    fn create_sauce(&self) -> Box<dyn Sauce> {
        Box::new(PlumTomatoSauce)
    }

    fn create_cheese(&self) -> Box<dyn Cheese> {
        Box::new(MozzarellaCheese)
    }

    fn create_veggies(&self) -> Vec<Box<dyn Veggies>> {
        vec![Box::new(Eggplant)]
    }

    fn create_pepperoni(&self) -> Box<dyn Pepperoni> {
        Box::new(SlicedPepperoni)
    }

    fn create_clam(&self) -> Box<dyn Clams> {
        Box::new(FrozenClams)
    }
}
