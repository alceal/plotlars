use bon::bon;

use crate::{
    components::{ColorBar, FacetConfig, IntensityMode, Legend, Lighting, Palette, Rgb, Text},
    ir::data::ColumnData,
    ir::layout::LayoutIR,
    ir::trace::{Mesh3DIR as Mesh3DIRStruct, TraceIR},
};
use polars::frame::DataFrame;

/// A structure representing a 3D mesh plot.
///
/// The `Mesh3D` struct is designed to create and customize 3D mesh visualizations
/// with support for explicit triangulation, intensity-based coloring, and various
/// lighting effects. It can handle both auto-triangulated point clouds and
/// explicitly defined mesh connectivity through triangle indices.
///
/// # Backend Support
///
/// | Backend | Supported |
/// |---------|-----------|
/// | Plotly  | Yes       |
/// | Plotters| --        |
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the mesh data.
/// * `x` - A string slice specifying the column name for x-axis vertex coordinates.
/// * `y` - A string slice specifying the column name for y-axis vertex coordinates.
/// * `z` - A string slice specifying the column name for z-axis vertex coordinates.
/// * `i` - An optional string slice specifying the column name for first vertex indices of triangles.
/// * `j` - An optional string slice specifying the column name for second vertex indices of triangles.
/// * `k` - An optional string slice specifying the column name for third vertex indices of triangles.
/// * `intensity` - An optional string slice specifying the column name for intensity values used in gradient coloring.
/// * `intensity_mode` - An optional `IntensityMode` specifying whether intensity applies to vertices or cells.
/// * `color` - An optional `Rgb` value for uniform mesh coloring.
/// * `color_bar` - An optional reference to a `ColorBar` for customizing the color legend.
/// * `color_scale` - An optional `Palette` defining the color gradient for intensity mapping.
/// * `reverse_scale` - An optional boolean to reverse the color scale direction.
/// * `show_scale` - An optional boolean to show/hide the color bar.
/// * `opacity` - An optional `f64` value specifying mesh transparency (range: 0.0 to 1.0).
/// * `flat_shading` - An optional boolean for angular (true) vs smooth (false) shading.
/// * `lighting` - An optional reference to a `Lighting` struct for custom lighting settings.
/// * `light_position` - An optional tuple `(x, y, z)` specifying the light source position.
/// * `delaunay_axis` - An optional string specifying the axis for Delaunay triangulation ("x", "y", or "z").
/// * `contour` - An optional boolean to enable contour lines on the mesh.
/// * `facet` - An optional string slice specifying the column name to be used for faceting (creating multiple subplots).
/// * `facet_config` - An optional reference to a `FacetConfig` struct for customizing facet behavior (grid dimensions, scales, gaps, etc.).
/// * `plot_title` - An optional `Text` struct specifying the plot title.
/// * `x_title` - An optional `Text` struct for the x-axis title.
/// * `y_title` - An optional `Text` struct for the y-axis title.
/// * `z_title` - An optional `Text` struct for the z-axis title.
/// * `legend` - An optional reference to a `Legend` struct for legend customization.
///
/// # Example
///
/// ```rust
/// use plotlars::{Lighting, Mesh3D, Plot, Rgb, Text};
/// use polars::prelude::*;
///
/// let mut x = Vec::new();
/// let mut y = Vec::new();
/// let mut z = Vec::new();
///
/// let n = 20;
/// for i in 0..n {
///     for j in 0..n {
///         let xi = (i as f64 / (n - 1) as f64) * 2.0 - 1.0;
///         let yj = (j as f64 / (n - 1) as f64) * 2.0 - 1.0;
///         x.push(xi);
///         y.push(yj);
///         z.push(0.3 * ((xi * 3.0).sin() + (yj * 3.0).cos()));
///     }
/// }
///
/// let dataset = DataFrame::new(x.len(), vec![
///     Column::new("x".into(), x),
///     Column::new("y".into(), y),
///     Column::new("z".into(), z),
/// ])
/// .unwrap();
///
/// Mesh3D::builder()
///     .data(&dataset)
///     .x("x")
///     .y("y")
///     .z("z")
///     .color(Rgb(200, 200, 255))
///     .lighting(
///         &Lighting::new()
///             .ambient(0.5)
///             .diffuse(0.8)
///             .specular(0.5)
///             .roughness(0.2)
///             .fresnel(0.2),
///     )
///     .light_position((1, 1, 2))
///     .opacity(1.0)
///     .flat_shading(false)
///     .contour(true)
///     .plot_title(
///         Text::from("Mesh 3D Plot")
///             .font("Arial")
///             .size(22),
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/bljzmw5.png)
#[derive(Clone)]
#[allow(dead_code)]
pub struct Mesh3D {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
}

#[bon]
impl Mesh3D {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        i: Option<&str>,
        j: Option<&str>,
        k: Option<&str>,
        intensity: Option<&str>,
        intensity_mode: Option<IntensityMode>,
        color: Option<Rgb>,
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        _reverse_scale: Option<bool>,
        _show_scale: Option<bool>,
        opacity: Option<f64>,
        flat_shading: Option<bool>,
        lighting: Option<&Lighting>,
        light_position: Option<(i32, i32, i32)>,
        delaunay_axis: Option<&str>,
        contour: Option<bool>,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        z_title: Option<Text>,
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
                    i,
                    j,
                    k,
                    intensity,
                    intensity_mode,
                    color,
                    color_bar,
                    color_scale,
                    opacity,
                    flat_shading,
                    lighting,
                    light_position,
                    delaunay_axis,
                    contour,
                    facet_column,
                    &config,
                )
            }
            None => Self::create_ir_traces(
                data,
                x,
                y,
                z,
                i,
                j,
                k,
                intensity,
                intensity_mode,
                color,
                color_bar,
                color_scale,
                opacity,
                flat_shading,
                lighting,
                light_position,
                delaunay_axis,
                contour,
            ),
        };

        let layout = LayoutIR {
            title: plot_title,
            x_title,
            y_title,
            y2_title: None,
            z_title,
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
}

#[bon]
impl Mesh3D {
    #[builder(
        start_fn = try_builder,
        finish_fn = try_build,
        builder_type = Mesh3DTryBuilder,
        on(String, into),
        on(Text, into),
    )]
    pub fn try_new(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        i: Option<&str>,
        j: Option<&str>,
        k: Option<&str>,
        intensity: Option<&str>,
        intensity_mode: Option<IntensityMode>,
        color: Option<Rgb>,
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        _reverse_scale: Option<bool>,
        _show_scale: Option<bool>,
        opacity: Option<f64>,
        flat_shading: Option<bool>,
        lighting: Option<&Lighting>,
        light_position: Option<(i32, i32, i32)>,
        delaunay_axis: Option<&str>,
        contour: Option<bool>,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        z_title: Option<Text>,
        legend: Option<&Legend>,
    ) -> Result<Self, crate::io::PlotlarsError> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Self::__orig_new(
                data,
                x,
                y,
                z,
                i,
                j,
                k,
                intensity,
                intensity_mode,
                color,
                color_bar,
                color_scale,
                _reverse_scale,
                _show_scale,
                opacity,
                flat_shading,
                lighting,
                light_position,
                delaunay_axis,
                contour,
                facet,
                facet_config,
                plot_title,
                x_title,
                y_title,
                z_title,
                legend,
            )
        }))
        .map_err(|panic| {
            let msg = panic
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| panic.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "unknown error".to_string());
            crate::io::PlotlarsError::PlotBuild { message: msg }
        })
    }
}

impl Mesh3D {
    fn get_integer_column(data: &DataFrame, column: &str) -> Vec<usize> {
        let column_data = data.column(column).expect("Column not found");

        column_data
            .cast(&polars::prelude::DataType::UInt32)
            .expect("Failed to cast to u32")
            .u32()
            .expect("Failed to extract u32 values")
            .into_iter()
            .map(|opt| opt.unwrap_or(0) as usize)
            .collect()
    }

    fn get_numeric_column_f64(data: &DataFrame, column: &str) -> Vec<f64> {
        let column_data = crate::data::get_numeric_column(data, column);
        column_data
            .into_iter()
            .map(|opt| opt.unwrap_or(0.0) as f64)
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        i: Option<&str>,
        j: Option<&str>,
        k: Option<&str>,
        intensity: Option<&str>,
        intensity_mode: Option<IntensityMode>,
        color: Option<Rgb>,
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        opacity: Option<f64>,
        flat_shading: Option<bool>,
        lighting: Option<&Lighting>,
        light_position: Option<(i32, i32, i32)>,
        delaunay_axis: Option<&str>,
        contour: Option<bool>,
    ) -> Vec<TraceIR> {
        let ir = Self::build_mesh3d_ir(
            data,
            x,
            y,
            z,
            i,
            j,
            k,
            intensity,
            intensity_mode,
            color,
            color_bar,
            color_scale,
            opacity,
            flat_shading,
            lighting,
            light_position,
            delaunay_axis,
            contour,
            None,
        );
        vec![TraceIR::Mesh3D(ir)]
    }

    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces_faceted(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        i: Option<&str>,
        j: Option<&str>,
        k: Option<&str>,
        intensity: Option<&str>,
        intensity_mode: Option<IntensityMode>,
        color: Option<Rgb>,
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        opacity: Option<f64>,
        flat_shading: Option<bool>,
        lighting: Option<&Lighting>,
        light_position: Option<(i32, i32, i32)>,
        delaunay_axis: Option<&str>,
        contour: Option<bool>,
        facet_column: &str,
        config: &FacetConfig,
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

            let ir = Self::build_mesh3d_ir(
                &facet_data,
                x,
                y,
                z,
                i,
                j,
                k,
                intensity,
                intensity_mode,
                color,
                color_bar,
                color_scale,
                opacity,
                flat_shading,
                lighting,
                light_position,
                delaunay_axis,
                contour,
                Some(scene),
            );

            traces.push(TraceIR::Mesh3D(ir));
        }

        traces
    }

    #[allow(clippy::too_many_arguments)]
    fn build_mesh3d_ir(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        i: Option<&str>,
        j: Option<&str>,
        k: Option<&str>,
        intensity: Option<&str>,
        intensity_mode: Option<IntensityMode>,
        color: Option<Rgb>,
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        opacity: Option<f64>,
        flat_shading: Option<bool>,
        lighting: Option<&Lighting>,
        light_position: Option<(i32, i32, i32)>,
        delaunay_axis: Option<&str>,
        contour: Option<bool>,
        scene_ref: Option<String>,
    ) -> Mesh3DIRStruct {
        let x_data = ColumnData::Numeric(crate::data::get_numeric_column(data, x));
        let y_data = ColumnData::Numeric(crate::data::get_numeric_column(data, y));
        let z_data = ColumnData::Numeric(crate::data::get_numeric_column(data, z));

        let i_data = if let (Some(i_col), Some(j_col), Some(k_col)) = (i, j, k) {
            let _ = (j_col, k_col);
            Some(ColumnData::Numeric(
                Self::get_integer_column(data, i_col)
                    .into_iter()
                    .map(|v| Some(v as f32))
                    .collect(),
            ))
        } else {
            None
        };

        let j_data = if let (Some(_), Some(j_col), Some(_)) = (i, j, k) {
            Some(ColumnData::Numeric(
                Self::get_integer_column(data, j_col)
                    .into_iter()
                    .map(|v| Some(v as f32))
                    .collect(),
            ))
        } else {
            None
        };

        let k_data = if let (Some(_), Some(_), Some(k_col)) = (i, j, k) {
            Some(ColumnData::Numeric(
                Self::get_integer_column(data, k_col)
                    .into_iter()
                    .map(|v| Some(v as f32))
                    .collect(),
            ))
        } else {
            None
        };

        let intensity_data = intensity.map(|intensity_col| {
            ColumnData::Numeric(
                Self::get_numeric_column_f64(data, intensity_col)
                    .into_iter()
                    .map(|v| Some(v as f32))
                    .collect(),
            )
        });

        Mesh3DIRStruct {
            x: x_data,
            y: y_data,
            z: z_data,
            i: i_data,
            j: j_data,
            k: k_data,
            intensity: intensity_data,
            intensity_mode,
            color_scale,
            color_bar: color_bar.cloned(),
            lighting: lighting.cloned(),
            opacity,
            color,
            flat_shading,
            light_position,
            delaunay_axis: delaunay_axis.map(|s| s.to_string()),
            contour,
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

impl crate::Plot for Mesh3D {
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

    fn sample_df() -> DataFrame {
        df![
            "x" => [0.0, 1.0, 0.5, 0.5],
            "y" => [0.0, 0.0, 1.0, 0.5],
            "z" => [0.0, 0.0, 0.0, 1.0]
        ]
        .unwrap()
    }

    #[test]
    fn test_basic_one_trace() {
        let df = sample_df();
        let plot = Mesh3D::builder().data(&df).x("x").y("y").z("z").build();
        assert_eq!(plot.ir_traces().len(), 1);
    }

    #[test]
    fn test_trace_variant() {
        let df = sample_df();
        let plot = Mesh3D::builder().data(&df).x("x").y("y").z("z").build();
        assert!(matches!(plot.ir_traces()[0], TraceIR::Mesh3D(_)));
    }

    #[test]
    fn test_layout_no_cartesian_axes() {
        let df = sample_df();
        let plot = Mesh3D::builder().data(&df).x("x").y("y").z("z").build();
        assert!(plot.ir_layout().axes_2d.is_none());
    }

    #[test]
    fn test_layout_title() {
        let df = sample_df();
        let plot = Mesh3D::builder()
            .data(&df)
            .x("x")
            .y("y")
            .z("z")
            .plot_title("Mesh")
            .build();
        assert!(plot.ir_layout().title.is_some());
    }
}
