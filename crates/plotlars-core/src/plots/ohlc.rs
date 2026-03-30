use bon::bon;

use polars::frame::DataFrame;

use crate::{
    components::{Axis, Text},
    ir::data::ColumnData,
    ir::layout::LayoutIR,
    ir::trace::{OhlcPlotIR, TraceIR},
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
/// let stock_data = LazyCsvReader::new(PlRefPath::new("data/stock_prices.csv"))
///     .finish()
///     .unwrap()
///     .collect()
///     .unwrap();
///
/// OhlcPlot::builder()
///     .data(&stock_data)
///     .dates("date")
///     .open("open")
///     .high("high")
///     .low("low")
///     .close("close")
///     .plot_title("OHLC Plot")
///     .y_title("Price ($)")
///     .y_axis(
///         &Axis::new()
///             .show_axis(true)
///     )
///     .build()
///     .plot();
/// ```
/// ![Exmple](https://imgur.com/Sv8r9VN.png)
#[derive(Clone)]
#[allow(dead_code)]
pub struct OhlcPlot {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
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
        // Build IR
        let ir_trace = TraceIR::OhlcPlot(OhlcPlotIR {
            dates: ColumnData::String(crate::data::get_string_column(data, dates)),
            open: ColumnData::Numeric(crate::data::get_numeric_column(data, open)),
            high: ColumnData::Numeric(crate::data::get_numeric_column(data, high)),
            low: ColumnData::Numeric(crate::data::get_numeric_column(data, low)),
            close: ColumnData::Numeric(crate::data::get_numeric_column(data, close)),
            tick_width,
        });
        let traces = vec![ir_trace];
        let layout = LayoutIR {
            title: plot_title.clone(),
            x_title: x_title.clone(),
            y_title: y_title.clone(),
            y2_title: None,
            z_title: None,
            legend_title: None,
            legend: None,
            dimensions: None,
            bar_mode: None,
            box_mode: None,
            box_gap: None,
            margin_bottom: None,
            axes_2d: Some(crate::ir::layout::Axes2dIR {
                x_axis: x_axis.cloned(),
                y_axis: y_axis.cloned(),
                y2_axis: None,
            }),
            scene_3d: None,
            polar: None,
            mapbox: None,
            grid: None,
            annotations: vec![],
        };
        Self { traces, layout }
    }
}

impl crate::Plot for OhlcPlot {
    fn ir_traces(&self) -> &[TraceIR] {
        &self.traces
    }

    fn ir_layout(&self) -> &LayoutIR {
        &self.layout
    }
}
