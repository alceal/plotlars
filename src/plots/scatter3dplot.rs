use bon::bon;

use plotly::{
    common::{Marker as MarkerPlotly, Mode},
    layout::Margin,
    Layout as LayoutPlotly, Scatter3D, Trace,
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, Marker, PlotHelper, Polar},
    components::{FacetConfig, FacetScales, Legend, Rgb, Shape, Text, DEFAULT_PLOTLY_COLORS},
};

/// A structure representing a 3D scatter plot.
///
/// The `Scatter3dPlot` struct is designed to create and customize 3D scatter plots with options for data selection,
/// grouping, layout configuration, and aesthetic adjustments. It supports visual differentiation in data groups
/// through varied marker shapes, colors, sizes, opacity levels, and comprehensive layout customization, including
/// titles, axis labels, and legends.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `x` - A string slice specifying the column name to be used for the x-axis (independent variable).
/// * `y` - A string slice specifying the column name to be used for the y-axis (dependent variable).
/// * `z` - A string slice specifying the column name to be used for the z-axis, adding a third dimension to the scatter plot.
/// * `group` - An optional string slice specifying the column name used for grouping data points by category.
/// * `sort_groups_by` - Optional comparator `fn(&str, &str) -> std::cmp::Ordering` to control group ordering. Groups are sorted lexically by default.
/// * `opacity` - An optional `f64` value specifying the opacity of the plot markers (range: 0.0 to 1.0).
/// * `size` - An optional `usize` specifying the size of the markers.
/// * `color` - An optional `Rgb` value for marker color when `group` is not specified.
/// * `colors` - An optional vector of `Rgb` values specifying colors for markers when `group` is specified, enhancing group distinction.
/// * `shape` - An optional `Shape` specifying the shape of markers when `group` is not specified.
/// * `shapes` - An optional vector of `Shape` values defining multiple marker shapes for different groups.
/// * `plot_title` - An optional `Text` struct specifying the plot title.
/// * `x_title` - An optional `Text` struct for the x-axis title.
/// * `y_title` - An optional `Text` struct for the y-axis title.
/// * `z_title` - An optional `Text` struct for the z-axis title.
/// * `legend_title` - An optional `Text` struct specifying the legend title.
/// * `x_axis` - An optional reference to an `Axis` struct for custom x-axis settings.
/// * `y_axis` - An optional reference to an `Axis` struct for custom y-axis settings.
/// * `z_axis` - An optional reference to an `Axis` struct for custom z-axis settings, adding depth perspective.
/// * `legend` - An optional reference to a `Legend` struct for legend customization, including position and font settings.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Legend, Plot, Rgb, Scatter3dPlot, Shape};
///
/// let dataset = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
///     .finish()
///     .unwrap()
///     .select([
///         col("species"),
///         col("sex").alias("gender"),
///         col("bill_length_mm").cast(DataType::Float32),
///         col("flipper_length_mm").cast(DataType::Int16),
///         col("body_mass_g").cast(DataType::Int16),
///     ])
///     .collect()
///     .unwrap();
///
/// Scatter3dPlot::builder()
///     .data(&dataset)
///     .x("body_mass_g")
///     .y("flipper_length_mm")
///     .z("bill_length_mm")
///     .group("species")
///     .opacity(0.25)
///     .size(8)
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
///     .plot_title("Scatter Plot")
///     .legend(
///         &Legend::new()
///             .x(0.6)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/WYTQxHA.png)
#[derive(Clone)]
pub struct Scatter3dPlot {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
    layout_json: Option<serde_json::Value>,
}

impl Serialize for Scatter3dPlot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Scatter3dPlot", 2)?;
        state.serialize_field("traces", &self.traces)?;
        if let Some(ref layout_json) = self.layout_json {
            state.serialize_field("layout", layout_json)?;
        } else {
            state.serialize_field("layout", &self.layout)?;
        }
        state.end()
    }
}

#[derive(Clone)]
struct FacetGrid {
    ncols: usize,
    nrows: usize,
    x_gap: f64,
    y_gap: f64,
}

const SCENE_FACET_TITLE_HEIGHT_RATIO: f64 = 0.12;
const SCENE_FACET_TOP_INSET_RATIO: f64 = 0.08;

#[bon]
impl Scatter3dPlot {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
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
        legend: Option<&Legend>,
    ) -> Self {
        let legend_title = None;
        let x_title = None;
        let y_title = None;
        let z_title = None;
        let x_axis = None;
        let y_axis = None;
        let z_axis = None;
        let y2_title = None;
        let y2_axis = None;

        let (layout, traces, layout_json) = match facet {
            Some(facet_column) => {
                let config = facet_config.cloned().unwrap_or_default();

                let (layout, grid) = Self::create_faceted_layout(
                    data,
                    facet_column,
                    &config,
                    plot_title,
                    legend_title,
                    legend,
                );

                let traces = Self::create_faceted_traces(
                    data,
                    x,
                    y,
                    z,
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

                let mut layout_json = serde_json::to_value(&layout).unwrap();
                Self::inject_scene_domains_static(
                    &mut layout_json,
                    grid.ncols,
                    grid.nrows,
                    grid.x_gap,
                    grid.y_gap,
                    &config.scales,
                );

                (layout, traces, Some(layout_json))
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
                    z,
                    group,
                    sort_groups_by,
                    opacity,
                    size,
                    color,
                    colors,
                    shape,
                    shapes,
                );

                (layout, traces, None)
            }
        };

        Self {
            traces,
            layout,
            layout_json,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
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

                    let trace = Self::create_trace(&subset, x, y, z, Some(group), marker);

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

                let trace = Self::create_trace(data, x, y, z, group, marker);

                traces.push(trace);
            }
        }

        traces
    }

    fn create_trace(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        group_name: Option<&str>,
        marker: MarkerPlotly,
    ) -> Box<dyn Trace + 'static> {
        Self::build_scatter3d_trace(data, x, y, z, group_name, marker, None, true, None)
    }

    #[allow(clippy::too_many_arguments)]
    fn build_scatter3d_trace(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        group_name: Option<&str>,
        marker: MarkerPlotly,
        scene: Option<&str>,
        show_legend: bool,
        legend_group: Option<&str>,
    ) -> Box<dyn Trace + 'static> {
        let x = Self::get_numeric_column(data, x);
        let y = Self::get_numeric_column(data, y);
        let z = Self::get_numeric_column(data, z);

        let mut trace = Scatter3D::default().x(x).y(y).z(z).mode(Mode::Markers);
        trace = trace.marker(marker);

        if let Some(name) = group_name {
            trace = trace.name(name);
        }

        if let Some(scene_ref) = scene {
            trace = trace.scene(scene_ref);
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

    fn get_scene_reference(index: usize) -> String {
        match index {
            0 => "scene".to_string(),
            1 => "scene2".to_string(),
            2 => "scene3".to_string(),
            3 => "scene4".to_string(),
            4 => "scene5".to_string(),
            5 => "scene6".to_string(),
            6 => "scene7".to_string(),
            7 => "scene8".to_string(),
            _ => "scene".to_string(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_faceted_traces(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
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
                "Facet column '{}' has {} unique values, but plotly.rs supports maximum {} 3D scenes",
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
                         Expected {} colors for {} facets, but got {} colors.",
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
                let scene = Self::get_scene_reference(facet_idx);

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

                        let trace = Self::build_scatter3d_trace(
                            &other_data,
                            x,
                            y,
                            z,
                            None,
                            grey_marker,
                            Some(&scene),
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

                            let trace = Self::build_scatter3d_trace(
                                &group_data,
                                x,
                                y,
                                z,
                                Some(group_val),
                                marker,
                                Some(&scene),
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

                        let trace = Self::build_scatter3d_trace(
                            &facet_data,
                            x,
                            y,
                            z,
                            None,
                            marker,
                            Some(&scene),
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
                let scene = Self::get_scene_reference(facet_idx);

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

                            let trace = Self::build_scatter3d_trace(
                                &group_data,
                                x,
                                y,
                                z,
                                Some(group_val),
                                marker,
                                Some(&scene),
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

                        let trace = Self::build_scatter3d_trace(
                            &facet_data,
                            x,
                            y,
                            z,
                            None,
                            marker,
                            Some(&scene),
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

    fn create_faceted_layout(
        data: &DataFrame,
        facet_column: &str,
        config: &FacetConfig,
        plot_title: Option<Text>,
        legend_title: Option<Text>,
        legend: Option<&Legend>,
    ) -> (LayoutPlotly, FacetGrid) {
        let facet_categories = Self::get_unique_groups(data, facet_column, config.sorter);
        let n_facets = facet_categories.len();

        let (ncols, nrows) = Self::calculate_grid_dimensions(n_facets, config.ncol, config.nrow);

        let x_gap = config.x_gap.unwrap_or(0.08);
        let y_gap = config.y_gap.unwrap_or(0.12);

        let grid = FacetGrid {
            ncols,
            nrows,
            x_gap,
            y_gap,
        };

        let mut layout = LayoutPlotly::new();

        if let Some(title) = plot_title {
            layout = layout.title(title.to_plotly());
        }

        let annotations = Self::create_facet_annotations_scene(
            &facet_categories,
            ncols,
            nrows,
            config.title_style.as_ref(),
            config.x_gap,
            config.y_gap,
        );
        layout = layout.annotations(annotations);

        layout = layout.legend(Legend::set_legend(legend_title, legend));
        layout = layout.margin(Margin::new().top(140).bottom(80).left(80).right(80));

        (layout, grid)
    }

    fn calculate_scene_facet_cell(
        subplot_index: usize,
        ncols: usize,
        nrows: usize,
        x_gap: Option<f64>,
        y_gap: Option<f64>,
    ) -> SceneFacetCell {
        let row = subplot_index / ncols;
        let col = subplot_index % ncols;

        let x_gap_val = x_gap.unwrap_or(0.08);
        let y_gap_val = y_gap.unwrap_or(0.12);

        let cell_width = (1.0 - x_gap_val * (ncols - 1) as f64) / ncols as f64;
        let cell_height = (1.0 - y_gap_val * (nrows - 1) as f64) / nrows as f64;

        let title_height = cell_height * SCENE_FACET_TITLE_HEIGHT_RATIO;
        let scene_padding = cell_height * SCENE_FACET_TOP_INSET_RATIO;

        let cell_x_start = col as f64 * (cell_width + x_gap_val);
        let cell_y_top = 1.0 - row as f64 * (cell_height + y_gap_val);
        let cell_y_bottom = cell_y_top - cell_height;

        let domain_y_top = cell_y_top - title_height - scene_padding;
        let domain_y_bottom = cell_y_bottom;

        let annotation_x = cell_x_start + cell_width / 2.0;
        let annotation_y = cell_y_top - scene_padding * 0.5;

        SceneFacetCell {
            annotation_x,
            annotation_y,
            domain_x: [cell_x_start, cell_x_start + cell_width],
            domain_y: [domain_y_bottom, domain_y_top],
        }
    }

    fn create_facet_annotations_scene(
        categories: &[String],
        ncols: usize,
        nrows: usize,
        title_style: Option<&Text>,
        x_gap: Option<f64>,
        y_gap: Option<f64>,
    ) -> Vec<plotly::layout::Annotation> {
        use plotly::common::Anchor;
        use plotly::layout::Annotation;

        categories
            .iter()
            .enumerate()
            .map(|(i, cat)| {
                let cell = Self::calculate_scene_facet_cell(i, ncols, nrows, x_gap, y_gap);

                let mut ann = Annotation::new()
                    .text(cat.as_str())
                    .x_ref("paper")
                    .y_ref("paper")
                    .x_anchor(Anchor::Center)
                    .y_anchor(Anchor::Bottom)
                    .x(cell.annotation_x)
                    .y(cell.annotation_y)
                    .show_arrow(false);

                if let Some(style) = title_style {
                    ann = ann.font(style.to_font());
                }

                ann
            })
            .collect()
    }

    fn inject_scene_domains_static(
        layout_json: &mut serde_json::Value,
        ncols: usize,
        nrows: usize,
        x_gap: f64,
        y_gap: f64,
        scales: &FacetScales,
    ) {
        let total_cells = (ncols * nrows).clamp(1, 8);

        for i in 0..total_cells {
            let scene_key = if i == 0 {
                "scene".to_string()
            } else {
                format!("scene{}", i + 1)
            };

            let cell = Self::calculate_scene_facet_cell(i, ncols, nrows, Some(x_gap), Some(y_gap));

            let mut scene_config = serde_json::json!({
                "domain": {
                    "x": cell.domain_x,
                    "y": cell.domain_y
                }
            });

            if i > 0 {
                match scales {
                    FacetScales::Fixed => {
                        scene_config["xaxis"] = serde_json::json!({"matches": "x"});
                        scene_config["yaxis"] = serde_json::json!({"matches": "y"});
                        scene_config["zaxis"] = serde_json::json!({"matches": "z"});
                    }
                    FacetScales::FreeX => {
                        scene_config["yaxis"] = serde_json::json!({"matches": "y"});
                        scene_config["zaxis"] = serde_json::json!({"matches": "z"});
                    }
                    FacetScales::FreeY => {
                        scene_config["xaxis"] = serde_json::json!({"matches": "x"});
                        scene_config["zaxis"] = serde_json::json!({"matches": "z"});
                    }
                    FacetScales::Free => {}
                }
            }

            layout_json[scene_key] = scene_config;
        }
    }
}

struct SceneFacetCell {
    annotation_x: f64,
    annotation_y: f64,
    domain_x: [f64; 2],
    domain_y: [f64; 2],
}

impl Layout for Scatter3dPlot {}
impl Marker for Scatter3dPlot {}
impl Polar for Scatter3dPlot {}

impl PlotHelper for Scatter3dPlot {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }

    fn get_layout_override(&self) -> Option<&serde_json::Value> {
        self.layout_json.as_ref()
    }
}
