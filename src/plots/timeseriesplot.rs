use bon::bon;

use plotly::{
    common::{Line as LinePlotly, Marker as MarkerPlotly, Mode},
    layout::{GridPattern, LayoutGrid},
    Layout as LayoutPlotly, Scatter, Trace,
};

use polars::{
    frame::DataFrame,
    prelude::{col, IntoLazy},
};
use serde::Serialize;

use crate::{
    common::{Layout, Line, Marker, PlotHelper, Polar},
    components::{Axis, FacetConfig, FacetScales, Legend, Line as LineStyle, Rgb, Shape, Text},
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
/// * `legend_title` - An optional `Text` struct specifying the title of the legend.
/// * `x_axis` - An optional reference to an `Axis` struct for customizing the x-axis.
/// * `y_axis` - An optional reference to an `Axis` struct for customizing the y-axis.
/// * `y_axis2` - An optional reference to an `Axis` struct for customizing the y-axis2.
/// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot (e.g., positioning, font, etc.).
///
/// # Examples
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Axis, Legend, Line, Plot, Rgb, Shape, Text, TimeSeriesPlot};
///
/// let dataset = LazyCsvReader::new(PlPath::new("data/revenue_and_cost.csv"))
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
///     .lines(vec![Line::Dash, Line::Solid])
///     .with_shape(true)
///     .shapes(vec![Shape::Circle, Shape::Square])
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
///     .y_title2(
///         Text::from("y2")
///             .color(Rgb(255, 0, 0))
///     )
///     .y_axis(
///         &Axis::new()
///             .value_color(Rgb(0, 0, 255))
///             .show_grid(false)
///             .zero_line_color(Rgb(0, 0, 0))
///     )
///     .y_axis2(
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
/// let dataset = LazyCsvReader::new(PlPath::new("data/debilt_2023_temps.csv"))
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
        y_title2: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        y_axis2: Option<&Axis>,
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
                let mut has_y_axis2 = false;

                if y_axis2.is_some() {
                    has_y_axis2 = true;
                }

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

                let traces = Self::create_traces(
                    data,
                    x,
                    y,
                    additional_series,
                    has_y_axis2,
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
        has_y_axis2: bool,
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

                if has_y_axis2 {
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
        let facet_categories = Self::get_unique_groups(data, facet_column, config.sorter);
        let n_facets = facet_categories.len();

        let (ncols, nrows) = Self::calculate_grid_dimensions(n_facets, config.ncol, config.nrow);

        let mut grid = LayoutGrid::new()
            .rows(nrows)
            .columns(ncols)
            .pattern(GridPattern::Independent);

        if let Some(x_gap) = config.x_gap {
            grid = grid.x_gap(x_gap);
        }
        if let Some(y_gap) = config.y_gap {
            grid = grid.y_gap(y_gap);
        }

        let mut layout = LayoutPlotly::new().grid(grid);

        if let Some(title) = plot_title {
            layout = layout.title(title.to_plotly());
        }

        layout = Self::apply_axis_matching(layout, n_facets, &config.scales);

        layout = Self::apply_facet_axis_titles(
            layout, n_facets, ncols, nrows, x_title, y_title, x_axis, y_axis,
        );

        let annotations =
            Self::create_facet_annotations(&facet_categories, config.title_style.as_ref());
        layout = layout.annotations(annotations);

        layout = layout.legend(Legend::set_legend(legend_title, legend));

        layout
    }

    fn apply_axis_matching(
        mut layout: LayoutPlotly,
        n_facets: usize,
        scales: &FacetScales,
    ) -> LayoutPlotly {
        use plotly::layout::Axis as AxisPlotly;

        match scales {
            FacetScales::Fixed => {
                for i in 1..n_facets {
                    let x_axis = AxisPlotly::new().matches("x");
                    let y_axis = AxisPlotly::new().matches("y");
                    layout = match i {
                        1 => layout.x_axis2(x_axis).y_axis2(y_axis),
                        2 => layout.x_axis3(x_axis).y_axis3(y_axis),
                        3 => layout.x_axis4(x_axis).y_axis4(y_axis),
                        4 => layout.x_axis5(x_axis).y_axis5(y_axis),
                        5 => layout.x_axis6(x_axis).y_axis6(y_axis),
                        6 => layout.x_axis7(x_axis).y_axis7(y_axis),
                        7 => layout.x_axis8(x_axis).y_axis8(y_axis),
                        _ => layout,
                    };
                }
            }
            FacetScales::FreeX => {
                for i in 1..n_facets {
                    let axis = AxisPlotly::new().matches("y");
                    layout = match i {
                        1 => layout.y_axis2(axis),
                        2 => layout.y_axis3(axis),
                        3 => layout.y_axis4(axis),
                        4 => layout.y_axis5(axis),
                        5 => layout.y_axis6(axis),
                        6 => layout.y_axis7(axis),
                        7 => layout.y_axis8(axis),
                        _ => layout,
                    };
                }
            }
            FacetScales::FreeY => {
                for i in 1..n_facets {
                    let axis = AxisPlotly::new().matches("x");
                    layout = match i {
                        1 => layout.x_axis2(axis),
                        2 => layout.x_axis3(axis),
                        3 => layout.x_axis4(axis),
                        4 => layout.x_axis5(axis),
                        5 => layout.x_axis6(axis),
                        6 => layout.x_axis7(axis),
                        7 => layout.x_axis8(axis),
                        _ => layout,
                    };
                }
            }
            FacetScales::Free => {}
        }

        layout
    }

    #[allow(clippy::too_many_arguments)]
    fn apply_facet_axis_titles(
        mut layout: LayoutPlotly,
        n_facets: usize,
        ncols: usize,
        nrows: usize,
        x_title: Option<Text>,
        y_title: Option<Text>,
        x_axis_config: Option<&Axis>,
        y_axis_config: Option<&Axis>,
    ) -> LayoutPlotly {
        for i in 0..n_facets {
            let is_bottom = Self::is_bottom_row(i, ncols, nrows, n_facets);
            let is_left = Self::is_left_column(i, ncols);

            let x_title_for_subplot = if is_bottom { x_title.clone() } else { None };
            let y_title_for_subplot = if is_left { y_title.clone() } else { None };

            if x_title_for_subplot.is_some() || x_axis_config.is_some() {
                let axis = match x_axis_config {
                    Some(config) => Axis::set_axis(x_title_for_subplot, config, None),
                    None => {
                        if let Some(title) = x_title_for_subplot {
                            Axis::set_axis(Some(title), &Axis::default(), None)
                        } else {
                            continue;
                        }
                    }
                };

                layout = match i {
                    0 => layout.x_axis(axis),
                    1 => layout.x_axis2(axis),
                    2 => layout.x_axis3(axis),
                    3 => layout.x_axis4(axis),
                    4 => layout.x_axis5(axis),
                    5 => layout.x_axis6(axis),
                    6 => layout.x_axis7(axis),
                    7 => layout.x_axis8(axis),
                    _ => layout,
                };
            }

            if y_title_for_subplot.is_some() || y_axis_config.is_some() {
                let axis = match y_axis_config {
                    Some(config) => Axis::set_axis(y_title_for_subplot, config, None),
                    None => {
                        if let Some(title) = y_title_for_subplot {
                            Axis::set_axis(Some(title), &Axis::default(), None)
                        } else {
                            continue;
                        }
                    }
                };

                layout = match i {
                    0 => layout.y_axis(axis),
                    1 => layout.y_axis2(axis),
                    2 => layout.y_axis3(axis),
                    3 => layout.y_axis4(axis),
                    4 => layout.y_axis5(axis),
                    5 => layout.y_axis6(axis),
                    6 => layout.y_axis7(axis),
                    7 => layout.y_axis8(axis),
                    _ => layout,
                };
            }
        }

        layout
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
