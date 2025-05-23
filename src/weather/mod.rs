use std::env;

use crate::utils::serde_utils;
use models::{WeatherData, WeatherDataResponse};

mod models;

pub fn get_weather_data(city_name: &str) -> Result<WeatherData, reqwest::Error> {
    let api_key = env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY not found");
    let units = "metric";
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units={units}",
        city_name, api_key
    );
    let data = reqwest::blocking::get(url)?;
    let text = data.text()?;
    println!("Response: {}", text);

    let response_data = serde_utils::parse_value_from_json::<WeatherDataResponse>(&text).unwrap();

    let weather_data = WeatherData {
        condition: String::from("Clear"),
        description: String::from("Clear"),
        temp: response_data.main.temp,
        humidity: response_data.main.humidity,
    };

    Result::Ok(weather_data)
}
