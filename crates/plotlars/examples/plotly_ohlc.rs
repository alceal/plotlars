use plotlars::{Axis, CsvReader, OhlcPlot, Plot};

fn main() {
    let stock_data = CsvReader::new("data/stock_prices.csv").finish().unwrap();

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
