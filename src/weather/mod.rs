use std::env;

use crate::{http::HTTPError, utils::serde_utils};
use models::{WeatherData, WeatherDataResponse};

mod models;

pub fn get_weather_data(city_name: &str) -> Result<WeatherData, HTTPError> {
    let api_key = env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY not found");
    let units = "metric";
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units={units}",
        city_name, api_key
    );

    let data = match reqwest::blocking::get(url) {
        Ok(data) => data,
        Err(error) => {
            println!("{}", error.to_string());
            return Err(HTTPError::UnexpectedError);
        }
    };

    let text = data.text().expect("Unexpected error");
    println!("Response: {}", text);

    let response_data = serde_utils::parse_value_from_json::<WeatherDataResponse>(&text).unwrap();

    let weather_data = match response_data {
        WeatherDataResponse::Success(data) => WeatherData {
            condition: data.weather[0].main.clone(),
            description: data.weather[0].description.clone(),
            temp: data.main.temp,
            humidity: data.main.humidity,
        },
        WeatherDataResponse::Error(error) => {
            if error.cod == "404" {
                return Err(HTTPError::NotFound);
            } else {
                return Err(HTTPError::UnexpectedError);
            }
        }
    };

    Result::Ok(weather_data)
}
