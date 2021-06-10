pub trait Pizza {
    fn get_name(&self) -> &str;

    fn prepare(&self) {
        println!("Preparing {}", self.get_name());
    }

    fn bake(&self) {
        println!("Baking {}", self.get_name());
    }

    fn cut(&self) {
        println!("Cutting {}", self.get_name());
    }

    fn boxes(&self) {
        println!("Boxing {}", self.get_name());
    }
}
