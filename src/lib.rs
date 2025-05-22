use clap::Parser;
use cli::{Args, CliAction};
use std::env;

mod cli;

pub fn run() {
    dotenvy::dotenv().unwrap();
    let api_key = env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY not found");

    println!("WEATHER_API_KEY: {}", api_key);

    let args = Args::parse();

    match args.action {
        CliAction::GetWeatherData { city } => {
            println!("City is: {}", city);
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
