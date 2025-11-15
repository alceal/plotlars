use plotlars::{Axis, OhlcPlot, Plot};
use polars::prelude::*;

fn main() {
    let stock_data = LazyCsvReader::new(PlPath::new("data/stock_prices.csv"))
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    OhlcPlot::builder()
        .data(&stock_data)
        .dates("date")
        .open("open")
        .high("high")
        .low("low")
        .close("close")
        .plot_title("OHLC Plot")
        .y_title("price ($)")
        .y_axis(&Axis::new().show_axis(true))
        .build()
        .plot();
}
