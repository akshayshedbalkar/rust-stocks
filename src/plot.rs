use crate::Api;
use chrono::{NaiveDate, TimeZone, Utc};
use plotters::prelude::*;

pub struct Plot<'a> {
    api_ref: &'a Api,
}

impl<'a> Plot<'a> {
    pub fn new(api_ref: &'a Api) -> Plot<'a> {
        Plot { api_ref }
    }

    pub fn plot(&self) {
        let naive_date =
            NaiveDate::parse_from_str(&self.api_ref.config.plot_start, "%Y-%m-%d").expect("Error getting date from Stocks.toml");
        let start_date = Utc.from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).expect("Error parsing start date"));
        let end_date = Utc::now();

        let root_drawing_area = BitMapBackend::new("stocks.png", (1920, 1080)).into_drawing_area();
        root_drawing_area.fill(&WHITE).expect("Some plotting error");

        let mut chart = ChartBuilder::on(&root_drawing_area)
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .caption(
                "Adjusted Close Price Over Time",
                ("sans-serif", 30).into_font(),
            )
            .build_cartesian_2d(start_date..end_date, -200.0_f32..self.api_ref.config.y_max)
            .expect("Problem in creating graph");

        chart
            .configure_mesh()
            .x_label_formatter(&|d| d.format("%Y-%m-%d").to_string())
            .y_desc("Growth (%)")
            .x_desc("Date")
            .draw()
            .expect("mesh problem");

        for (i, stock) in self.api_ref.data.iter().enumerate() {
            let y_0 = stock.historical_data.first().expect("no first y").adj_close;

            let color = Palette99::pick(i).filled();

            chart
                .draw_series(LineSeries::new(
                    stock.historical_data.iter().map(|d| {
                        let growth = (d.adj_close - y_0) * 100.0 / y_0;
                        (d.date, growth)
                    }),
                    color,
                ))
                .expect("Error while drawing a plot")
                .label(&self.api_ref.config.stocks[i])
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        }

        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .background_style(&WHITE.mix(0.8))
            .position(SeriesLabelPosition::Coordinate(10, 10))
            .draw()
            .expect("Legend creation error");

        root_drawing_area.present().expect("Plot creation error");
    }
}
