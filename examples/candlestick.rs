use plotlars::{Axis, CandlestickPlot, Direction, Plot, Rgb};
use polars::prelude::*;

fn main() {
    // Create sample candlestick data
    let dates = vec![
        "2024-01-01",
        "2024-01-02",
        "2024-01-03",
        "2024-01-04",
        "2024-01-05",
        "2024-01-08",
        "2024-01-09",
        "2024-01-10",
        "2024-01-11",
        "2024-01-12",
        "2024-01-15",
        "2024-01-16",
        "2024-01-17",
        "2024-01-18",
        "2024-01-19",
        "2024-01-22",
        "2024-01-23",
        "2024-01-24",
        "2024-01-25",
        "2024-01-26",
    ];

    let open_prices = vec![
        100.0, 102.5, 101.0, 103.5, 105.0, 104.5, 106.0, 105.5, 107.0, 108.5, 108.0, 110.0, 109.5,
        111.0, 112.5, 112.0, 113.5, 113.0, 114.5, 115.0,
    ];

    let high_prices = vec![
        103.0, 104.0, 103.5, 106.0, 107.5, 107.0, 108.5, 108.0, 109.5, 111.0, 110.5, 112.5, 112.0,
        113.5, 115.0, 114.5, 116.0, 115.5, 117.0, 117.5,
    ];

    let low_prices = vec![
        99.0, 101.5, 100.0, 102.5, 104.0, 103.5, 105.0, 104.5, 106.0, 107.5, 107.0, 109.0, 108.5,
        110.0, 111.5, 111.0, 112.5, 112.0, 113.5, 114.0,
    ];

    let close_prices = vec![
        102.5, 101.0, 103.5, 105.0, 104.5, 106.0, 105.5, 107.0, 108.5, 108.0, 110.0, 109.5, 111.0,
        112.5, 112.0, 113.5, 113.0, 114.5, 115.0, 116.5,
    ];

    let stock_data = df! {
        "date" => dates,
        "open" => open_prices,
        "high" => high_prices,
        "low" => low_prices,
        "close" => close_prices,
    }
    .unwrap();

    // Candlestick chart with whisker width customization
    let increasing = Direction::new()
        .line_color(Rgb(0, 200, 100)) // Green
        .line_width(0.5);

    let decreasing = Direction::new()
        .line_color(Rgb(200, 50, 50)) // Red
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
        .whisker_width(0.1) // Thin whiskers
        .plot_title("Stock Price - Thin Whiskers")
        .y_title("Price ($)")
        .y_axis(&Axis::new().show_axis(true).show_grid(true))
        .build()
        .plot();
}