pub mod api;
pub mod config;

pub use api::*;
pub use config::*;

pub fn run() {
    let c = Config::new();
    let mut a: Api = Api::new(&c);
    a.fetch();
    if !a.data.is_empty()
    {
        println!("{:#?}", a.data[0].historical_data.first().unwrap());
    }
}
