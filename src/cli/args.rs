use clap::{Parser, Subcommand};

use crate::weather::TempreatureUnit;

#[derive(Debug, Clone, Subcommand)]
pub enum CliAction {
    /// Get weather data
    #[arg()]
    Weather {
        /// Name of the city
        #[arg()]
        city: String,
        /// Language (en)
        #[arg(short, long, default_value_t = String::from("en"))]
        lang: String,
        /// Units (metric or imperial)
        #[arg(short, long, default_value_t = TempreatureUnit::Metric)]
        units: TempreatureUnit,
    },
}

/// Simple program to fetch weather data
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Action to take
    #[command(subcommand)]
    pub action: CliAction,
    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
}
