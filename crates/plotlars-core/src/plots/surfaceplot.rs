use bon::bon;
use indexmap::IndexSet;
use ordered_float::OrderedFloat;

use crate::{
    components::{ColorBar, FacetConfig, Legend, Lighting, Palette, Text},
    ir::data::ColumnData,
    ir::layout::LayoutIR,
    ir::trace::{SurfacePlotIR, TraceIR},
};
use polars::frame::DataFrame;

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
#[allow(dead_code)]
pub struct SurfacePlot {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
}

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
        let grid = facet.map(|facet_column| {
            let config = facet_config.cloned().unwrap_or_default();
            let facet_categories =
                crate::data::get_unique_groups(data, facet_column, config.sorter);
            let n_facets = facet_categories.len();
            let (ncols, nrows) =
                crate::faceting::calculate_grid_dimensions(n_facets, config.cols, config.rows);
            crate::ir::facet::GridSpec {
                kind: crate::ir::facet::FacetKind::Scene,
                rows: nrows,
                cols: ncols,
                h_gap: config.h_gap,
                v_gap: config.v_gap,
                scales: config.scales.clone(),
                n_facets,
                facet_categories,
                title_style: config.title_style.clone(),
                x_title: None,
                y_title: None,
                x_axis: None,
                y_axis: None,
                legend_title: None,
                legend: legend.cloned(),
            }
        });

        let traces = match facet {
            Some(facet_column) => {
                let config = facet_config.cloned().unwrap_or_default();
                Self::create_ir_traces_faceted(
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
                )
            }
            None => Self::create_ir_traces(
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
            ),
        };

        let layout = LayoutIR {
            title: plot_title,
            x_title: None,
            y_title: None,
            y2_title: None,
            z_title: None,
            legend_title: None,
            legend: if grid.is_some() {
                None
            } else {
                legend.cloned()
            },
            dimensions: None,
            bar_mode: None,
            box_mode: None,
            box_gap: None,
            margin_bottom: None,
            axes_2d: None,
            scene_3d: None,
            polar: None,
            mapbox: None,
            grid,
            annotations: vec![],
        };

        Self { traces, layout }
    }
    fn unique_ordered(v: Vec<Option<f32>>) -> Vec<f32> {
        IndexSet::<OrderedFloat<f32>>::from_iter(v.into_iter().flatten().map(OrderedFloat))
            .into_iter()
            .map(|of| of.into_inner())
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces(
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
    ) -> Vec<TraceIR> {
        let ir = Self::build_surface_ir(
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
        vec![TraceIR::SurfacePlot(ir)]
    }

    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces_faceted(
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
    ) -> Vec<TraceIR> {
        const MAX_FACETS: usize = 8;

        let facet_categories = crate::data::get_unique_groups(data, facet_column, config.sorter);

        if facet_categories.len() > MAX_FACETS {
            panic!(
                "Facet column '{}' has {} unique values, but plotly.rs supports maximum {} 3D scenes",
                facet_column,
                facet_categories.len(),
                MAX_FACETS
            );
        }

        let mut traces = Vec::new();

        for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
            let facet_data = crate::data::filter_data_by_group(data, facet_column, facet_value);
            let scene = Self::get_scene_reference(facet_idx);

            // Only show colorbar on the first faceted trace to avoid overlap
            let facet_show_scale = if facet_idx == 0 {
                show_scale
            } else {
                Some(false)
            };

            let ir = Self::build_surface_ir(
                &facet_data,
                x,
                y,
                z,
                if facet_idx == 0 { color_bar } else { None },
                color_scale,
                reverse_scale,
                facet_show_scale,
                lighting,
                opacity,
                Some(scene),
            );

            traces.push(TraceIR::SurfacePlot(ir));
        }

        traces
    }

    #[allow(clippy::too_many_arguments)]
    fn build_surface_ir(
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
        scene_ref: Option<String>,
    ) -> SurfacePlotIR {
        let x_raw = crate::data::get_numeric_column(data, x);
        let y_raw = crate::data::get_numeric_column(data, y);
        let z_raw = crate::data::get_numeric_column(data, z);

        let x_unique = Self::unique_ordered(x_raw);
        let y_unique = Self::unique_ordered(y_raw.clone());

        let z_grid: Vec<Vec<f64>> = z_raw
            .into_iter()
            .collect::<Vec<_>>()
            .chunks(y_unique.len())
            .map(|chunk| chunk.iter().map(|v| v.unwrap_or(0.0) as f64).collect())
            .collect();

        SurfacePlotIR {
            x: ColumnData::Numeric(x_unique.iter().map(|v| Some(*v)).collect()),
            y: ColumnData::Numeric(y_unique.iter().map(|v| Some(*v)).collect()),
            z: z_grid,
            color_scale,
            color_bar: color_bar.cloned(),
            reverse_scale,
            show_scale,
            lighting: lighting.cloned(),
            opacity,
            scene_ref,
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
}

impl crate::Plot for SurfacePlot {
    fn ir_traces(&self) -> &[TraceIR] {
        &self.traces
    }

    fn ir_layout(&self) -> &LayoutIR {
        &self.layout
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Plot;
    use polars::prelude::*;

    #[test]
    fn test_basic_one_trace() {
        let df = df![
            "x" => [1.0, 1.0, 2.0, 2.0],
            "y" => [1.0, 2.0, 1.0, 2.0],
            "z" => [5.0, 6.0, 7.0, 8.0]
        ]
        .unwrap();
        let plot = SurfacePlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .z("z")
            .build();
        assert_eq!(plot.ir_traces().len(), 1);
        assert!(matches!(plot.ir_traces()[0], TraceIR::SurfacePlot(_)));
    }

    #[test]
    fn test_layout_no_axes_2d() {
        let df = df![
            "x" => [1.0, 1.0, 2.0, 2.0],
            "y" => [1.0, 2.0, 1.0, 2.0],
            "z" => [5.0, 6.0, 7.0, 8.0]
        ]
        .unwrap();
        let plot = SurfacePlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .z("z")
            .build();
        assert!(plot.ir_layout().axes_2d.is_none());
    }

    #[test]
    fn test_layout_title() {
        let df = df![
            "x" => [1.0, 1.0, 2.0, 2.0],
            "y" => [1.0, 2.0, 1.0, 2.0],
            "z" => [5.0, 6.0, 7.0, 8.0]
        ]
        .unwrap();
        let plot = SurfacePlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .z("z")
            .plot_title("Surface")
            .build();
        assert!(plot.ir_layout().title.is_some());
    }
}
