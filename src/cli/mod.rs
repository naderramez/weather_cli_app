pub use args::{Args, CliAction};

use crate::weather;

mod args;

pub fn get_weather_data(city_name: &str) {
    let weather_data = weather::get_weather_data(&city_name);

    match weather_data {
        Ok(data) => {
            println!("Condition: {}\n", data.condition);
            println!("Description: {}\n", data.description);
            println!("Humidity: {}\n", data.humidity);
            println!("Tempreature: {}\n", data.temp);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}
