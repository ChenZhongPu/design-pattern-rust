use proxy::{Car, ProxyCar, ICar};

fn main() {
    let car = Car;
    let proxy_car = ProxyCar::new(18, &car);
    proxy_car.drive();
}