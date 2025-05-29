use std::{env, sync::OnceLock};

static API_KEY: OnceLock<String> = OnceLock::new();

pub fn get_api_key() -> &'static str {
    API_KEY.get_or_init(|| env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY not found"))
}
