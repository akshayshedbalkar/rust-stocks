pub mod api;
pub mod config;
pub mod plot;

pub use api::*;
pub use config::*;
pub use plot::*;

pub fn run() {
    let mut a: Api = Api::new();
    a.fetch();

    let p: Plot = Plot::new(&a);
    p.plot();
}
