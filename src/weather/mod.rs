use clap::ValueEnum;
use std::{env, fmt::Display, str::FromStr};

use crate::{http::HTTPError, utils::serde_utils};
use models::{WeatherData, WeatherDataResponse};

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
    let api_key = env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY not found");
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
