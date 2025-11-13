use bon::bon;
use indexmap::IndexSet;
use ordered_float::OrderedFloat;

use plotly::{surface::Position, Layout as LayoutPlotly, Surface, Trace};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper, Polar},
    components::{ColorBar, FacetConfig, FacetScales, Legend, Lighting, Palette, Text},
};

/// A structure representing a 3-D surface plot.
///
/// The `SurfacePlot` struct is designed to build and customize 3-dimensional
/// surface visualizations.  It supports fine-grained control over the color
/// mapping of *z* values, interactive color bars, lighting effects that enhance
/// depth perception, and global opacity settings.  Layout elements such as the
/// plot title and axis labels can also be configured through the builder API,
/// allowing you to embed the surface seamlessly in complex dashboards.
///
/// # Arguments
///
/// * `data` – A reference to the `DataFrame` that supplies the data.
///   It must contain three numeric columns whose names are given by `x`, `y`
///   and `z`.
/// * `x` – The column name to be used for the x-axis coordinates.
///   Each distinct *x* value becomes a row in the underlying *z* grid.
/// * `y` – The column name to be used for the y-axis coordinates.
///   Each distinct *y* value becomes a column in the *z* grid.
/// * `z` – The column name that provides the z-axis heights.  These values
///   are mapped to colors according to `color_scale` / `reverse_scale`.
/// * `color_bar` – An optional Reference to a `ColorBar` describing the
///   appearance of the color legend (length, tick formatting, border, etc.).
/// * `color_scale` – An optional `Palette` that defines the color gradient
///   (e.g. *Viridis*, *Cividis*).
/// * `reverse_scale` – An optional `bool` indicating whether the chosen
///   `color_scale` should run in the opposite direction.
/// * `show_scale` – An optional `bool` that toggles the visibility of the
///   color bar.  Useful when you have multiple surfaces that share an external
///   legend.
/// * `lighting` – An optional Reference to a `Lighting` struct that
///   specifies *ambient*, *diffuse*, *specular* components, *roughness*,
///   *fresnel* and light position.  Leaving it `None` applies Plotly's
///   default Phong shading.
/// * `opacity` – An optional `f64` in `[0.0, 1.0]` that sets the global
///   transparency of the surface (1 = opaque, 0 = fully transparent).
/// * `facet` – An optional string slice specifying the column name to create faceted subplots (one surface per category).
/// * `facet_config` – An optional reference to a `FacetConfig` struct for customizing facet layout (ncol, nrow, gap sizes, etc.).
/// * `plot_title` – An optional `Text` that customizes the title (content,
///   font, size, alignment).
/// * `legend` – An optional reference to a `Legend` struct for legend customization.
///
/// # Example
///
/// ```rust
/// use ndarray::Array;
/// use plotlars::{ColorBar, Lighting, Palette, Plot, SurfacePlot, Text};
/// use polars::prelude::*;
/// use std::iter;
///
/// let n: usize = 100;
/// let (x_base, _): (Vec<f64>, Option<usize>) = Array::linspace(-10.0, 10.0, n).into_raw_vec_and_offset();
/// let (y_base, _): (Vec<f64>, Option<usize>) = Array::linspace(-10.0, 10.0, n).into_raw_vec_and_offset();
///
/// let x = x_base
///     .iter()
///     .flat_map(|&xi| iter::repeat_n(xi, n))
///     .collect::<Vec<_>>();
///
/// let y = y_base
///     .iter()
///     .cycle()
///     .take(n * n)
///     .cloned()
///     .collect::<Vec<_>>();
///
/// let z = x_base
///     .iter()
///     .flat_map(|i| {
///         y_base
///             .iter()
///             .map(|j| 1.0 / (j * j + 5.0) * j.sin() + 1.0 / (i * i + 5.0) * i.cos())
///             .collect::<Vec<_>>()
///     })
///     .collect::<Vec<_>>();
///
/// let dataset = df![
///         "x" => &x,
///         "y" => &y,
///         "z" => &z,
///     ]
///     .unwrap();
///
/// SurfacePlot::builder()
///     .data(&dataset)
///     .x("x")
///     .y("y")
///     .z("z")
///     .plot_title(
///         Text::from("Surface Plot")
///             .font("Arial")
///             .size(18),
///     )
///     .color_bar(
///         &ColorBar::new()
///             .border_width(1),
///     )
///     .color_scale(Palette::Cividis)
///     .reverse_scale(true)
///     .opacity(0.5)
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/tdVte4l.png)
#[derive(Clone)]
pub struct SurfacePlot {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
    layout_json: Option<serde_json::Value>,
}

impl Serialize for SurfacePlot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("SurfacePlot", 2)?;
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
impl SurfacePlot {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        reverse_scale: Option<bool>,
        show_scale: Option<bool>,
        lighting: Option<&Lighting>,
        opacity: Option<f64>,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
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
                    facet_column,
                    &config,
                    color_bar,
                    color_scale,
                    reverse_scale,
                    show_scale,
                    lighting,
                    opacity,
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
                    color_bar,
                    color_scale,
                    reverse_scale,
                    show_scale,
                    lighting,
                    opacity,
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
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        reverse_scale: Option<bool>,
        show_scale: Option<bool>,
        lighting: Option<&Lighting>,
        opacity: Option<f64>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

        let trace = Self::create_trace(
            data,
            x,
            y,
            z,
            color_bar,
            color_scale,
            reverse_scale,
            show_scale,
            lighting,
            opacity,
            None,
        );

        traces.push(trace);
        traces
    }

    #[allow(clippy::too_many_arguments)]
    fn create_trace(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        reverse_scale: Option<bool>,
        show_scale: Option<bool>,
        lighting: Option<&Lighting>,
        opacity: Option<f64>,
        scene: Option<&str>,
    ) -> Box<dyn Trace + 'static> {
        let x = Self::get_numeric_column(data, x);
        let y = Self::get_numeric_column(data, y);
        let z = Self::get_numeric_column(data, z);

        let x = Self::unique_ordered(x);
        let y = Self::unique_ordered(y);
        let z = z
            .into_iter()
            .collect::<Vec<_>>()
            .chunks(y.len())
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<Option<f32>>>>();

        let mut trace = Surface::new(z).x(x).y(y);

        trace = Self::set_color_bar(trace, color_bar);
        trace = Self::set_color_scale(trace, color_scale);
        trace = Self::set_light_position(trace, lighting);
        trace = trace.lighting(Lighting::set_lighting(lighting));
        trace = Self::set_opacity(trace, opacity);
        trace = Self::set_reverse_scale(trace, reverse_scale);
        trace = Self::set_show_scale(trace, show_scale);

        if let Some(scene_ref) = scene {
            Box::new(SurfaceWithScene {
                inner: trace,
                scene: scene_ref.to_string(),
            })
        } else {
            trace
        }
    }

    fn set_show_scale<X, Y, Z>(
        mut trace: Box<Surface<X, Y, Z>>,
        show_scale: Option<bool>,
    ) -> Box<Surface<X, Y, Z>>
    where
        X: Serialize + Clone,
        Y: Serialize + Clone,
        Z: Serialize + Clone,
    {
        if let Some(show_scale) = show_scale {
            trace = trace.show_scale(show_scale);
        }

        trace
    }

    fn set_reverse_scale<X, Y, Z>(
        mut trace: Box<Surface<X, Y, Z>>,
        reverse_scale: Option<bool>,
    ) -> Box<Surface<X, Y, Z>>
    where
        X: Serialize + Clone,
        Y: Serialize + Clone,
        Z: Serialize + Clone,
    {
        if let Some(reverse_scale) = reverse_scale {
            trace = trace.reverse_scale(reverse_scale);
        }

        trace
    }

    fn set_opacity<X, Y, Z>(
        mut trace: Box<Surface<X, Y, Z>>,
        opacity: Option<f64>,
    ) -> Box<Surface<X, Y, Z>>
    where
        X: Serialize + Clone,
        Y: Serialize + Clone,
        Z: Serialize + Clone,
    {
        if let Some(opacity) = opacity {
            trace = trace.opacity(opacity);
        }

        trace
    }

    fn set_light_position<X, Y, Z>(
        mut trace: Box<Surface<X, Y, Z>>,
        lighting: Option<&Lighting>,
    ) -> Box<Surface<X, Y, Z>>
    where
        X: Serialize + Clone,
        Y: Serialize + Clone,
        Z: Serialize + Clone,
    {
        if let Some(light) = lighting {
            if let Some(position) = light.position {
                let position = Position::new(position[0], position[1], position[2]);

                trace = trace.light_position(position);
            }
        }

        trace
    }

    fn set_color_scale<X, Y, Z>(
        mut trace: Box<Surface<X, Y, Z>>,
        color_scale: Option<Palette>,
    ) -> Box<Surface<X, Y, Z>>
    where
        X: Serialize + Clone,
        Y: Serialize + Clone,
        Z: Serialize + Clone,
    {
        if let Some(color_scale) = color_scale {
            trace = trace.color_scale(color_scale.to_plotly());
        }

        trace
    }

    fn set_color_bar<X, Y, Z>(
        mut trace: Box<Surface<X, Y, Z>>,
        color_bar: Option<&ColorBar>,
    ) -> Box<Surface<X, Y, Z>>
    where
        X: Serialize + Clone,
        Y: Serialize + Clone,
        Z: Serialize + Clone,
    {
        if let Some(color_bar) = color_bar {
            trace = trace.color_bar(color_bar.to_plotly())
        }

        trace
    }

    fn unique_ordered(v: Vec<Option<f32>>) -> Vec<f32> {
        IndexSet::<OrderedFloat<f32>>::from_iter(v.into_iter().flatten().map(OrderedFloat))
            .into_iter()
            .map(|of| of.into_inner())
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    fn create_faceted_traces(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        facet_column: &str,
        config: &FacetConfig,
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        reverse_scale: Option<bool>,
        show_scale: Option<bool>,
        lighting: Option<&Lighting>,
        opacity: Option<f64>,
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

        let mut all_traces = Vec::new();

        for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
            let facet_data = Self::filter_data_by_group(data, facet_column, facet_value);
            let scene = Self::get_scene_reference(facet_idx);

            let trace = Self::create_trace(
                &facet_data,
                x,
                y,
                z,
                color_bar,
                color_scale,
                reverse_scale,
                show_scale,
                lighting,
                opacity,
                Some(&scene),
            );

            all_traces.push(trace);
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

        (layout, grid)
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

#[derive(Clone)]
struct SurfaceWithScene<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    inner: Box<Surface<X, Y, Z>>,
    scene: String,
}

impl<X, Y, Z> Serialize for SurfaceWithScene<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut value: serde_json::Value =
            serde_json::from_str(&self.inner.to_json()).map_err(serde::ser::Error::custom)?;
        value["scene"] = serde_json::Value::String(self.scene.clone());
        value.serialize(serializer)
    }
}

impl<X, Y, Z> Trace for SurfaceWithScene<X, Y, Z>
where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
    Z: Serialize + Clone + 'static,
{
    fn to_json(&self) -> String {
        let mut value: serde_json::Value = serde_json::from_str(&self.inner.to_json()).unwrap();
        value["scene"] = serde_json::Value::String(self.scene.clone());
        serde_json::to_string(&value).unwrap()
    }
}

impl Layout for SurfacePlot {}
impl Polar for SurfacePlot {}

impl PlotHelper for SurfacePlot {
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
