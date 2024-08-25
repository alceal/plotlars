use bon::bon;

use plotly::{
    common::{Line as LinePlotly, Marker, Mode},
    Layout, Scatter, Trace as TracePlotly,
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

/// A structure representing a scatter plot.
pub struct ScatterPlot {
    traces: Vec<Box<dyn TracePlotly + 'static>>,
    layout: Layout,
}

#[bon]
impl ScatterPlot {
    /// Creates a new `ScatterPlot`.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to the `DataFrame` containing the data to be plotted.
    /// * `x` - A string specifying the column name to be used for the x-axis.
    /// * `y` - A string specifying the column name to be used for the y-axis.
    /// * `group` - An optional string specifying the column name to be used for grouping data points.
    /// * `opacity` - An optional f64 value specifying the opacity of the plot markers (range: 0.0 to 1.0).
    /// * `size` - An optional `usize` specifying the size of the markers.
    /// * `colors` - An optional vector of `Rgb` values specifying the colors to be used for the plot.
    /// * `plot_title` - An optional `Text` struct specifying the title of the plot.
    /// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
    /// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
    /// * `legend_title` - An optional `Text` struct specifying the title of the legend.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ScatterPlot`.
    ///
    /// # Example
    ///
    /// ```
    /// ScatterPlot::builder()
    ///     .data(&dataset)
    ///     .x("body_mass_g")
    ///     .y("flipper_length_mm")
    ///     .group("species")
    ///     .opacity(0.5)
    ///     .size(20)
    ///     .colors(vec![
    ///         Rgb(255, 0, 0),
    ///         Rgb(0, 255, 0),
    ///         Rgb(0, 0, 255),
    ///     ])
    ///     .plot_title(
    ///         Text::from("Scatter Plot")
    ///             .font("Arial")
    ///             .size(18)
    ///     )
    ///     .x_title(
    ///         Text::from("body mass (g)")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .y_title(
    ///         Text::from("flipper length (mm)")
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
    /// ![Scatter Plot](https://imgur.com/f5vgrNd.png)
    ///
    #[builder]
    pub fn new(
        // Data
        data: &DataFrame,
        x: String,
        y: String,
        group: Option<String>,
        // Marker
        opacity: Option<f64>,
        size: Option<usize>,
        colors: Option<Vec<Rgb>>,
        // Layout
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        legend_title: Option<Text>,
    ) -> Self {
        let x_col = x.as_str();
        let y_col = y.as_str();

        // Layout
        let bar_mode = None;

        let layout = Self::create_layout(bar_mode, plot_title, x_title, y_title, legend_title);

        // Trace
        let error = None;
        let box_points = None;
        let point_offset = None;
        let jitter = None;
        let aditional_series = None;
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
            aditional_series,
            opacity,
            size,
            colors,
            line_types,
        );

        Self { traces, layout }
    }
}

impl LayoutPlotly for ScatterPlot {}
impl Polar for ScatterPlot {}
impl Mark for ScatterPlot {}
impl Line for ScatterPlot {}

impl Trace for ScatterPlot {
    fn create_trace(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        group_name: Option<&str>,
        #[allow(unused_variables)] error: Option<String>,
        #[allow(unused_variables)] box_points: Option<bool>,
        #[allow(unused_variables)] point_offset: Option<f64>,
        #[allow(unused_variables)] jitter: Option<f64>,
        marker: Marker,
        #[allow(unused_variables)] line: LinePlotly,
    ) -> Box<dyn TracePlotly + 'static> {
        let x_data = Self::get_numeric_column(data, x_col);
        let y_data = Self::get_numeric_column(data, y_col);

        let mut trace = Scatter::default().x(x_data).y(y_data).mode(Mode::Markers);

        trace = trace.marker(marker);

        if let Some(name) = group_name {
            trace = trace.name(name);
        }

        trace
    }
}

impl Plot for ScatterPlot {
    fn get_layout(&self) -> &Layout {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn TracePlotly + 'static>> {
        &self.traces
    }
}
