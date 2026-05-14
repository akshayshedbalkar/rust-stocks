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
        let query = vec!(("symbol", stock));

        let response = self
            .client
            .get(&self.base_url)
            .query(&query)
            .send()
            .unwrap_or_else(|_| panic!("There was a problem fetching data for {}", stock));

        let deserialized_response: ApiResponse = response
            .json()
            .expect("There was a problem in deserialization");

        match deserialized_response {
            ApiResponse::Success(data) => {
                self.data.push(data);
            }
            ApiResponse::Failure { information } => {
                println!("{}", information);
            }
        }

        self
    }

    pub fn fetch(&mut self) -> &Self {
        for stock in &self.config.stocks {
            self.fetch_stock(stock);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        self
    }
}

#[derive(Deserialize, Debug)]
pub struct MetaData {
    #[serde(rename = "1. Information")]
    pub info: String,
    #[serde(rename = "2. Symbol")]
    pub symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    pub refresh: String,
    #[serde(rename = "4. Time Zone")]
    pub zone: String,
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
    pub open: String,
    #[serde(rename = "2. high")]
    pub high: String,
    #[serde(rename = "3. low")]
    pub low: String,
    #[serde(rename = "4. close")]
    pub close: String,
    #[serde(rename = "5. adjusted close")]
    pub adjusted: String,
    #[serde(rename = "6. volume")]
    pub volume: String,
    #[serde(rename = "7. dividend amount")]
    pub dividend: String,
}

#[derive(Deserialize, Debug)]
pub struct ApiData {
    #[serde(rename = "Meta Data")]
    pub meta_data: MetaData,
    #[serde(alias = "Weekly Adjusted Time Series")]
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

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ApiResponse {
    Success(ApiData),
    Failure {
        #[serde(rename = "Information")]
        information: String,
    },
}
