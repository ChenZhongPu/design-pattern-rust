use observer::CurrentConditionDisplay;
use observer::Observer;
use observer::PredictDisplay;
use observer::WeatherData;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut weather_data = WeatherData::new();
    let observer_1 = Rc::new(RefCell::new(CurrentConditionDisplay::new(1)));
    let observer_2 = Rc::new(RefCell::new(CurrentConditionDisplay::new(2)));
    let observer_3 = Rc::new(RefCell::new(PredictDisplay::new(3)));

    // println!("-----add three observers-----");
    let tmp = Rc::clone(&observer_1);
    let observer_1_clone1: Rc<RefCell<dyn Observer>> = tmp;
    weather_data.add_observer(observer_1_clone1);

    let tmp = Rc::clone(&observer_2);
    let observer_2_clone1: Rc<RefCell<dyn Observer>> = tmp;
    weather_data.add_observer(observer_2_clone1);

    let tmp = Rc::clone(&observer_3);
    let observer_3_clone1: Rc<RefCell<dyn Observer>> = tmp;
    weather_data.add_observer(observer_3_clone1);

    weather_data.set_measurement(10.0);
    println!("------------");
    weather_data.set_measurement(12.0);

    println!("-----remove the third observer-----");
    let tmp = Rc::clone(&observer_1);
    let observer_1_clone2: Rc<RefCell<dyn Observer>> = tmp;
    weather_data.remove_observer(observer_1_clone2);
    weather_data.set_measurement(14.0);
}
