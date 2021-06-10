use observer::CurrentConditionDisplay;
use observer::PredictDisplay;
use observer::WeatherData;

fn main() {
    let mut weather_data = WeatherData::new();
    let observer_1 = CurrentConditionDisplay::new(1);
    let observer_2 = CurrentConditionDisplay::new(2);
    let observer_3 = PredictDisplay::new(3);

    println!("-----add three observers-----");
    weather_data.add_observer(&observer_1);
    weather_data.add_observer(&observer_2);
    weather_data.add_observer(&observer_3);
    weather_data.set_measurement(10.0);

    println!("-----remove the third observer----");
    weather_data.remove_observer(&observer_1);
    weather_data.set_measurement(11.0);
}
