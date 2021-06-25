pub trait MenuComponent {
    fn add(&mut self, _component: Box<dyn MenuComponent>) {
       panic!("Unsupported operation");
    }

    fn get_child(&self, _i: usize) -> &Box<dyn MenuComponent> {
        panic!("Unsupported operation");
    }

    fn name(&self) -> &str {
        panic!("Unsupported operation");
    }

    fn description(&self) -> &str {
        panic!("Unsupported operation");
    }

    fn price(&self) -> f64 {
        panic!("Unsupported operation");
    }

    fn is_vegetarian(&self) -> bool {
        panic!("Unsupported operation");
    }

    fn print(&self) {
        panic!("Unsupported operation");
    }
}