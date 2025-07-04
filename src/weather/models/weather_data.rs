use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct WeatherData {
    pub condition: String,
    pub description: String,
    pub temp: f32,
    pub humidity: i32,
    pub icon: String,
}
