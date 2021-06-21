pub enum Singleton {
    INSTANCE,
}

impl Singleton {
    pub fn get_description(&self) -> String {
        String::from("I'm a thread safe Singleton!")
    }
}