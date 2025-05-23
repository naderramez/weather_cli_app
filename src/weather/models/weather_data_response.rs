use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f32,
    pub humidity: i32,
}

#[derive(Debug, Deserialize)]
pub struct WeatherDataResponse {
    pub main: Main,
}
