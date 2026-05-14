use crate::config::Config;
use reqwest::blocking::Client;
use serde::Deserialize;

pub struct Api<'a> {
    pub data: Vec<ApiData>,
    base_url: String,
    queries: String,
    config: &'a Config,
    client: Client,
}

impl<'a> Api<'a> {
    pub fn new(config: &'a Config) -> Self {
        let function_info = "&resampleFreq=".to_string() + &config.function;
        let license_info = "&token=".to_string() + &config.license;
        let start_date = "&startDate=".to_string() + &config.plot_start;

        let b = "https://api.tiingo.com/tiingo/daily/".to_string();
        let q = "/prices?sort=date".to_string() + &function_info + &license_info + &start_date;

        Api {
            data: Vec::new(),
            base_url: b,
            queries: q,
            config,
            client: Client::new(),
        }
    }

    pub fn fetch_stock(&mut self, stock: &str) -> &Self {
        let stock_url = self.base_url.clone() + stock + self.queries.as_str();

        let response = self
            .client
            .get(&stock_url)
            .send()
            .unwrap_or_else(|_| panic!("There was a problem fetching data for {}", stock));

        let deserialized_response: ApiResponse = response
            .json()
            .expect("There was a problem in deserialization");

        match deserialized_response {
            ApiResponse::Success(data) => {
                self.data.push(data);
            }
            ApiResponse::Failure(information) => {
                println!("{}", information);
            }
        }

        self
    }

    pub fn fetch(&mut self) -> &Self {
        for stock in &self.config.stocks {
            self.fetch_stock(stock);
        }

        self
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataBlock {
    pub date: String,
    pub adj_close: f32,
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct ApiData {
    pub historical_data: Vec<DataBlock>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ApiResponse {
    Success(ApiData),
    Failure(String),
}
