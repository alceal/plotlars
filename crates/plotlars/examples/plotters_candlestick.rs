use plotlars::{CandlestickPlot, Direction, Plot, Rgb};
use polars::prelude::*;

fn main() {
    let dataset = LazyCsvReader::new(PlRefPath::new("data/financial_timeseries.csv"))
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    let increasing = Direction::new()
        .line_color(Rgb(38, 166, 91))
        .line_width(1.0);

    let decreasing = Direction::new()
        .line_color(Rgb(239, 85, 59))
        .line_width(1.0);

    CandlestickPlot::builder()
        .data(&dataset)
        .dates("date")
        .open("open")
        .high("high")
        .low("low")
        .close("close")
        .increasing(&increasing)
        .decreasing(&decreasing)
        .plot_title("Candlestick Plot")
        .build()
        .plot();
}
