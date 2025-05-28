use clap::Parser;
use cli::{Args, CliAction};
use weather::WeatherDataOptions;

mod cli;
mod http;
mod utils;
mod weather;

pub fn run() {
    dotenvy::dotenv().unwrap();
    let args = Args::parse();

    match args.action {
        CliAction::Weather { city, lang, units } => {
            let options = WeatherDataOptions {
                lang: Some(lang),
                units: Some(units),
            };
            cli::get_weather_data(&city, options);
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
