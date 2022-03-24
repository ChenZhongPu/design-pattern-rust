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
