use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum CliAction {
    /// Get weather data
    #[arg()]
    GetWeatherData {
        /// Name of the city
        #[arg()]
        city: String,
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
