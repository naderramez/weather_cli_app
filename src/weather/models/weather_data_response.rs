use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f32,
    pub humidity: i32,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct WeatherSuccessResponse {
    pub main: Main,
    pub weather: [Weather; 1],
}

#[derive(Debug, Deserialize)]
pub struct WeatherErrorResponse {
    pub cod: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum WeatherDataResponse {
    Success(WeatherSuccessResponse),
    Error(WeatherErrorResponse),
}
