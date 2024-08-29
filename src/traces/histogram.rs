use bon::bon;

use plotly::{
    common::{Line as LinePlotly, Marker},
    histogram::HistFunc,
    layout::BarMode,
    Histogram as HistogramPlotly, Layout, Trace as TracePlotly,
};

use polars::frame::DataFrame;

use crate::{
    aesthetics::{line::Line, mark::Mark},
    colors::Rgb,
    texts::Text,
    traits::layout::LayoutPlotly,
    traits::plot::Plot,
    traits::polar::Polar,
    traits::trace::Trace,
};

/// A structure representing a histogram.
pub struct Histogram {
    traces: Vec<Box<dyn TracePlotly + 'static>>,
    layout: Layout,
}

#[bon]
impl Histogram {
    /// Creates a new `Histogram`.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to the `DataFrame` containing the data to be plotted.
    /// * `x` - A string specifying the column name to be used for the x-axis.
    /// * `group` - An optional string specifying the column name to be used for grouping data points.
    /// * `opacity` - An optional f64 value specifying the opacity of the plot markers (range: 0.0 to 1.0).
    /// * `colors` - An optional vector of `Rgb` values specifying the colors to be used for the plot.
    /// * `plot_title` - An optional `Text` struct specifying the title of the plot.
    /// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
    /// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
    /// * `legend_title` - An optional `Text` struct specifying the title of the legend.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Histogram`.
    ///
    /// # Example
    ///
    /// ```
    /// Histogram::builder()
    ///     .data(&dataset)
    ///     .x("body_mass_g")
    ///     .group("species")
    ///     .opacity(0.5)
    ///     .colors(vec![
    ///         Rgb(255, 0, 0),
    ///         Rgb(0, 255, 0),
    ///         Rgb(0, 0, 255),
    ///     ])
    ///     .plot_title(
    ///         Text::from("Histogram")
    ///             .font("Arial")
    ///             .size(18)
    ///     )
    ///     .x_title(
    ///         Text::from("body mass (g)")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .y_title(
    ///         Text::from("count")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .legend_title(
    ///         Text::from("species")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .build()
    ///     .plot();
    /// ```
    ///
    /// ![Histogram](https://imgur.com/ZNomy9V.png)
    ///
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: String,
        group: Option<String>,
        // Marker
        opacity: Option<f64>,
        colors: Option<Vec<Rgb>>,
        // Layout
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        legend_title: Option<Text>,
    ) -> Self {
        let x_col = x.as_str();

        // Layout
        let bar_mode = Some(BarMode::Overlay);

        let layout = Self::create_layout(bar_mode, plot_title, x_title, y_title, legend_title);

        // Trace
        let y_col = "";
        let error = None;
        let box_points = None;
        let point_offset = None;
        let jitter = None;
        let additional_series = None;

        let size = None;
        let line_types = None;

        let traces = Self::create_traces(
            data,
            x_col,
            y_col,
            group,
            error,
            box_points,
            point_offset,
            jitter,
            additional_series,
            opacity,
            size,
            colors,
            line_types,
        );

        Self { traces, layout }
    }
}

impl LayoutPlotly for Histogram {}
impl Polar for Histogram {}
impl Mark for Histogram {}
impl Line for Histogram {}

impl Trace for Histogram {
    fn create_trace(
        data: &DataFrame,
        x_col: &str,
        #[allow(unused_variables)] y_col: &str,
        group_name: Option<&str>,
        #[allow(unused_variables)] error: Option<String>,
        #[allow(unused_variables)] box_points: Option<bool>,
        #[allow(unused_variables)] point_offset: Option<f64>,
        #[allow(unused_variables)] jitter: Option<f64>,
        marker: Marker,
        #[allow(unused_variables)] line: LinePlotly,
    ) -> Box<dyn TracePlotly + 'static> {
        let x_data = Self::get_numeric_column(data, x_col);

        let mut trace = HistogramPlotly::default()
            .x(x_data)
            .hist_func(HistFunc::Count);

        trace = trace.marker(marker);

        if let Some(name) = group_name {
            trace = trace.name(name);
        }

        trace
    }
}

impl Plot for Histogram {
    fn get_layout(&self) -> &Layout {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn TracePlotly + 'static>> {
        &self.traces
    }
}
