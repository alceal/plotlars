use bon::bon;

use plotly::{
    box_plot::BoxPoints, common::Marker as MarkerPlotly, layout::{BoxMode, GridPattern, LayoutGrid},
    BoxPlot as BoxPlotly, Layout as LayoutPlotly, Trace,
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, Marker, PlotHelper, Polar},
    components::{Axis, FacetConfig, FacetScales, Legend, Orientation, Rgb, Text},
};

/// A structure representing a box plot.
///
/// The `BoxPlot` struct facilitates the creation and customization of box plots with various options
/// for data selection, layout configuration, and aesthetic adjustments. It supports both horizontal
/// and vertical orientations, grouping of data, display of individual data points with jitter and offset,
/// opacity settings, and customizable markers and colors.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `labels` - A string slice specifying the column name to be used for the x-axis (independent variable).
/// * `values` - A string slice specifying the column name to be used for the y-axis (dependent variable).
/// * `orientation` - An optional `Orientation` enum specifying whether the plot should be horizontal or vertical.
/// * `group` - An optional string slice specifying the column name to be used for grouping data points.
/// * `sort_groups_by` - Optional comparator `fn(&str, &str) -> std::cmp::Ordering` to control group ordering. Groups are sorted lexically by default.
/// * `facet` - An optional string slice specifying the column name to be used for creating facets (small multiples).
/// * `facet_config` - An optional reference to a `FacetConfig` struct for customizing facet layout and behavior.
/// * `box_points` - An optional boolean indicating whether individual data points should be plotted along with the box plot.
/// * `point_offset` - An optional `f64` value specifying the horizontal offset for individual data points when `box_points` is enabled.
/// * `jitter` - An optional `f64` value indicating the amount of jitter (random noise) to apply to individual data points for visibility.
/// * `opacity` - An optional `f64` value specifying the opacity of the plot markers (range: 0.0 to 1.0).
/// * `color` - An optional `Rgb` value specifying the color of the markers to be used for the plot. This is used when `group` is not specified.
/// * `colors` - An optional vector of `Rgb` values specifying the colors to be used for the plot. This is used when `group` is specified to differentiate between groups.
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
/// use polars::prelude::*;
/// use plotlars::{Axis, BoxPlot, Legend, Orientation, Plot, Rgb, Text};
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
/// BoxPlot::builder()
///     .data(&dataset)
///     .labels("species")
///     .values("body_mass_g")
///     .orientation(Orientation::Vertical)
///     .group("gender")
///     .box_points(true)
///     .point_offset(-1.5)
///     .jitter(0.01)
///     .opacity(0.1)
///     .colors(vec![
///         Rgb(0, 191, 255),
///         Rgb(57, 255, 20),
///         Rgb(255, 105, 180),
///     ])
///     .plot_title(
///         Text::from("Box Plot")
///             .font("Arial")
///             .size(18)
///     )
///     .x_title(
///         Text::from("species")
///             .font("Arial")
///             .size(15)
///     )
///     .y_title(
///         Text::from("body mass (g)")
///             .font("Arial")
///             .size(15)
///     )
///     .legend_title(
///         Text::from("gender")
///             .font("Arial")
///             .size(15)
///     )
///     .y_axis(
///         &Axis::new()
///             .value_thousands(true)
///     )
///     .legend(
///         &Legend::new()
///             .border_width(1)
///             .x(0.9)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/uj1LY90.png)
#[derive(Clone, Serialize)]
pub struct BoxPlot {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl BoxPlot {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        labels: &str,
        values: &str,
        orientation: Option<Orientation>,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
        opacity: Option<f64>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
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
                    labels,
                    values,
                    orientation,
                    group,
                    sort_groups_by,
                    facet_column,
                    &config,
                    box_points,
                    point_offset,
                    jitter,
                    opacity,
                    color,
                    colors,
                );

                (layout, traces)
            }
            None => {
                let mut layout = Self::create_layout(
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

                layout = layout.box_mode(BoxMode::Group);

                let traces = Self::create_traces(
                    data,
                    labels,
                    values,
                    orientation,
                    group,
                    sort_groups_by,
                    box_points,
                    point_offset,
                    jitter,
                    opacity,
                    color,
                    colors,
                );

                (layout, traces)
            }
        };

        Self { traces, layout }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
        data: &DataFrame,
        labels: &str,
        values: &str,
        orientation: Option<Orientation>,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
        opacity: Option<f64>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

        let size = None;
        let shape = None;
        let shapes = None;

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

                    let trace = Self::create_trace(
                        &subset,
                        labels,
                        values,
                        orientation.clone(),
                        Some(group),
                        box_points,
                        point_offset,
                        jitter,
                        marker,
                        None,
                        None,
                        true,
                    );

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

                let trace = Self::create_trace(
                    data,
                    labels,
                    values,
                    orientation,
                    group,
                    box_points,
                    point_offset,
                    jitter,
                    marker,
                    None,
                    None,
                    true,
                );

                traces.push(trace);
            }
        }

        traces
    }

    #[allow(clippy::too_many_arguments)]
    fn create_trace(
        data: &DataFrame,
        labels: &str,
        values: &str,
        orientation: Option<Orientation>,
        group_name: Option<&str>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
        marker: MarkerPlotly,
        x_axis: Option<&str>,
        y_axis: Option<&str>,
        show_legend: bool,
    ) -> Box<dyn Trace + 'static> {
        let category_data = Self::get_string_column(data, labels);
        let value_data = Self::get_numeric_column(data, values);

        let orientation = orientation.unwrap_or(Orientation::Vertical);

        match orientation {
            Orientation::Vertical => {
                let mut trace = BoxPlotly::default()
                    .x(category_data)
                    .y(value_data)
                    .orientation(orientation.to_plotly());

                if let Some(all) = box_points {
                    if all {
                        trace = trace.box_points(BoxPoints::All);
                    } else {
                        trace = trace.box_points(BoxPoints::False);
                    }
                }

                if let Some(point_position) = point_offset {
                    trace = trace.point_pos(point_position);
                }

                if let Some(jitter) = jitter {
                    trace = trace.jitter(jitter);
                }

                trace = trace.marker(marker);

                if let Some(name) = group_name {
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
            Orientation::Horizontal => {
                let mut trace = BoxPlotly::default()
                    .x(value_data)
                    .y(category_data)
                    .orientation(orientation.to_plotly());

                if let Some(all) = box_points {
                    if all {
                        trace = trace.box_points(BoxPoints::All);
                    } else {
                        trace = trace.box_points(BoxPoints::False);
                    }
                }

                if let Some(point_position) = point_offset {
                    trace = trace.point_pos(point_position);
                }

                if let Some(jitter) = jitter {
                    trace = trace.jitter(jitter);
                }

                trace = trace.marker(marker);

                if let Some(name) = group_name {
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
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_faceted_traces(
        data: &DataFrame,
        labels: &str,
        values: &str,
        orientation: Option<Orientation>,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet_column: &str,
        config: &FacetConfig,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
        opacity: Option<f64>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
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

        // Validate color scenarios
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

        let mut all_traces = Vec::new();
        let size = None;
        let shape = None;
        let shapes = None;

        // Note: BoxPlot does not support highlight_facet mode per spec
        // Statistics are calculated per-facet (each facet gets its own data subset)
        for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
            let facet_data = Self::filter_data_by_group(data, facet_column, facet_value);

            let x_axis = Self::get_axis_reference(facet_idx, "x");
            let y_axis = Self::get_axis_reference(facet_idx, "y");

            match group {
                Some(group_col) => {
                    let groups = Self::get_unique_groups(&facet_data, group_col, sort_groups_by);

                    for (group_idx, group_val) in groups.iter().enumerate() {
                        let group_data = Self::filter_data_by_group(&facet_data, group_col, group_val);

                        let marker = Self::create_marker(
                            group_idx,
                            opacity,
                            size,
                            color,
                            colors.clone(),
                            shape,
                            shapes.clone(),
                        );

                        // Show legend only for first facet
                        let show_legend = facet_idx == 0;

                        let trace = Self::create_trace(
                            &group_data,
                            labels,
                            values,
                            orientation.clone(),
                            Some(group_val),
                            box_points,
                            point_offset,
                            jitter,
                            marker,
                            Some(&x_axis),
                            Some(&y_axis),
                            show_legend,
                        );

                        all_traces.push(trace);
                    }
                }
                None => {
                    // Scenario 2: colors + facet without group
                    // Each facet gets its own color
                    let marker = Self::create_marker(
                        facet_idx,
                        opacity,
                        size,
                        color,
                        colors.clone(),
                        shape,
                        shapes.clone(),
                    );

                    let trace = Self::create_trace(
                        &facet_data,
                        labels,
                        values,
                        orientation.clone(),
                        None,
                        box_points,
                        point_offset,
                        jitter,
                        marker,
                        Some(&x_axis),
                        Some(&y_axis),
                        false, // No legend for facets without groups
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

        let mut layout = LayoutPlotly::new().grid(grid).box_mode(BoxMode::Group);

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

impl Layout for BoxPlot {}
impl Marker for BoxPlot {}
impl Polar for BoxPlot {}

impl PlotHelper for BoxPlot {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}