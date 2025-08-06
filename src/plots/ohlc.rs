use bon::bon;

use plotly::{Layout as LayoutPlotly, Ohlc as OhlcPlotly, Trace};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper, Polar},
    components::{Axis, Text},
};

/// A structure representing an OHLC (Open-High-Low-Close) financial chart.
///
/// The `OhlcPlot` struct facilitates the creation and customization of OHLC charts commonly used
/// for visualizing financial data such as stock prices. It supports multiple OHLC series, custom
/// styling for increasing/decreasing values, hover information, and comprehensive layout customization
/// including range selectors and sliders for interactive time navigation.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `dates` - A string slice specifying the column name for dates/timestamps.
/// * `open` - A string slice specifying the column name for opening values.
/// * `high` - A string slice specifying the column name for high values.
/// * `low` - A string slice specifying the column name for low values.
/// * `close` - A string slice specifying the column name for closing values.
/// * `tick_width` - An optional `f64` specifying the width of the open/close ticks (0-1 range).
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
/// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
/// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
/// * `x_axis` - An optional reference to an `Axis` struct for customizing the x-axis.
/// * `y_axis` - An optional reference to an `Axis` struct for customizing the y-axis.
///
/// # Examples
///
/// ```rust
/// use plotlars::{Axis, OhlcPlot, Plot};
/// use polars::prelude::*;
///
/// let dates = vec![
///     "2024-01-01", "2024-01-02", "2024-01-03", "2024-01-04", "2024-01-05",
/// ];
///
/// let open_prices = vec![100.0, 102.5, 101.0, 103.5, 105.0];
/// let high_prices = vec![103.0, 104.0, 103.5, 106.0, 107.5];
/// let low_prices = vec![99.0, 101.5, 100.0, 102.5, 104.0];
/// let close_prices = vec![102.5, 101.0, 103.5, 105.0, 104.5];
///
/// let stock_data = df! {
///     "date" => dates,
///     "open" => open_prices,
///     "high" => high_prices,
///     "low" => low_prices,
///     "close" => close_prices,
/// }
/// .unwrap();
///
/// OhlcPlot::builder()
///     .data(&stock_data)
///     .dates("date")
///     .open("open")
///     .high("high")
///     .low("low")
///     .close("close")
///     .plot_title("Stock Price - Custom Axes")
///     .y_title("Price ($)")
///     .y_axis(&Axis::new().show_axis(true))
///     .build()
///     .plot();
/// ```
/// ![Exmple](https://imgur.com/jZM3bGq.png)
#[derive(Clone, Serialize)]
pub struct OhlcPlot {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl OhlcPlot {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        dates: &str,
        open: &str,
        high: &str,
        low: &str,
        close: &str,
        tick_width: Option<f64>,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
    ) -> Self {
        let z_title = None;
        let y_title2 = None;
        let z_axis = None;
        let y_axis2 = None;
        let legend_title = None;
        let legend = None;

        let layout = Self::create_layout(
            plot_title,
            x_title,
            y_title,
            y_title2,
            z_title,
            legend_title,
            x_axis,
            y_axis,
            y_axis2,
            z_axis,
            legend,
        );

        let traces = Self::create_traces(data, dates, open, high, low, close, tick_width);

        Self { traces, layout }
    }

    fn create_traces(
        data: &DataFrame,
        dates_col: &str,
        open_col: &str,
        high_col: &str,
        low_col: &str,
        close_col: &str,
        tick_width: Option<f64>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

        let trace = Self::create_trace(
            data,
            dates_col,
            open_col,
            high_col,
            low_col,
            close_col,
            tick_width,
        );

        traces.push(trace);
        traces
    }

    fn create_trace(
        data: &DataFrame,
        dates_col: &str,
        open_col: &str,
        high_col: &str,
        low_col: &str,
        close_col: &str,
        tick_width: Option<f64>,
    ) -> Box<dyn Trace + 'static> {
        let dates_data = Self::get_string_column(data, dates_col);
        let open_data = Self::get_numeric_column(data, open_col);
        let high_data = Self::get_numeric_column(data, high_col);
        let low_data = Self::get_numeric_column(data, low_col);
        let close_data = Self::get_numeric_column(data, close_col);

        // Convert Option<f32> to f32 for OHLC trace
        let open_values: Vec<f32> = open_data.into_iter().map(|v| v.unwrap_or(0.0)).collect();
        let high_values: Vec<f32> = high_data.into_iter().map(|v| v.unwrap_or(0.0)).collect();
        let low_values: Vec<f32> = low_data.into_iter().map(|v| v.unwrap_or(0.0)).collect();
        let close_values: Vec<f32> = close_data.into_iter().map(|v| v.unwrap_or(0.0)).collect();
        let dates_values: Vec<String> = dates_data
            .into_iter()
            .map(|v| v.unwrap_or_default())
            .collect();

        let mut trace = *OhlcPlotly::new(
            dates_values,
            open_values,
            high_values,
            low_values,
            close_values,
        );

        // Set tick width
        if let Some(tick_w) = tick_width {
            trace = trace.tick_width(tick_w);
        }

        // Return trace as Box
        Box::new(trace)
    }
}

impl Layout for OhlcPlot {}
impl Polar for OhlcPlot {}

impl PlotHelper for OhlcPlot {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
