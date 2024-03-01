use observer_3::subject::WeatherData;
use std::rc::Rc;

use observer_3::observer::{CurrentConditionDisplay, Observer, PredictDisplay};

fn main() {
    let mut weather_data = WeatherData::new();

    let observer_1: Rc<dyn Observer> = Rc::new(CurrentConditionDisplay);

    let observer_2: Rc<dyn Observer> = Rc::new(CurrentConditionDisplay);
    let observer_3: Rc<dyn Observer> = Rc::new(PredictDisplay);

    println!("-----add three observers-----");
    weather_data.add_observer(&observer_1);
    weather_data.add_observer(&observer_2);
    weather_data.add_observer(&observer_3);

    weather_data.set_measurement(10.0);

    println!("-----remove the third observer----");
    weather_data.remove_observer(&observer_1);
    weather_data.set_measurement(11.0);
}
