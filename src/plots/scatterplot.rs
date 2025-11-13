use bon::bon;

use plotly::{
    common::{Marker as MarkerPlotly, Mode},
    layout::{GridPattern, LayoutGrid},
    Layout as LayoutPlotly, Scatter, Trace,
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, Marker, PlotHelper, Polar},
    components::{Axis, FacetConfig, FacetScales, Legend, Rgb, Shape, Text, DEFAULT_PLOTLY_COLORS},
};

/// A structure representing a scatter plot.
///
/// The `ScatterPlot` struct facilitates the creation and customization of scatter plots with various options
/// for data selection, grouping, layout configuration, and aesthetic adjustments. It supports grouping of data,
/// customization of marker shapes, colors, sizes, opacity settings, and comprehensive layout customization
/// including titles, axes, and legends.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `x` - A string slice specifying the column name to be used for the x-axis (independent variable).
/// * `y` - A string slice specifying the column name to be used for the y-axis (dependent variable).
/// * `group` - An optional string slice specifying the column name to be used for grouping data points.
/// * `sort_groups_by` - Optional comparator `fn(&str, &str) -> std::cmp::Ordering` to control group ordering. Groups are sorted lexically by default.
/// * `facet` - An optional string slice specifying the column name to be used for faceting (creating multiple subplots).
/// * `facet_config` - An optional reference to a `FacetConfig` struct for customizing facet behavior (grid dimensions, scales, gaps, etc.).
/// * `opacity` - An optional `f64` value specifying the opacity of the plot markers (range: 0.0 to 1.0).
/// * `size` - An optional `usize` specifying the size of the markers.
/// * `color` - An optional `Rgb` value specifying the color of the markers. This is used when `group` is not specified.
/// * `colors` - An optional vector of `Rgb` values specifying the colors for the markers. This is used when `group` is specified to differentiate between groups.
/// * `shape` - An optional `Shape` specifying the shape of the markers. This is used when `group` is not specified.
/// * `shapes` - An optional vector of `Shape` values specifying multiple shapes for the markers when plotting multiple groups.
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
/// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
/// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
/// * `legend_title` - An optional `Text` struct specifying the title of the legend.
/// * `x_axis` - An optional reference to an `Axis` struct for customizing the x-axis.
/// * `y_axis` - An optional reference to an `Axis` struct for customizing the y-axis.
/// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot (e.g., positioning, font, etc.).
///
/// # Example
///
/// ```rust
/// use plotlars::{Axis, Legend, Plot, Rgb, ScatterPlot, Shape, Text, TickDirection};
/// use polars::prelude::*;
///
/// let dataset = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
///     .finish()
///     .unwrap()
///     .select([
///         col("species"),
///         col("sex").alias("gender"),
///         col("flipper_length_mm").cast(DataType::Int16),
///         col("body_mass_g").cast(DataType::Int16),
///     ])
///     .collect()
///     .unwrap();
///
/// let axis = Axis::new()
///     .show_line(true)
///     .tick_direction(TickDirection::OutSide)
///     .value_thousands(true);
///
/// ScatterPlot::builder()
///     .data(&dataset)
///     .x("body_mass_g")
///     .y("flipper_length_mm")
///     .group("species")
///     .sort_groups_by(|a, b| {
///         if a.len() == b.len() {
///             a.cmp(b)
///         } else {
///             a.len().cmp(&b.len())
///         }
///     })
///     .opacity(0.5)
///     .size(12)
///     .colors(vec![
///         Rgb(178, 34, 34),
///         Rgb(65, 105, 225),
///         Rgb(255, 140, 0),
///     ])
///     .shapes(vec![
///         Shape::Circle,
///         Shape::Square,
///         Shape::Diamond,
///     ])
///     .plot_title(
///         Text::from("Scatter Plot")
///             .font("Arial")
///             .size(20)
///             .x(0.065)
///     )
///     .x_title("body mass (g)")
///     .y_title("flipper length (mm)")
///     .legend_title("species")
///     .x_axis(
///         &axis.clone()
///             .value_range(vec![2500.0, 6500.0])
///     )
///     .y_axis(
///         &axis.clone()
///             .value_range(vec![170.0, 240.0])
///     )
///     .legend(
///         &Legend::new()
///             .x(0.85)
///             .y(0.15)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/9jfO8RU.png)
#[derive(Clone, Serialize)]
pub struct ScatterPlot {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl ScatterPlot {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: &str,
        y: &str,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        legend: Option<&Legend>,
    ) -> Self {
        let z_title = None;
        let z_axis = None;
        let y2_title = None;
        let y2_axis = None;

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
                    group,
                    sort_groups_by,
                    facet_column,
                    &config,
                    opacity,
                    size,
                    color,
                    colors,
                    shape,
                    shapes,
                );

                (layout, traces)
            }
            None => {
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
                );

                let traces = Self::create_traces(
                    data,
                    x,
                    y,
                    group,
                    sort_groups_by,
                    opacity,
                    size,
                    color,
                    colors,
                    shape,
                    shapes,
                );

                (layout, traces)
            }
        };

        Self { traces, layout }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
        data: &DataFrame,
        x: &str,
        y: &str,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

        match group {
            Some(group_col) => {
                let groups = Self::get_unique_groups(data, group_col, sort_groups_by);

                let groups = groups.iter().map(|s| s.as_str());

                for (i, group) in groups.enumerate() {
                    let marker = Self::create_marker(
                        i,
                        opacity,
                        size,
                        color,
                        colors.clone(),
                        shape,
                        shapes.clone(),
                    );

                    let subset = Self::filter_data_by_group(data, group_col, group);

                    let trace = Self::create_trace(&subset, x, y, Some(group), marker);

                    traces.push(trace);
                }
            }
            None => {
                let group = None;

                let marker = Self::create_marker(
                    0,
                    opacity,
                    size,
                    color,
                    colors.clone(),
                    shape,
                    shapes.clone(),
                );

                let trace = Self::create_trace(data, x, y, group, marker);

                traces.push(trace);
            }
        }

        traces
    }

    fn create_trace(
        data: &DataFrame,
        x: &str,
        y: &str,
        group_name: Option<&str>,
        marker: MarkerPlotly,
    ) -> Box<dyn Trace + 'static> {
        Self::build_scatter_trace(data, x, y, group_name, marker)
    }

    fn build_scatter_trace(
        data: &DataFrame,
        x: &str,
        y: &str,
        group_name: Option<&str>,
        marker: MarkerPlotly,
    ) -> Box<dyn Trace + 'static> {
        Self::build_scatter_trace_with_axes(data, x, y, group_name, marker, None, None, true, None)
    }

    #[allow(clippy::too_many_arguments)]
    fn build_scatter_trace_with_axes(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        group_name: Option<&str>,
        marker: MarkerPlotly,
        x_axis: Option<&str>,
        y_axis: Option<&str>,
        show_legend: bool,
        legend_group: Option<&str>,
    ) -> Box<dyn Trace + 'static> {
        let x = Self::get_numeric_column(data, x_col);
        let y = Self::get_numeric_column(data, y_col);

        let trace = Scatter::default().x(x).y(y).mode(Mode::Markers);

        let trace = trace.marker(marker);

        let trace = if let Some(name) = group_name {
            trace.name(name)
        } else {
            trace
        };

        let trace = if let Some(axis) = x_axis {
            trace.x_axis(axis)
        } else {
            trace
        };

        let trace = if let Some(axis) = y_axis {
            trace.y_axis(axis)
        } else {
            trace
        };

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
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet_column: &str,
        config: &FacetConfig,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
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

        if let Some(ref color_vec) = colors {
            if group.is_none() {
                let color_count = color_vec.len();
                let facet_count = facet_categories.len();

                if color_count != facet_count {
                    panic!(
                        "When using colors with facet (without group), colors.len() must equal number of facets. \
                         Expected {} colors for {} facets, but got {} colors. \
                         Each facet must be assigned exactly one color.",
                        facet_count, facet_count, color_count
                    );
                }
            } else if let Some(group_col) = group {
                let groups = Self::get_unique_groups(data, group_col, sort_groups_by);
                let color_count = color_vec.len();
                let group_count = groups.len();

                if color_count < group_count {
                    panic!(
                        "When using colors with group, colors.len() must be >= number of groups. \
                         Need at least {} colors for {} groups, but got {} colors",
                        group_count, group_count, color_count
                    );
                }
            }
        }

        let global_group_indices: std::collections::HashMap<String, usize> =
            if let Some(group_col) = group {
                let global_groups = Self::get_unique_groups(data, group_col, sort_groups_by);
                global_groups
                    .into_iter()
                    .enumerate()
                    .map(|(idx, group_name)| (group_name, idx))
                    .collect()
            } else {
                std::collections::HashMap::new()
            };

        let colors = if group.is_some() && colors.is_none() {
            Some(DEFAULT_PLOTLY_COLORS.to_vec())
        } else {
            colors
        };

        let mut all_traces = Vec::new();

        if config.highlight_facet {
            for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
                let x_axis = Self::get_axis_reference(facet_idx, "x");
                let y_axis = Self::get_axis_reference(facet_idx, "y");

                for other_facet_value in facet_categories.iter() {
                    if other_facet_value != facet_value {
                        let other_data =
                            Self::filter_data_by_group(data, facet_column, other_facet_value);

                        let grey_color = config.unhighlighted_color.unwrap_or(Rgb(200, 200, 200));
                        let grey_marker = Self::create_marker(
                            0,
                            opacity,
                            size,
                            Some(grey_color),
                            None,
                            shape,
                            None,
                        );

                        let trace = Self::build_scatter_trace_with_axes(
                            &other_data,
                            x,
                            y,
                            None,
                            grey_marker,
                            Some(&x_axis),
                            Some(&y_axis),
                            false,
                            None,
                        );

                        all_traces.push(trace);
                    }
                }

                let facet_data = Self::filter_data_by_group(data, facet_column, facet_value);

                match group {
                    Some(group_col) => {
                        let groups =
                            Self::get_unique_groups(&facet_data, group_col, sort_groups_by);

                        for group_val in groups.iter() {
                            let group_data =
                                Self::filter_data_by_group(&facet_data, group_col, group_val);

                            let global_idx =
                                global_group_indices.get(group_val).copied().unwrap_or(0);

                            let marker = Self::create_marker(
                                global_idx,
                                opacity,
                                size,
                                color,
                                colors.clone(),
                                shape,
                                shapes.clone(),
                            );

                            let show_legend = facet_idx == 0;

                            let trace = Self::build_scatter_trace_with_axes(
                                &group_data,
                                x,
                                y,
                                Some(group_val),
                                marker,
                                Some(&x_axis),
                                Some(&y_axis),
                                show_legend,
                                Some(group_val),
                            );

                            all_traces.push(trace);
                        }
                    }
                    None => {
                        let marker = Self::create_marker(
                            facet_idx,
                            opacity,
                            size,
                            color,
                            colors.clone(),
                            shape,
                            shapes.clone(),
                        );

                        let trace = Self::build_scatter_trace_with_axes(
                            &facet_data,
                            x,
                            y,
                            None,
                            marker,
                            Some(&x_axis),
                            Some(&y_axis),
                            false,
                            None,
                        );

                        all_traces.push(trace);
                    }
                }
            }
        } else {
            for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
                let facet_data = Self::filter_data_by_group(data, facet_column, facet_value);

                let x_axis = Self::get_axis_reference(facet_idx, "x");
                let y_axis = Self::get_axis_reference(facet_idx, "y");

                match group {
                    Some(group_col) => {
                        let groups =
                            Self::get_unique_groups(&facet_data, group_col, sort_groups_by);

                        for group_val in groups.iter() {
                            let group_data =
                                Self::filter_data_by_group(&facet_data, group_col, group_val);

                            let global_idx =
                                global_group_indices.get(group_val).copied().unwrap_or(0);

                            let marker = Self::create_marker(
                                global_idx,
                                opacity,
                                size,
                                color,
                                colors.clone(),
                                shape,
                                shapes.clone(),
                            );

                            let show_legend = facet_idx == 0;

                            let trace = Self::build_scatter_trace_with_axes(
                                &group_data,
                                x,
                                y,
                                Some(group_val),
                                marker,
                                Some(&x_axis),
                                Some(&y_axis),
                                show_legend,
                                Some(group_val),
                            );

                            all_traces.push(trace);
                        }
                    }
                    None => {
                        let marker = Self::create_marker(
                            facet_idx,
                            opacity,
                            size,
                            color,
                            colors.clone(),
                            shape,
                            shapes.clone(),
                        );

                        let trace = Self::build_scatter_trace_with_axes(
                            &facet_data,
                            x,
                            y,
                            None,
                            marker,
                            Some(&x_axis),
                            Some(&y_axis),
                            false,
                            None,
                        );

                        all_traces.push(trace);
                    }
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

        let (ncols, nrows) = Self::calculate_grid_dimensions(n_facets, config.cols, config.rows);

        let mut grid = LayoutGrid::new()
            .rows(nrows)
            .columns(ncols)
            .pattern(GridPattern::Independent);

        if let Some(x_gap) = config.h_gap {
            grid = grid.x_gap(x_gap);
        }
        if let Some(y_gap) = config.v_gap {
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

impl Layout for ScatterPlot {}
impl Marker for ScatterPlot {}
impl Polar for ScatterPlot {}

impl PlotHelper for ScatterPlot {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
