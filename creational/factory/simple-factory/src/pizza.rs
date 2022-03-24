pub trait Pizza {
    fn name(&self) -> &str;

    fn prepare(&self) {
        println!("Preparing {}", self.name());
    }

    fn bake(&self) {
        println!("Baking {}", self.name());
    }

    fn cut(&self) {
        println!("Cutting {}", self.name());
    }

    fn boxes(&self) {
        println!("Boxing {}", self.name());
    }
}

pub struct CheesePizza {
    name: String,
}

impl CheesePizza {
    pub fn new() -> Self {
        CheesePizza {
            name: String::from("Cheese Pizza"),
        }
    }
}

impl Pizza for CheesePizza {
    fn name(&self) -> &str {
        &self.name
    }
}

pub struct ClamPizza {
    name: String,
}

impl ClamPizza {
    pub fn new() -> Self {
        ClamPizza {
            name: String::from("Clam Pizza"),
        }
    }
}

impl Pizza for ClamPizza {
    fn name(&self) -> &str {
        &self.name
    }
}

pub struct VeggiePizza {
    name: String,
}

impl VeggiePizza {
    pub fn new() -> Self {
        VeggiePizza {
            name: String::from("Veggie Pizza"),
        }
    }
}

impl Pizza for VeggiePizza {
    fn name(&self) -> &str {
        &self.name
    }
}
