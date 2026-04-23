use reqwest::blocking as Rest;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
struct MetaData{
    #[serde(rename = "1. Information")]
    info: String,
    #[serde(rename = "2. Symbol")]
    symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    refresh: String,
    #[serde(rename = "4. Time Zone")]
    zone: String,
}

#[derive(Deserialize,Debug)]
struct Data{
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
struct WeeklyData{
    #[serde(rename = "Meta Data")]
    meta_data: MetaData,
    #[serde(rename = "Weekly Adjusted Time Series")]
    data: BTreeMap<String,Data>,
}

fn main() {
    let response = Rest::get("https://www.alphavantage.co/query?function=TIME_SERIES_WEEKLY_ADJUSTED&symbol=IBM&apikey=demo").unwrap();
    let mut weekly_data: WeeklyData = response.json().unwrap();
    println!("{:#?}", weekly_data.data.first_entry().unwrap());

}

