use bon::bon;

use plotly::{
    common::{Line as LinePlotly, Marker as MarkerPlotly, Mode},
    Layout as LayoutPlotly, Scatter, Trace,
};

use polars::{
    frame::DataFrame,
    prelude::{col, IntoLazy},
};
use serde::Serialize;

use crate::{
    common::{Layout, Line, Marker, PlotHelper, Polar},
    components::{Axis, FacetConfig, Legend, Line as LineStyle, Rgb, Shape, Text},
};

/// A structure representing a time series plot.
///
/// The `TimeSeriesPlot` struct facilitates the creation and customization of time series plots with various options
/// for data selection, grouping, layout configuration, and aesthetic adjustments. It supports the addition of multiple
/// series, customization of marker shapes, colors, sizes, opacity settings, and comprehensive layout customization
/// including titles, axes, and legends.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `x` - A string slice specifying the column name to be used for the x-axis, typically representing time or dates.
/// * `y` - A string slice specifying the column name to be used for the y-axis, typically representing the primary metric.
/// * `additional_series` - An optional vector of string slices specifying additional y-axis columns to be plotted as series.
/// * `facet` - An optional string slice specifying the column name to be used for faceting (creating multiple subplots).
/// * `facet_config` - An optional reference to a `FacetConfig` struct for customizing facet behavior (grid dimensions, scales, gaps, etc.).
/// * `size` - An optional `usize` specifying the size of the markers or line thickness.
/// * `color` - An optional `Rgb` value specifying the color of the markers. This is used when `group` is not specified.
/// * `colors` - An optional vector of `Rgb` values specifying the colors for the markers. This is used when `group` is specified to differentiate between groups.
/// * `with_shape` - An optional `bool` indicating whether to use shapes for markers in the plot.
/// * `shape` - An optional `Shape` specifying the shape of the markers. This is used when `group` is not specified.
/// * `shapes` - An optional vector of `Shape` values specifying multiple shapes for the markers when plotting multiple groups.
/// * `width` - An optional `f64` specifying the width of the plotted lines.
/// * `line` - An optional `LineStyle` specifying the style of the line. This is used when `additional_series` is not specified.
/// * `lines` - An optional vector of `LineStyle` enums specifying the styles of lines for each plotted series. This is used when `additional_series` is specified to differentiate between multiple series.
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
/// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
/// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
/// * `y2_title` - An optional `Text` struct specifying the title of the secondary y-axis.
/// * `legend_title` - An optional `Text` struct specifying the title of the legend.
/// * `x_axis` - An optional reference to an `Axis` struct for customizing the x-axis.
/// * `y_axis` - An optional reference to an `Axis` struct for customizing the y-axis.
/// * `y2_axis` - An optional reference to an `Axis` struct for customizing the secondary y-axis.
/// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot (e.g., positioning, font, etc.).
///
/// # Examples
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Axis, Legend, Line, Plot, Rgb, Shape, Text, TimeSeriesPlot};
///
/// let dataset = LazyCsvReader::new(PlRefPath::new("data/revenue_and_cost.csv"))
///     .finish()
///     .unwrap()
///     .select([
///         col("Date").cast(DataType::String),
///         col("Revenue").cast(DataType::Int32),
///         col("Cost").cast(DataType::Int32),
///     ])
///     .collect()
///     .unwrap();
///
/// TimeSeriesPlot::builder()
///     .data(&dataset)
///     .x("Date")
///     .y("Revenue")
///     .additional_series(vec!["Cost"])
///     .size(8)
///     .colors(vec![
///         Rgb(0, 0, 255),
///         Rgb(255, 0, 0),
///     ])
///     .lines(vec![
///         Line::Dash,
///         Line::Solid,
///     ])
///     .with_shape(true)
///     .shapes(vec![
///         Shape::Circle,
///         Shape::Square,
///     ])
///     .plot_title(
///         Text::from("Time Series Plot")
///             .font("Arial")
///             .size(18)
///     )
///     .legend(
///         &Legend::new()
///             .x(0.05)
///             .y(0.9)
///     )
///     .x_title("x")
///     .y_title(
///         Text::from("y")
///             .color(Rgb(0, 0, 255))
///     )
///     .y2_title(
///         Text::from("y2")
///             .color(Rgb(255, 0, 0))
///     )
///     .y_axis(
///         &Axis::new()
///             .value_color(Rgb(0, 0, 255))
///             .show_grid(false)
///             .zero_line_color(Rgb(0, 0, 0))
///     )
///     .y2_axis(
///         &Axis::new()
///             .axis_side(plotlars::AxisSide::Right)
///             .value_color(Rgb(255, 0, 0))
///             .show_grid(false)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example1](https://imgur.com/hL27Xcn.png)
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Plot, TimeSeriesPlot, Rgb, Line};
///
/// let dataset = LazyCsvReader::new(PlRefPath::new("data/debilt_2023_temps.csv"))
///     .with_has_header(true)
///     .with_try_parse_dates(true)
///     .finish()
///     .unwrap()
///     .with_columns(vec![
///         (col("tavg") / lit(10)).alias("tavg"),
///         (col("tmin") / lit(10)).alias("tmin"),
///         (col("tmax") / lit(10)).alias("tmax"),
///     ])
///     .collect()
///     .unwrap();
///
///     TimeSeriesPlot::builder()
///     .data(&dataset)
///     .x("date")
///     .y("tavg")
///     .additional_series(vec!["tmin", "tmax"])
///     .colors(vec![
///         Rgb(128, 128, 128),
///         Rgb(0, 122, 255),
///         Rgb(255, 128, 0),
///     ])
///     .lines(vec![
///         Line::Solid,
///         Line::Dot,
///         Line::Dot,
///     ])
///     .plot_title("Temperature at De Bilt (2023)")
///     .legend_title("Legend")
///     .build()
///     .plot();
/// ```
///
/// ![Example2](https://imgur.com/NBioox6.png)
#[derive(Clone, Serialize)]
pub struct TimeSeriesPlot {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl TimeSeriesPlot {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: &str,
        y: &str,
        additional_series: Option<Vec<&str>>,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        with_shape: Option<bool>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        line: Option<LineStyle>,
        lines: Option<Vec<LineStyle>>,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        y2_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        y2_axis: Option<&Axis>,
        legend: Option<&Legend>,
    ) -> Self {
        let z_title = None;
        let z_axis = None;

        let (layout, traces) = match facet {
            Some(facet_column) => {
                let config = facet_config.cloned().unwrap_or_default();

                let layout = Self::create_faceted_layout(
                    data,
                    facet_column,
                    &config,
                    plot_title,
                    x_title,
                    y_title,
                    legend_title,
                    x_axis,
                    y_axis,
                    legend,
                );

                let traces = Self::create_faceted_traces(
                    data,
                    x,
                    y,
                    additional_series,
                    facet_column,
                    &config,
                    size,
                    color,
                    colors,
                    with_shape,
                    shape,
                    shapes,
                    width,
                    line,
                    lines,
                );

                (layout, traces)
            }
            None => {
                let mut has_y2_axis = false;

                if y2_axis.is_some() {
                    has_y2_axis = true;
                }

                let layout = Self::create_layout(
                    plot_title,
                    x_title,
                    y_title,
                    y2_title,
                    z_title,
                    legend_title,
                    x_axis,
                    y_axis,
                    y2_axis,
                    z_axis,
                    legend,
                    None,
                );

                let traces = Self::create_traces(
                    data,
                    x,
                    y,
                    additional_series,
                    has_y2_axis,
                    size,
                    color,
                    colors,
                    with_shape,
                    shape,
                    shapes,
                    width,
                    line,
                    lines,
                );

                (layout, traces)
            }
        };

        Self { traces, layout }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        additional_series: Option<Vec<&str>>,
        has_y2_axis: bool,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        with_shape: Option<bool>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        style: Option<LineStyle>,
        styles: Option<Vec<LineStyle>>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

        let opacity = None;

        let marker = Self::create_marker(
            0,
            opacity,
            size,
            color,
            colors.clone(),
            shape,
            shapes.clone(),
        );

        let line = Self::create_line(0, width, style, styles.clone());

        let name = Some(y_col);
        let mut y_axis_index = "";

        let trace = Self::create_trace(
            data,
            x_col,
            y_col,
            name,
            with_shape,
            marker,
            line,
            y_axis_index,
        );

        traces.push(trace);

        if let Some(additional_series) = additional_series {
            let additional_series = additional_series.into_iter();

            for (i, series) in additional_series.enumerate() {
                let marker = Self::create_marker(
                    i + 1,
                    opacity,
                    size,
                    color,
                    colors.clone(),
                    shape,
                    shapes.clone(),
                );

                let line = Self::create_line(i + 1, width, style, styles.clone());

                let subset = data
                    .clone()
                    .lazy()
                    .select([col(x_col), col(series)])
                    .collect()
                    .unwrap();

                let name = Some(series);

                if has_y2_axis {
                    y_axis_index = "y2";
                }

                let trace = Self::create_trace(
                    &subset,
                    x_col,
                    series,
                    name,
                    with_shape,
                    marker,
                    line,
                    y_axis_index,
                );

                traces.push(trace);
            }
        }

        traces
    }

    #[allow(clippy::too_many_arguments)]
    fn create_trace(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        name: Option<&str>,
        with_shape: Option<bool>,
        marker: MarkerPlotly,
        line: LinePlotly,
        index: &str,
    ) -> Box<dyn Trace + 'static> {
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

        if let Some(name) = name {
            trace = trace.name(name);
        }

        trace.y_axis(index)
    }

    #[allow(clippy::too_many_arguments)]
    fn build_timeseries_trace_with_axes(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        name: Option<&str>,
        with_shape: Option<bool>,
        marker: MarkerPlotly,
        line: LinePlotly,
        x_axis: Option<&str>,
        y_axis: Option<&str>,
        show_legend: bool,
        legend_group: Option<&str>,
    ) -> Box<dyn Trace + 'static> {
        let x_data = Self::get_string_column(data, x_col);
        let y_data = Self::get_numeric_column(data, y_col);

        let mut trace = Scatter::default().x(x_data).y(y_data);

        if let Some(with_shape) = with_shape {
            if with_shape {
                trace = trace.mode(Mode::LinesMarkers);
            } else {
                trace = trace.mode(Mode::Lines);
            }
        } else {
            trace = trace.mode(Mode::Lines);
        }

        trace = trace.marker(marker);
        trace = trace.line(line);

        if let Some(name) = name {
            trace = trace.name(name);
        }

        if let Some(axis) = x_axis {
            trace = trace.x_axis(axis);
        }

        if let Some(axis) = y_axis {
            trace = trace.y_axis(axis);
        }

        let trace = if let Some(group) = legend_group {
            trace.legend_group(group)
        } else {
            trace
        };

        if !show_legend {
            trace.show_legend(false)
        } else {
            trace
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_faceted_traces(
        data: &DataFrame,
        x: &str,
        y: &str,
        additional_series: Option<Vec<&str>>,
        facet_column: &str,
        config: &FacetConfig,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        with_shape: Option<bool>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        style: Option<LineStyle>,
        styles: Option<Vec<LineStyle>>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        const MAX_FACETS: usize = 8;

        let facet_categories = Self::get_unique_groups(data, facet_column, config.sorter);

        if facet_categories.len() > MAX_FACETS {
            panic!(
                "Facet column '{}' has {} unique values, but plotly.rs supports maximum {} subplots",
                facet_column,
                facet_categories.len(),
                MAX_FACETS
            );
        }

        let all_y_cols = if let Some(ref add_series) = additional_series {
            let mut cols = vec![y];
            cols.extend(add_series.iter().copied());
            cols
        } else {
            vec![y]
        };

        if let Some(ref color_vec) = colors {
            if additional_series.is_none() {
                let color_count = color_vec.len();
                let facet_count = facet_categories.len();

                if color_count != facet_count {
                    panic!(
                        "When using colors with facet (without additional_series), colors.len() must equal number of facets. \
                         Expected {} colors for {} facets, but got {} colors. \
                         Each facet must be assigned exactly one color.",
                        facet_count, facet_count, color_count
                    );
                }
            } else {
                let color_count = color_vec.len();
                let series_count = all_y_cols.len();

                if color_count < series_count {
                    panic!(
                        "When using colors with additional_series, colors.len() must be >= number of series. \
                         Need at least {} colors for {} series, but got {} colors",
                        series_count, series_count, color_count
                    );
                }
            }
        }

        let mut all_traces = Vec::new();
        let opacity = None;

        if config.highlight_facet {
            for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
                let x_axis = Self::get_axis_reference(facet_idx, "x");
                let y_axis = Self::get_axis_reference(facet_idx, "y");

                for other_facet_value in facet_categories.iter() {
                    if other_facet_value != facet_value {
                        let other_data =
                            Self::filter_data_by_group(data, facet_column, other_facet_value);

                        let grey_color = config.unhighlighted_color.unwrap_or(Rgb(200, 200, 200));

                        for (series_idx, y_col) in all_y_cols.iter().enumerate() {
                            let grey_marker = Self::create_marker(
                                series_idx,
                                opacity,
                                size,
                                Some(grey_color),
                                None,
                                shape,
                                None,
                            );

                            let grey_line =
                                Self::create_line(series_idx, width, style, styles.clone());

                            let trace = Self::build_timeseries_trace_with_axes(
                                &other_data,
                                x,
                                y_col,
                                None,
                                with_shape,
                                grey_marker,
                                grey_line,
                                Some(&x_axis),
                                Some(&y_axis),
                                false,
                                None,
                            );

                            all_traces.push(trace);
                        }
                    }
                }

                let facet_data = Self::filter_data_by_group(data, facet_column, facet_value);

                for (series_idx, y_col) in all_y_cols.iter().enumerate() {
                    let color_index = if additional_series.is_none() {
                        facet_idx
                    } else {
                        series_idx
                    };

                    let marker = Self::create_marker(
                        color_index,
                        opacity,
                        size,
                        color,
                        colors.clone(),
                        shape,
                        shapes.clone(),
                    );

                    let line = Self::create_line(series_idx, width, style, styles.clone());

                    let show_legend = facet_idx == 0;
                    let name = if show_legend { Some(*y_col) } else { None };

                    let trace = Self::build_timeseries_trace_with_axes(
                        &facet_data,
                        x,
                        y_col,
                        name,
                        with_shape,
                        marker,
                        line,
                        Some(&x_axis),
                        Some(&y_axis),
                        show_legend,
                        Some(*y_col),
                    );

                    all_traces.push(trace);
                }
            }
        } else {
            for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
                let facet_data = Self::filter_data_by_group(data, facet_column, facet_value);

                let x_axis = Self::get_axis_reference(facet_idx, "x");
                let y_axis = Self::get_axis_reference(facet_idx, "y");

                for (series_idx, y_col) in all_y_cols.iter().enumerate() {
                    let color_index = if additional_series.is_none() {
                        facet_idx
                    } else {
                        series_idx
                    };

                    let marker = Self::create_marker(
                        color_index,
                        opacity,
                        size,
                        color,
                        colors.clone(),
                        shape,
                        shapes.clone(),
                    );

                    let line = Self::create_line(series_idx, width, style, styles.clone());

                    let show_legend = facet_idx == 0;
                    let name = if show_legend { Some(*y_col) } else { None };

                    let trace = Self::build_timeseries_trace_with_axes(
                        &facet_data,
                        x,
                        y_col,
                        name,
                        with_shape,
                        marker,
                        line,
                        Some(&x_axis),
                        Some(&y_axis),
                        show_legend,
                        Some(*y_col),
                    );

                    all_traces.push(trace);
                }
            }
        }

        all_traces
    }

    #[allow(clippy::too_many_arguments)]
    fn create_faceted_layout(
        data: &DataFrame,
        facet_column: &str,
        config: &FacetConfig,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        legend: Option<&Legend>,
    ) -> LayoutPlotly {
        crate::faceting::create_axis_faceted_layout::<Self>(
            data,
            facet_column,
            config,
            plot_title,
            x_title,
            y_title,
            legend_title,
            x_axis,
            y_axis,
            legend,
        )
    }
}

impl Layout for TimeSeriesPlot {}
impl Line for TimeSeriesPlot {}
impl Marker for TimeSeriesPlot {}
impl Polar for TimeSeriesPlot {}

impl PlotHelper for TimeSeriesPlot {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
