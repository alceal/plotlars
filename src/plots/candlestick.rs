use bon::bon;

use plotly::{Layout as LayoutPlotly, Trace};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper, Polar},
    components::{Axis, Direction, Text},
    ir::data::ColumnData,
    ir::layout::LayoutIR,
    ir::trace::{CandlestickPlotIR, TraceIR},
};

/// A structure representing a Candlestick financial chart.
///
/// The `CandlestickPlot` struct facilitates the creation and customization of candlestick charts commonly used
/// for visualizing financial data such as stock prices. It supports custom styling for increasing/decreasing
/// values, whisker width configuration, hover information, and comprehensive layout customization
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
/// * `increasing` - An optional reference to a `Direction` struct for customizing increasing candlesticks.
/// * `decreasing` - An optional reference to a `Direction` struct for customizing decreasing candlesticks.
/// * `whisker_width` - An optional `f64` specifying the width of the whiskers (0-1 range).
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
/// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
/// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
/// * `x_axis` - An optional reference to an `Axis` struct for customizing the x-axis.
/// * `y_axis` - An optional reference to an `Axis` struct for customizing the y-axis.
///
/// # Examples
///
/// ```rust
/// use plotlars::{Axis, CandlestickPlot, Direction, Plot, Rgb};
/// use polars::prelude::*;
///
/// let stock_data = LazyCsvReader::new(PlRefPath::new("data/stock_prices.csv"))
///     .finish()
///     .unwrap()
///     .collect()
///     .unwrap();
///
/// let increasing = Direction::new()
///     .line_color(Rgb(0, 200, 100))
///     .line_width(0.5);
///
/// let decreasing = Direction::new()
///     .line_color(Rgb(200, 50, 50))
///     .line_width(0.5);
///
/// CandlestickPlot::builder()
///     .data(&stock_data)
///     .dates("date")
///     .open("open")
///     .high("high")
///     .low("low")
///     .close("close")
///     .increasing(&increasing)
///     .decreasing(&decreasing)
///     .whisker_width(0.1)
///     .plot_title("Candlestick Plot")
///     .y_title("Price ($)")
///     .y_axis(
///         &Axis::new()
///             .show_axis(true)
///             .show_grid(true)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/fNDRLDX.png)
#[derive(Clone, Serialize)]
#[allow(dead_code)]
pub struct CandlestickPlot {
    #[serde(skip)]
    ir_traces: Vec<TraceIR>,
    #[serde(skip)]
    ir_layout: LayoutIR,
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl CandlestickPlot {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        dates: &str,
        open: &str,
        high: &str,
        low: &str,
        close: &str,
        increasing: Option<&Direction>,
        decreasing: Option<&Direction>,
        whisker_width: Option<f64>,
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

        // Build IR
        let ir_trace = TraceIR::CandlestickPlot(CandlestickPlotIR {
            dates: ColumnData::String(Self::get_string_column(data, dates)),
            open: ColumnData::Numeric(Self::get_numeric_column(data, open)),
            high: ColumnData::Numeric(Self::get_numeric_column(data, high)),
            low: ColumnData::Numeric(Self::get_numeric_column(data, low)),
            close: ColumnData::Numeric(Self::get_numeric_column(data, close)),
            increasing: increasing.cloned(),
            decreasing: decreasing.cloned(),
            whisker_width,
        });
        let ir_traces = vec![ir_trace];
        let ir_layout = LayoutIR {
            title: plot_title.clone(),
            x_title: x_title.clone(),
            y_title: y_title.clone(),
            y2_title: None,
            z_title: None,
            legend_title: None,
            legend: None,
            dimensions: None,
            bar_mode: None,
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

        // Build plotly types from IR
        let plotly_traces: Vec<Box<dyn Trace + 'static>> = ir_traces
            .iter()
            .map(crate::plotly_conversions::trace::convert)
            .collect();

        let layout = Self::create_layout(
            plot_title, x_title, y_title, y_title2, z_title, legend_title, x_axis, y_axis,
            y_axis2, z_axis, legend, None,
        );

        Self {
            ir_traces,
            ir_layout,
            traces: plotly_traces,
            layout,
        }
    }
}

impl Layout for CandlestickPlot {}
impl Polar for CandlestickPlot {}

impl PlotHelper for CandlestickPlot {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }

    #[allow(private_interfaces)]
    fn get_ir_layout(&self) -> Option<&LayoutIR> {
        Some(&self.ir_layout)
    }

    #[allow(private_interfaces)]
    fn get_ir_traces(&self) -> Option<&[TraceIR]> {
        Some(&self.ir_traces)
    }
}
