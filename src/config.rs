use serde::Deserialize;
use dotenvy::dotenv;
use std::env;

#[derive(Deserialize)]
pub struct Config {
    #[serde(skip_deserializing)]
    pub license: String,
    pub function: String,
    pub columns: String,
    pub plot_start: String,
    pub stocks: Vec<String>,
    pub y_max: f32
}

impl Config {
    const CONFIG: &str = include_str!("Stocks.toml");

    pub fn new() -> Self {
        if !std::fs::exists("Stocks.toml").expect("Something has gone horribly wrong.") {
            let _ = std::fs::write("Stocks.toml", Config::CONFIG);
        }

        let file_contents = std::fs::read_to_string("Stocks.toml").expect("Error reading Stocks.toml");
        let mut c: Config = toml::from_str(&file_contents).expect("Unable to parse configuration file");

        dotenv().ok();
        c.license = env::var("license").expect("Please provide an API key in .env file");
        c
        
    }
}
