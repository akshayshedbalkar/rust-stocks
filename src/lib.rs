pub mod api;
pub mod config;

use api::*;
use config::*;

pub fn run() {
    let c = Config::new();
    let mut a: Api = Api::new(&c);
    a.fetch();
    println!("{:#?}", a.data[0].meta_data);
    println!("{:#?}", a.data[0].historical_data.last_entry().unwrap());
}
