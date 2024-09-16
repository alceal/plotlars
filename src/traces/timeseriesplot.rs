use bon::bon;
use plotly::{
    common::{Line as LinePlotly, Marker, Mode},
    Layout, Scatter, Trace as TracePlotly,
};

use polars::{
    frame::DataFrame,
    prelude::{col, IntoLazy},
};

use crate::{
    aesthetics::{
        line::{Line, LineType},
        mark::Mark,
    },
    colors::Rgb,
    texts::Text,
    traits::{layout::LayoutPlotly, plot::Plot, polar::Polar, trace::Trace},
    Axis, Legend, Orientation, Shape,
};

/// A structure representing a time series plot.
pub struct TimeSeriesPlot {
    traces: Vec<Box<dyn TracePlotly + 'static>>,
    layout: Layout,
}

#[bon]
impl TimeSeriesPlot {
    /// Creates a new `TimeSeriesPlot`.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to the `DataFrame` containing the data to be plotted.
    /// * `x` - A string specifying the column name to be used for the x-axis, typically representing time or dates.
    /// * `y` - A string specifying the column name to be used for the y-axis, typically representing the primary metric.
    /// * `additional_series` - An optional vector of strings specifying additional y-axis columns to be plotted as series.
    /// * `size` - An optional `usize` specifying the size of the markers or line thickness.
    /// * `color` - An optional `Rgb` value specifying the color of the marker to be used for the plot.
    /// * `colors` - An optional vector of `Rgb` values specifying the color for the markers to be used for the plot.
    /// * `with_shape` - An optional `bool` indicating whether to use shapes for markers in the plot.
    /// * `shape` - An optional `Shape` specifying the shape of the markers.
    /// * `shapes` - An optional `Vec<Shape>` specifying multiple shapes for the markers.
    /// * `line_types` - An optional vector of `LineType` specifying the types of lines (e.g., solid, dashed) for each plotted series.
    /// * `line_width` - An optional `f64` specifying the width of the plotted lines.
    /// * `plot_title` - An optional `Text` struct specifying the title of the plot.
    /// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
    /// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
    /// * `x_axis` - An optional reference to an `Axis` struct for customizing the x-axis.
    /// * `y_axis` - An optional reference to an `Axis` struct for customizing the y-axis.
    /// * `legend_title` - An optional `Text` struct specifying the title of the legend.
    /// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot (e.g., positioning, font, etc.).
    ///
    /// # Returns
    ///
    /// Returns an instance of `TimeSeriesPlot`.
    ///
    /// **Example**
    ///
    /// ```
    /// TimeSeriesPlot::builder()
    ///     .data(&timeseries_dataset)
    ///     .x("date")
    ///     .y("series_1")
    ///     .additional_series(vec!["series_2"])
    ///     .size(5)
    ///     .colors(vec![Rgb(255, 0, 0), Rgb(0, 255, 0)])
    ///     .line_types(vec![LineType::Dash, LineType::Solid])
    ///     .shapes(vec![Shape::Circle, Shape::Square])
    ///     .plot_title(
    ///         Text::from("Time Series Plot")
    ///             .font("Arial")
    ///             .size(18)
    ///     )
    ///     .x_title(
    ///         Text::from("date")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .y_title(
    ///         Text::from("sales")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .legend_title(
    ///         Text::from("series")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .legend(
    ///         &Legend::new()
    ///             .x(0.05)
    ///             .y(0.9)
    ///     )
    ///     .build()
    ///     .plot();
    /// ```
    ///
    /// ![Time Series Plot](https://imgur.com/k0FShJp.png)
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        // Data
        data: &DataFrame,
        x: String,
        y: String,
        additional_series: Option<Vec<&str>>,
        // Marker
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        with_shape: Option<bool>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        line_types: Option<Vec<LineType>>,
        line_width: Option<f64>,
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
        let group = None;
        let error = None;
        let box_points = None;
        let point_offset = None;
        let jitter = None;
        let opacity = None;

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

impl LayoutPlotly for TimeSeriesPlot {}
impl Polar for TimeSeriesPlot {}
impl Mark for TimeSeriesPlot {}
impl Line for TimeSeriesPlot {}

impl Trace for TimeSeriesPlot {
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
        with_shape: Option<bool>,
        marker: Marker,
        line: LinePlotly,
    ) -> Box<dyn TracePlotly + 'static> {
        let x_data = Self::get_string_column(data, x_col);
        let y_data = Self::get_numeric_column(data, y_col);

        let mut trace = Scatter::default().x(x_data).y(y_data);

        if let Some(with_shape) = with_shape {
            if with_shape {
                trace = trace.mode(Mode::LinesMarkers);
            } else {
                trace = trace.mode(Mode::Lines);
            }
        }

        trace = trace.marker(marker);
        trace = trace.line(line);

        if let Some(name) = group_name {
            trace = trace.name(name);
        }

        trace
    }

    fn create_traces(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        orientation: Option<Orientation>,
        #[allow(unused_variables)] group: Option<String>,
        error: Option<String>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
        additional_series: Option<Vec<&str>>,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        with_shape: Option<bool>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        line_type: Option<Vec<LineType>>,
        line_width: Option<f64>,
    ) -> Vec<Box<dyn TracePlotly + 'static>> {
        let mut traces: Vec<Box<dyn TracePlotly + 'static>> = Vec::new();

        let mark = Self::create_marker(opacity, size);
        let line = Self::create_line();

        let series_mark = Self::set_color(&mark, &color, &colors, 0);

        let series_mark = Self::set_shape(&series_mark, &shape, &shapes, 0);

        let series_line = Self::set_line_type(&line, &line_type, line_width, 0);

        let group_name = Some(y_col);

        let trace = Self::create_trace(
            data,
            x_col,
            y_col,
            orientation.clone(),
            group_name,
            error.clone(),
            box_points,
            point_offset,
            jitter,
            with_shape,
            series_mark,
            series_line,
        );

        traces.push(trace);

        if let Some(additional_series) = additional_series {
            let additional_series = additional_series.into_iter();

            for (i, series) in additional_series.enumerate() {
                let series_mark = Self::set_color(&mark, &color, &colors, i + 1);

                let series_mark = Self::set_shape(&series_mark, &shape, &shapes, i + 1);

                let series_line = Self::set_line_type(&line, &line_type, line_width, i + 1);

                let subset = data
                    .clone()
                    .lazy()
                    .select([col(x_col), col(series)])
                    .collect()
                    .unwrap();

                let group_name = Some(series);

                let trace = Self::create_trace(
                    &subset,
                    x_col,
                    series,
                    orientation.clone(),
                    group_name,
                    error.clone(),
                    box_points,
                    point_offset,
                    jitter,
                    with_shape,
                    series_mark,
                    series_line,
                );

                traces.push(trace);
            }
        }

        traces
    }
}

impl Plot for TimeSeriesPlot {
    fn get_layout(&self) -> &Layout {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn TracePlotly + 'static>> {
        &self.traces
    }
}
