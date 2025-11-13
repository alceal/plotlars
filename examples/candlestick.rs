use plotlars::{Axis, CandlestickPlot, Direction, Plot, Rgb};
use polars::prelude::*;

fn main() {
    let stock_data = LazyCsvReader::new(PlPath::new("data/stock_prices.csv"))
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    let increasing = Direction::new()
        .line_color(Rgb(0, 200, 100))
        .line_width(0.5);

    let decreasing = Direction::new()
        .line_color(Rgb(200, 50, 50))
        .line_width(0.5);

    CandlestickPlot::builder()
        .data(&stock_data)
        .dates("date")
        .open("open")
        .high("high")
        .low("low")
        .close("close")
        .increasing(&increasing)
        .decreasing(&decreasing)
        .whisker_width(0.1)
        .plot_title("Candlestick")
        .y_title("Price ($)")
        .y_axis(&Axis::new().show_axis(true).show_grid(true))
        .build()
        .plot();
}
