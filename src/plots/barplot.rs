use bon::bon;

use plotly::{
    common::{ErrorData, ErrorType, Marker as MarkerPlotly},
    layout::{BarMode, GridPattern, LayoutGrid},
    Bar, Layout as LayoutPlotly, Trace,
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, Marker, PlotHelper, Polar},
    components::{Axis, FacetConfig, FacetScales, Legend, Orientation, Rgb, Text, DEFAULT_PLOTLY_COLORS},
};

/// A structure representing a bar plot.
///
/// The `BarPlot` struct allows for the creation and customization of bar plots with various options
/// for data, layout, and aesthetics. It supports both vertical and horizontal orientations, grouping
/// of data, error bars, and customizable markers and colors.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `labels` - A string slice specifying the column name to be used for the x-axis (independent variable).
/// * `values` - A string slice specifying the column name to be used for the y-axis (dependent variable).
/// * `orientation` - An optional `Orientation` enum specifying whether the plot should be horizontal or vertical.
/// * `group` - An optional string slice specifying the column name to be used for grouping data points.
/// * `sort_groups_by` - Optional comparator `fn(&str, &str) -> std::cmp::Ordering` to control group ordering.
///   Groups are sorted lexically by default.
/// * `error` - An optional string slice specifying the column name containing error values for the y-axis data.
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
/// use plotlars::{BarPlot, Legend, Orientation, Plot, Rgb, Text};
///
/// let dataset = df![
///         "animal" => &["giraffe", "giraffe", "orangutan", "orangutan", "monkey", "monkey"],
///         "gender" => &vec!["female", "male", "female", "male", "female", "male"],
///         "value" => &vec![20.0f32, 25.0, 14.0, 18.0, 23.0, 31.0],
///         "error" => &vec![1.0, 0.5, 1.5, 1.0, 0.5, 1.5],
///     ]
///     .unwrap();
///
/// BarPlot::builder()
///     .data(&dataset)
///     .labels("animal")
///     .values("value")
///     .orientation(Orientation::Vertical)
///     .group("gender")
///     .sort_groups_by(|a, b| a.len().cmp(&b.len()))
///     .error("error")
///     .colors(vec![
///         Rgb(255, 127, 80),
///         Rgb(64, 224, 208),
///     ])
///     .plot_title(
///         Text::from("Bar Plot")
///             .font("Arial")
///             .size(18)
///     )
///     .x_title(
///         Text::from("animal")
///             .font("Arial")
///             .size(15)
///     )
///     .y_title(
///         Text::from("value")
///             .font("Arial")
///             .size(15)
///     )
///     .legend_title(
///         Text::from("gender")
///             .font("Arial")
///             .size(15)
///     )
///     .legend(
///         &Legend::new()
///             .orientation(Orientation::Horizontal)
///             .y(1.0)
///             .x(0.4)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/xKHucCp.png)
#[derive(Clone, Serialize)]
pub struct BarPlot {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl BarPlot {
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
        error: Option<&str>,
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
        let y2_title = None;
        let z_title = None;
        let y2_axis = None;
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
                    labels,
                    values,
                    orientation,
                    group,
                    sort_groups_by,
                    facet_column,
                    &config,
                    error,
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

                layout = layout.bar_mode(BarMode::Group);

                let traces = Self::create_traces(
                    data,
                    labels,
                    values,
                    orientation,
                    group,
                    sort_groups_by,
                    error,
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
        error: Option<&str>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

        let opacity = None;
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
                        error,
                        marker,
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

                let trace =
                    Self::create_trace(data, labels, values, orientation, group, error, marker);

                traces.push(trace);
            }
        }

        traces
    }

    fn create_trace(
        data: &DataFrame,
        labels: &str,
        values: &str,
        orientation: Option<Orientation>,
        group: Option<&str>,
        error: Option<&str>,
        marker: MarkerPlotly,
    ) -> Box<dyn Trace + 'static> {
        let values = Self::get_numeric_column(data, values);
        let labels = Self::get_string_column(data, labels);

        let orientation = orientation.unwrap_or(Orientation::Vertical);

        match orientation {
            Orientation::Vertical => {
                let mut trace = Bar::default()
                    .x(labels)
                    .y(values)
                    .orientation(orientation.to_plotly());

                if let Some(error) = error {
                    let error = Self::get_numeric_column(data, error)
                        .iter()
                        .map(|x| x.unwrap() as f64)
                        .collect::<Vec<_>>();

                    trace = trace.error_y(ErrorData::new(ErrorType::Data).array(error))
                }

                trace = trace.marker(marker);

                if let Some(group) = group {
                    trace = trace.name(group);
                }

                trace
            }
            Orientation::Horizontal => {
                let mut trace = Bar::default()
                    .x(values)
                    .y(labels)
                    .orientation(orientation.to_plotly());

                if let Some(error) = error {
                    let error = Self::get_numeric_column(data, error)
                        .iter()
                        .map(|x| x.unwrap() as f64)
                        .collect::<Vec<_>>();

                    trace = trace.error_x(ErrorData::new(ErrorType::Data).array(error))
                }

                trace = trace.marker(marker);

                if let Some(group) = group {
                    trace = trace.name(group);
                }

                trace
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn build_bar_trace_with_axes(
        data: &DataFrame,
        labels: &str,
        values: &str,
        orientation: Option<Orientation>,
        group_name: Option<&str>,
        error: Option<&str>,
        marker: MarkerPlotly,
        x_axis: Option<&str>,
        y_axis: Option<&str>,
        show_legend: bool,
    ) -> Box<dyn Trace + 'static> {
        let values = Self::get_numeric_column(data, values);
        let labels = Self::get_string_column(data, labels);

        let orientation = orientation.unwrap_or(Orientation::Vertical);

        match orientation {
            Orientation::Vertical => {
                let mut trace = Bar::default()
                    .x(labels)
                    .y(values)
                    .orientation(orientation.to_plotly());

                if let Some(error) = error {
                    let error_data = Self::get_numeric_column(data, error)
                        .iter()
                        .map(|x| x.unwrap() as f64)
                        .collect::<Vec<_>>();
                    trace = trace.error_y(ErrorData::new(ErrorType::Data).array(error_data));
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
                let mut trace = Bar::default()
                    .x(values)
                    .y(labels)
                    .orientation(orientation.to_plotly());

                if let Some(error) = error {
                    let error_data = Self::get_numeric_column(data, error)
                        .iter()
                        .map(|x| x.unwrap() as f64)
                        .collect::<Vec<_>>();
                    trace = trace.error_x(ErrorData::new(ErrorType::Data).array(error_data));
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
        error: Option<&str>,
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

        let global_group_indices: std::collections::HashMap<String, usize> = if let Some(group_col) = group {
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
        let opacity = None;
        let size = None;
        let shape = None;
        let shapes = None;

        for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
            let facet_data = Self::filter_data_by_group(data, facet_column, facet_value);

            let x_axis = Self::get_axis_reference(facet_idx, "x");
            let y_axis = Self::get_axis_reference(facet_idx, "y");

            match group {
                Some(group_col) => {
                    let groups = Self::get_unique_groups(&facet_data, group_col, sort_groups_by);

                    for group_val in groups.iter() {
                        let group_data =
                            Self::filter_data_by_group(&facet_data, group_col, group_val);

                        let global_idx = global_group_indices
                            .get(group_val)
                            .copied()
                            .unwrap_or(0);

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

                        let trace = Self::build_bar_trace_with_axes(
                            &group_data,
                            labels,
                            values,
                            orientation.clone(),
                            Some(group_val),
                            error,
                            marker,
                            Some(&x_axis),
                            Some(&y_axis),
                            show_legend,
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

                    let trace = Self::build_bar_trace_with_axes(
                        &facet_data,
                        labels,
                        values,
                        orientation.clone(),
                        None,
                        error,
                        marker,
                        Some(&x_axis),
                        Some(&y_axis),
                        false,
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

        let mut layout = LayoutPlotly::new().grid(grid).bar_mode(BarMode::Group);

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

impl Layout for BarPlot {}
impl Marker for BarPlot {}
impl Polar for BarPlot {}

impl PlotHelper for BarPlot {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
