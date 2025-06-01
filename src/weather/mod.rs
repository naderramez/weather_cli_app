use clap::ValueEnum;
use std::{collections::HashMap, env, fmt::Display, str::FromStr};

use crate::{
    cache::{load_from_cache, save_to_cache},
    env::get_api_key,
    http::HTTPError,
    utils::serde_utils::{self, serialize_data},
};
use models::{CachedWeatherData, WeatherData, WeatherDataResponse};

mod models;

#[derive(Debug, Clone, Default, PartialEq, Eq, ValueEnum)]
pub enum TempreatureUnit {
    #[default]
    Metric,
    Imperial,
}

impl FromStr for TempreatureUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "metric" => Ok(Self::Metric),
            "imperial" => Ok(Self::Imperial),
            _ => Err(()),
        }
    }
}

impl Display for TempreatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unit_str = match self {
            Self::Metric => "metric",
            Self::Imperial => "imperial",
        };

        write!(f, "{}", unit_str)
    }
}

impl TempreatureUnit {
    pub fn get_symbol(&self) -> &str {
        match self {
            Self::Metric => "°C",
            Self::Imperial => "°F",
        }
    }
}

#[derive(Debug, Default)]
pub struct WeatherDataOptions {
    pub lang: Option<String>,
    pub units: Option<TempreatureUnit>,
}

pub fn get_weather_data(
    city_name: &str,
    options: WeatherDataOptions,
) -> Result<WeatherData, HTTPError> {
    let api_key = get_api_key();
    let lang = options.lang.unwrap_or("en".to_string());
    let units = options.units.unwrap_or(TempreatureUnit::Metric);
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&lang={}&units={units}",
        city_name, api_key, lang
    );

    let data = match reqwest::blocking::get(url) {
        Ok(data) => data,
        Err(error) => {
            println!("{}", error.to_string());
            let cached_data = load_from_cache().map_err(|_| HTTPError::UnexpectedError)?;
            if cached_data.is_none() {
                return Err(HTTPError::UnexpectedError);
            }
            let parsed_cached_data =
                serde_utils::parse_value_from_json::<CachedWeatherData>(&cached_data.unwrap())
                    .map_err(|_| HTTPError::UnexpectedError)?;
            if let Some(weather_data) = parsed_cached_data.get(city_name) {
                return Ok(weather_data.clone());
            } else {
                return Err(HTTPError::UnexpectedError);
            }
        }
    };

    let text = data.text().expect("Unexpected error");
    println!("Response: {}", text);

    let response_data = serde_utils::parse_value_from_json::<WeatherDataResponse>(&text).unwrap();

    let weather_data = match response_data {
        WeatherDataResponse::Success(data) => {
            let weather_data = WeatherData {
                condition: data.weather[0].main.clone(),
                description: data.weather[0].description.clone(),
                temp: data.main.temp,
                humidity: data.main.humidity,
                icon: data.weather[0].icon.clone(),
            };
            let cached_data = load_from_cache().map_err(|_| HTTPError::UnexpectedError)?;
            let mut parsed_cached_data = {
                if cached_data.is_none() {
                    HashMap::new()
                } else {
                    let data = serde_utils::parse_value_from_json::<CachedWeatherData>(
                        &cached_data.unwrap(),
                    )
                    .map_err(|_| HTTPError::UnexpectedError)?;
                    data
                }
            };

            parsed_cached_data.insert(city_name.to_string(), weather_data.clone());

            let serialized_data =
                serialize_data(&parsed_cached_data).map_err(|_| HTTPError::UnexpectedError)?;

            save_to_cache(&serialized_data).map_err(|_| HTTPError::UnexpectedError)?;
            weather_data
        }
        WeatherDataResponse::Error(error) => {
            println!("Error block");
            if error.cod == "404" {
                return Err(HTTPError::NotFound);
            } else {
                return Err(HTTPError::UnexpectedError);
            }
        }
    };

    Result::Ok(weather_data)
}

pub fn get_weather_emoji(icon_code: &str) -> &str {
    match icon_code {
        "01d" => "☀️",         // Clear sky (day)
        "01n" => "🌙",         // Clear sky (night)
        "02d" | "02n" => "⛅", // Few clouds
        "03d" | "03n" => "☁️", // Scattered clouds
        "04d" | "04n" => "🌥️", // Broken clouds
        "09d" | "09n" => "🌧️", // Shower rain
        "10d" => "🌦️",         // Rain (day)
        "10n" => "🌧️",         // Rain (night)
        "11d" | "11n" => "⛈️", // Thunderstorm
        "13d" | "13n" => "❄️", // Snow
        "50d" | "50n" => "🌫️", // Mist
        _ => "❓",             // Unknown
    }
}
