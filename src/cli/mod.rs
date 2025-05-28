pub use args::{Args, CliAction};

use crate::{
    http::HTTPError,
    weather::{self, WeatherDataOptions},
};

mod args;

pub fn get_weather_data(city_name: &str, options: WeatherDataOptions) {
    let units = options.units.clone().unwrap_or_default();
    let units_symbol = units.get_symbol();
    let weather_data = weather::get_weather_data(&city_name, options);

    match weather_data {
        Ok(data) => {
            println!("Weather data for: {}\n", city_name);
            println!("Condition: {}\n", data.condition);
            println!("Description: {}\n", data.description);
            println!("Humidity: {}\n", data.humidity);
            println!(
                "Tempreature: {}\n",
                format!("{} {}", data.temp, units_symbol)
            );
        }
        Err(error) => match error {
            HTTPError::NotFound => {
                println!("City not found");
            }
            HTTPError::UnexpectedError => {
                println!("Unexpected error");
            }
        },
    }
}
