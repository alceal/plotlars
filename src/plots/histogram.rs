use bon::bon;

use plotly::{
    common::Marker as MarkerPlotly,
    histogram::{Bins, HistFunc},
    layout::{BarMode, GridPattern, LayoutGrid},
    Histogram as HistogramPlotly, Layout as LayoutPlotly, Trace,
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, Marker, PlotHelper, Polar},
    components::{Axis, FacetConfig, FacetScales, Legend, Rgb, Text, DEFAULT_PLOTLY_COLORS},
};

/// A structure representing a histogram.
///
/// The `Histogram` struct facilitates the creation and customization of histograms with various options
/// for data selection, layout configuration, and aesthetic adjustments. It supports grouping of data,
/// opacity settings, and customizable markers and colors.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `x` - A string slice specifying the column name to be used for the x-axis (independent variable).
/// * `group` - An optional string slice specifying the column name to be used for grouping data points.
/// * `sort_groups_by` - Optional comparator `fn(&str, &str) -> std::cmp::Ordering` to control group ordering. Groups are sorted lexically by default.
/// * `facet` - An optional string slice specifying the column name to be used for faceting (creating multiple subplots).
/// * `facet_config` - An optional reference to a `FacetConfig` struct for customizing facet behavior (grid dimensions, scales, gaps, etc.).
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
/// use plotlars::{Axis, Histogram, Legend, Plot, Rgb, Text, TickDirection};
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
///     .show_grid(true)
///     .value_thousands(true)
///     .tick_direction(TickDirection::OutSide);
///
/// Histogram::builder()
///     .data(&dataset)
///     .x("body_mass_g")
///     .group("species")
///     .opacity(0.5)
///     .colors(vec![
///         Rgb(255, 165, 0),
///         Rgb(147, 112, 219),
///         Rgb(46, 139, 87),
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
///     .x_axis(&axis)
///     .y_axis(&axis)
///     .legend(
///         &Legend::new()
///             .x(0.9)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/w2oiuIo.png)
#[derive(Clone, Serialize)]
pub struct Histogram {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl Histogram {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: &str,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
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
                    x,
                    group,
                    sort_groups_by,
                    facet_column,
                    &config,
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

                layout = layout.bar_mode(BarMode::Overlay);

                let traces =
                    Self::create_traces(data, x, group, sort_groups_by, opacity, color, colors);

                (layout, traces)
            }
        };

        Self { traces, layout }
    }

    fn create_traces(
        data: &DataFrame,
        x: &str,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
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

                    let trace = Self::create_trace(&subset, x, Some(group), marker, None);

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

                let trace = Self::create_trace(data, x, group, marker, None);

                traces.push(trace);
            }
        }

        traces
    }

    fn create_trace(
        data: &DataFrame,
        x: &str,
        group_name: Option<&str>,
        marker: MarkerPlotly,
        bins: Option<Bins>,
    ) -> Box<dyn Trace + 'static> {
        let x = Self::get_numeric_column(data, x);

        let mut trace = HistogramPlotly::default().x(x).hist_func(HistFunc::Count);

        trace = trace.marker(marker);

        if let Some(bins) = bins {
            trace = trace.x_bins(bins);
        }

        if let Some(name) = group_name {
            trace = trace.name(name);
        }

        trace
    }

    #[allow(clippy::too_many_arguments)]
    fn build_histogram_trace_with_axes(
        data: &DataFrame,
        x: &str,
        group_name: Option<&str>,
        marker: MarkerPlotly,
        bins: Option<Bins>,
        x_axis: Option<&str>,
        y_axis: Option<&str>,
        show_legend: bool,
        legend_group: Option<&str>,
    ) -> Box<dyn Trace + 'static> {
        let x_data = Self::get_numeric_column(data, x);

        let mut trace = HistogramPlotly::default()
            .x(x_data)
            .hist_func(HistFunc::Count);

        trace = trace.marker(marker);

        if let Some(bins) = bins {
            trace = trace.x_bins(bins);
        }

        if let Some(name) = group_name {
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
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet_column: &str,
        config: &FacetConfig,
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

        let global_bins = Self::should_use_global_bins(&config.scales);
        let bins = if global_bins {
            Some(Self::calculate_global_bins(data, x))
        } else {
            None
        };

        let mut all_traces = Vec::new();
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

                        let global_idx = global_group_indices.get(group_val).copied().unwrap_or(0);

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

                        let trace = Self::build_histogram_trace_with_axes(
                            &group_data,
                            x,
                            Some(group_val),
                            marker,
                            bins.clone(),
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

                    let trace = Self::build_histogram_trace_with_axes(
                        &facet_data,
                        x,
                        None,
                        marker,
                        bins.clone(),
                        Some(&x_axis),
                        Some(&y_axis),
                        false,
                        None,
                    );

                    all_traces.push(trace);
                }
            }
        }

        all_traces
    }

    fn should_use_global_bins(scales: &FacetScales) -> bool {
        match scales {
            FacetScales::Fixed | FacetScales::FreeY => true,
            FacetScales::Free | FacetScales::FreeX => false,
        }
    }

    fn calculate_global_bins(data: &DataFrame, x: &str) -> Bins {
        let x_data = Self::get_numeric_column(data, x);

        let values: Vec<f32> = x_data.iter().filter_map(|v| *v).collect();

        if values.is_empty() {
            return Bins::new(0.0, 1.0, 0.1);
        }

        let min = values.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

        let n = values.len() as f32;
        let nbins = (n.sqrt().ceil() as usize).clamp(10, 100);

        let range = max - min;
        let bin_size = if range > 0.0 {
            range / nbins as f32
        } else {
            1.0
        };

        Bins::new(min as f64, max as f64, bin_size as f64)
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

        let mut layout = LayoutPlotly::new().grid(grid).bar_mode(BarMode::Overlay);

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

impl Layout for Histogram {}
impl Marker for Histogram {}
impl Polar for Histogram {}

impl PlotHelper for Histogram {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
