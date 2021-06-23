pub trait Turkey {
    fn gobble(&self);
    fn fly(&self);
}

pub struct WildTurkey;
impl Turkey for WildTurkey {
    fn gobble(&self) {
        println!("Gobble gobble");
    }

    fn fly(&self) {
        println!("I'm flying a short distance");
    }
}