use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub license: String,
    pub function: String,
    pub column: String,
    pub plot_start: String,
    pub stocks: Vec<String>,
}

impl Config {
    const CONFIG: &str = include_str!("Stocks.toml");

    pub fn new() -> Self {
        toml::from_str(Config::CONFIG).expect("Unable to parse configuration file")
    }
}
