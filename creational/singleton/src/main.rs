use singleton::Singleton;

fn main() {
    let instance = Singleton::INSTANCE;
    println!("{}", instance.get_description());
}
