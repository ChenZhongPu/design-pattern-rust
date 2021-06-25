use crate::MenuComponent;

pub struct Menu {
    components: Vec<Box<dyn MenuComponent>>,
    name: String,
    description: String,
}

impl Menu {
    pub fn new(name: &str, description: &str) -> Self {
        Menu {
            components: vec![],
            name: String::from(name),
            description: String::from(description),
        }
    }
}

impl MenuComponent for Menu {
    fn add(&mut self, component: Box<dyn MenuComponent>) {
        self.components.push(component);
    }

    fn get_child(&self, i: usize) -> &Box<dyn MenuComponent> {
        &self.components[i]
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn print(&self) {
        println!("\n{}", self.name);
        println!(", {}", self.description);
        println!("-----------------------");

        for item in self.components.iter() {
            item.print();
        }
    }
}