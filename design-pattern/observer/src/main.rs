use observer::WeatherData;
use observer::CurrentConditionDisplay;
use observer::PredictDisplay;
use observer::Subject;

fn main() {
    let mut weather_data = WeatherData::new();

    weather_data.register_observer(Box::new(CurrentConditionDisplay::new(1)));
    weather_data.register_observer(Box::new(CurrentConditionDisplay::new(2)));

    weather_data.register_observer(Box::new(PredictDisplay::new(44)));

    weather_data.set_measusrements(32.0, 56.0, 180.1);
}