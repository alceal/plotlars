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
    traits::{layout::LayoutPlotly, plot::Plot, polar::Polar, trace::Trace},
    Axis, Legend, Orientation, Shape,
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
    /// * `color` - An optional `Rgb` value specifying the color of the marker to be used for the plot.
    /// * `colors` - An optional vector of `Rgb` values specifying the color for the markers to be used for the plot.
    /// * `shape` - An optional `Shape` specifying the shape of the markers.
    /// * `shapes` - An optional `Vec<Shape>` specifying multiple shapes for the markers.
    /// * `plot_title` - An optional `Text` struct specifying the title of the plot.
    /// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
    /// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
    /// * `legend_title` - An optional `Text` struct specifying the title of the legend.
    /// * `x_axis` - An optional reference to an `Axis` struct for customizing the x-axis.
    /// * `y_axis` - An optional reference to an `Axis` struct for customizing the y-axis.
    /// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot (e.g., positioning, font, etc.).
    ///
    /// # Returns
    ///
    /// Returns an instance of `ScatterPlot`.
    ///
    /// **Example**
    ///
    /// ```
    /// let axis_format = Axis::new()
    ///     .show_line(true)
    ///     .tick_direction(TickDirection::OutSide)
    ///     .value_thousands(true);
    ///
    /// ScatterPlot::builder()
    ///     .data(&dataset)
    ///     .x("body_mass_g")
    ///     .y("flipper_length_mm")
    ///     .group("species")
    ///     .opacity(0.5)
    ///     .size(12)
    ///     .colors(vec![
    ///         Rgb(255, 0, 0),
    ///         Rgb(0, 255, 0),
    ///         Rgb(0, 0, 255),
    ///     ])
    ///     .shapes(vec![Shape::Circle, Shape::Square, Shape::Diamond])
    ///     .plot_title(
    ///         Text::from("Scatter Plot")
    ///             .font("Arial")
    ///             .size(20)
    ///             .x(0.045)
    ///     )
    ///     .x_title("body mass (g)")
    ///     .y_title("flipper length (mm)")
    ///     .legend_title("species")
    ///     .x_axis(&axis_format)
    ///     .y_axis(&axis_format)
    ///     .build()
    ///     .plot();
    /// ```
    ///
    /// ![Scatter Plot](https://imgur.com/LQm4we9.png)
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        // Data
        data: &DataFrame,
        x: String,
        y: String,
        group: Option<String>,
        // Marker
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        // Layout
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        legend: Option<&Legend>,
    ) -> Self {
        let x_col = x.as_str();
        let y_col = y.as_str();

        // Layout
        let bar_mode = None;

        let layout = Self::create_layout(
            bar_mode,
            plot_title,
            x_title,
            x_axis,
            y_title,
            y_axis,
            legend_title,
            legend,
        );

        // Trace
        let orientation = None;
        let error = None;
        let box_points = None;
        let point_offset = None;
        let jitter = None;
        let additional_series = None;
        let line_types = None;
        let with_shape = None;
        let line_width = None;

        let traces = Self::create_traces(
            data,
            x_col,
            y_col,
            orientation,
            group,
            error,
            box_points,
            point_offset,
            jitter,
            additional_series,
            opacity,
            size,
            color,
            colors,
            with_shape,
            shape,
            shapes,
            line_types,
            line_width,
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
        #[allow(unused_variables)] orientation: Option<Orientation>,
        group_name: Option<&str>,
        #[allow(unused_variables)] error: Option<String>,
        #[allow(unused_variables)] box_points: Option<bool>,
        #[allow(unused_variables)] point_offset: Option<f64>,
        #[allow(unused_variables)] jitter: Option<f64>,
        #[allow(unused_variables)] with_shape: Option<bool>,
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
