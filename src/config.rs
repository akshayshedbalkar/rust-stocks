use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub license: String,
    pub function: String,
    pub columns: String,
    pub plot_start: String,
    pub stocks: Vec<String>,
}

impl Config {
    const CONFIG: &str = include_str!("Stocks.toml");

    pub fn new() -> Self {
        if !std::fs::exists("Stocks.toml").unwrap() {
            let _ = std::fs::write("Stocks.toml", Config::CONFIG);
        }

        let file_contents = std::fs::read_to_string("Stocks.toml").unwrap();
        toml::from_str(&file_contents).expect("Unable to parse configuration file")
    }
}
