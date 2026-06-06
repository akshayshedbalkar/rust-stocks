use crate::config::Config;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

pub struct Api {
    pub data: Vec<ApiData>,
    base_url: String,
    pub config: Config,
    client: Client,
    queries: Queries,
}

impl Api {
    pub fn new() -> Self {
        let c = Config::new();
        let q = Queries::new(&c.function, &c.license, &c.plot_start, "date", &c.columns);
        let b = "https://api.tiingo.com/tiingo/daily/".to_string();

        Api {
            data: Vec::new(),
            base_url: b,
            config: c,
            client: Client::new(),
            queries: q,
        }
    }

    fn fetch_stock(&self, stock: &str) -> Option<ApiData> {
        let stock_url = self.base_url.clone() + stock + "/prices";

        let response = self
            .client
            .get(&stock_url)
            .query(&self.queries)
            .send()
            .unwrap_or_else(|_| panic!("There was a problem fetching data for {}", stock));

        let deserialized_response: ApiResponse = response
            .json()
            .expect("There was a problem in deserialization");

        match deserialized_response {
            ApiResponse::Success(data) => Some(data),
            ApiResponse::Failure(information) => {
                println!("{}", information);
                None
            }
        }
    }

    pub fn fetch(&mut self) -> &Self {
        for stock in &self.config.stocks {
            if let Some(s) = self.fetch_stock(stock) {
                self.data.push(s);
            }
        }

        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Queries {
    resample_freq: String,
    token: String,
    start_date: String,
    sort: String,
    columns: String,
}
impl Queries {
    pub fn new(
        resample_freq: &str,
        token: &str,
        start_date: &str,
        sort: &str,
        columns: &str,
    ) -> Self {
        Queries {
            resample_freq: resample_freq.to_string(),
            token: token.to_string(),
            start_date: start_date.to_string(),
            sort: sort.to_string(),
            columns: columns.to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataBlock {
    pub date: DateTime<Utc>,
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
