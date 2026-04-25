use crate::config::Config;
use chrono::NaiveDate;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::BTreeMap;

pub struct Api<'a> {
    pub data: Vec<ApiData>,
    base_url: String,
    config: &'a Config,
    client: Client,
}

impl<'a> Api<'a> {
    pub fn new(config: &'a Config) -> Self {
        let function_info = "function=".to_string() + &config.function;
        let license_info = "&apikey=".to_string() + &config.license;
        let construct_url =
            "https://www.alphavantage.co/query?".to_string() + &function_info + &license_info;

        Api {
            data: Vec::new(),
            base_url: construct_url,
            config,
            client: Client::new(),
        }
    }

    pub fn fetch_stock(&mut self, stock: &str) -> &Self {
        let stock_url = self.base_url.clone() + "&symbol=" + stock;

        let response = self
            .client
            .get(&stock_url)
            .send()
            .expect(&format!("There was a problem fetching data for {}", stock));

        self.data.push(response.json().expect(&format!(
            "There was a problem deserializing data for {} using url {}",
            stock, stock_url
        )));

        self
    }
    pub fn fetch(&mut self) -> &Self {
        for stock in self.config.stocks.clone() {
            self.fetch_stock(&stock);
        }
        
        self
    }
}

#[derive(Deserialize)]
struct MetaData {
    #[serde(rename = "1. Information")]
    info: String,
    #[serde(rename = "2. Symbol")]
    symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    refresh: String,
    #[serde(rename = "4. Time Zone")]
    zone: String,
}

impl MetaData {
    pub fn new() -> Self {
        MetaData {
            info: String::new(),
            symbol: String::new(),
            refresh: String::new(),
            zone: String::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct DataBlock {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. adjusted close")]
    adjusted: String,
    #[serde(rename = "6. volume")]
    volume: String,
    #[serde(rename = "7. dividend amount")]
    dividend: String,
}

#[derive(Deserialize)]
pub struct ApiData {
    #[serde(rename = "Meta Data")]
    meta_data: MetaData,
    #[serde(rename = "Weekly Adjusted Time Series")]
    pub historical_data: BTreeMap<NaiveDate, DataBlock>,
}

impl ApiData {
    pub fn new() -> Self {
        ApiData {
            meta_data: MetaData::new(),
            historical_data: BTreeMap::new(),
        }
    }
}
