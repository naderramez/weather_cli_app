use clap::Parser;
use cli::{Args, CliAction};

mod cli;
mod http;
mod utils;
mod weather;

pub fn run() {
    dotenvy::dotenv().unwrap();
    let args = Args::parse();

    match args.action {
        CliAction::GetWeatherData { city } => {
            cli::get_weather_data(&city);
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
