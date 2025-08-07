use bon::bon;

use plotly::{
    Layout as LayoutPlotly, Mesh3D as Mesh3DPlotly, Trace,
    mesh3d::{Contour, DelaunayAxis, LightPosition, Lighting as LightingMesh3D},
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper, Polar},
    components::{ColorBar, IntensityMode, Legend, Lighting, Palette, Rgb, Text},
};

/// A structure representing a 3D mesh plot.
///
/// The `Mesh3D` struct is designed to create and customize 3D mesh visualizations
/// with support for explicit triangulation, intensity-based coloring, and various
/// lighting effects. It can handle both auto-triangulated point clouds and
/// explicitly defined mesh connectivity through triangle indices.
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
/// let dataset = DataFrame::new(vec![
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
///         Text::from("Wavy Surface with Custom Lighting")
///             .font("Arial")
///             .size(22),
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/lKI23dJ.png)
#[derive(Clone, Serialize)]
pub struct Mesh3D {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
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
        reverse_scale: Option<bool>,
        show_scale: Option<bool>,
        opacity: Option<f64>,
        flat_shading: Option<bool>,
        lighting: Option<&Lighting>,
        light_position: Option<(i32, i32, i32)>,
        delaunay_axis: Option<&str>,
        contour: Option<bool>,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        z_title: Option<Text>,
        legend: Option<&Legend>,
    ) -> Self {
        let legend_title = None;
        let x_axis = None;
        let y_axis = None;
        let z_axis = None;
        let y2_title = None;
        let y2_axis = None;

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
            i,
            j,
            k,
            intensity,
            intensity_mode,
            color,
            color_bar,
            color_scale,
            reverse_scale,
            show_scale,
            opacity,
            flat_shading,
            lighting,
            light_position,
            delaunay_axis,
            contour,
        );

        Self { traces, layout }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
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
        reverse_scale: Option<bool>,
        show_scale: Option<bool>,
        opacity: Option<f64>,
        flat_shading: Option<bool>,
        lighting: Option<&Lighting>,
        light_position: Option<(i32, i32, i32)>,
        delaunay_axis: Option<&str>,
        contour: Option<bool>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

        let trace = Self::create_trace(
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
            reverse_scale,
            show_scale,
            opacity,
            flat_shading,
            lighting,
            light_position,
            delaunay_axis,
            contour,
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
        i: Option<&str>,
        j: Option<&str>,
        k: Option<&str>,
        intensity: Option<&str>,
        intensity_mode: Option<IntensityMode>,
        color: Option<Rgb>,
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        reverse_scale: Option<bool>,
        show_scale: Option<bool>,
        opacity: Option<f64>,
        flat_shading: Option<bool>,
        lighting: Option<&Lighting>,
        light_position: Option<(i32, i32, i32)>,
        delaunay_axis: Option<&str>,
        contour: Option<bool>,
    ) -> Box<dyn Trace + 'static> {
        let x_data = Self::get_numeric_column(data, x);
        let y_data = Self::get_numeric_column(data, y);
        let z_data = Self::get_numeric_column(data, z);

        let mut trace = Mesh3DPlotly::new(x_data, y_data, z_data, None, None, None);

        if let (Some(i_col), Some(j_col), Some(k_col)) = (i, j, k) {
            let i_data = Self::get_integer_column(data, i_col);
            let j_data = Self::get_integer_column(data, j_col);
            let k_data = Self::get_integer_column(data, k_col);
            trace = trace.i(i_data).j(j_data).k(k_data);
        }

        if let Some(intensity_col) = intensity {
            let intensity_data = Self::get_numeric_column_f64(data, intensity_col);
            trace = trace.intensity(intensity_data);
        }

        trace = Self::set_intensity_mode(trace, intensity_mode);
        trace = Self::set_color(trace, color);
        trace = Self::set_color_bar(trace, color_bar);
        trace = Self::set_color_scale(trace, color_scale);
        trace = Self::set_reverse_scale(trace, reverse_scale);
        trace = Self::set_show_scale(trace, show_scale);
        trace = Self::set_opacity(trace, opacity);
        trace = Self::set_flat_shading(trace, flat_shading);
        trace = Self::set_lighting(trace, lighting);
        trace = Self::set_light_position(trace, light_position);
        trace = Self::set_delaunay_axis(trace, delaunay_axis);
        trace = Self::set_contour(trace, contour);

        trace
    }

    fn set_intensity_mode<X, Y, Z>(
        mut trace: Box<Mesh3DPlotly<X, Y, Z>>,
        intensity_mode: Option<IntensityMode>,
    ) -> Box<Mesh3DPlotly<X, Y, Z>>
    where
        X: Serialize + Clone + Default + 'static,
        Y: Serialize + Clone + Default + 'static,
        Z: Serialize + Clone + Default + 'static,
    {
        if let Some(mode) = intensity_mode {
            trace = trace.intensity_mode(mode.to_plotly());
        }
        trace
    }

    fn set_color<X, Y, Z>(
        mut trace: Box<Mesh3DPlotly<X, Y, Z>>,
        color: Option<Rgb>,
    ) -> Box<Mesh3DPlotly<X, Y, Z>>
    where
        X: Serialize + Clone + Default + 'static,
        Y: Serialize + Clone + Default + 'static,
        Z: Serialize + Clone + Default + 'static,
    {
        if let Some(color) = color {
            trace = trace.color(color.to_plotly());
        }
        trace
    }

    fn set_color_bar<X, Y, Z>(
        mut trace: Box<Mesh3DPlotly<X, Y, Z>>,
        color_bar: Option<&ColorBar>,
    ) -> Box<Mesh3DPlotly<X, Y, Z>>
    where
        X: Serialize + Clone + Default + 'static,
        Y: Serialize + Clone + Default + 'static,
        Z: Serialize + Clone + Default + 'static,
    {
        if let Some(color_bar) = color_bar {
            trace = trace.color_bar(color_bar.to_plotly());
        }
        trace
    }

    fn set_color_scale<X, Y, Z>(
        mut trace: Box<Mesh3DPlotly<X, Y, Z>>,
        color_scale: Option<Palette>,
    ) -> Box<Mesh3DPlotly<X, Y, Z>>
    where
        X: Serialize + Clone + Default + 'static,
        Y: Serialize + Clone + Default + 'static,
        Z: Serialize + Clone + Default + 'static,
    {
        if let Some(color_scale) = color_scale {
            trace = trace.color_scale(color_scale.to_plotly());
        }
        trace
    }

    fn set_reverse_scale<X, Y, Z>(
        mut trace: Box<Mesh3DPlotly<X, Y, Z>>,
        reverse_scale: Option<bool>,
    ) -> Box<Mesh3DPlotly<X, Y, Z>>
    where
        X: Serialize + Clone + Default + 'static,
        Y: Serialize + Clone + Default + 'static,
        Z: Serialize + Clone + Default + 'static,
    {
        if let Some(reverse) = reverse_scale {
            trace = trace.reverse_scale(reverse);
        }
        trace
    }

    fn set_show_scale<X, Y, Z>(
        mut trace: Box<Mesh3DPlotly<X, Y, Z>>,
        show_scale: Option<bool>,
    ) -> Box<Mesh3DPlotly<X, Y, Z>>
    where
        X: Serialize + Clone + Default + 'static,
        Y: Serialize + Clone + Default + 'static,
        Z: Serialize + Clone + Default + 'static,
    {
        if let Some(show) = show_scale {
            trace = trace.show_scale(show);
        }
        trace
    }

    fn set_opacity<X, Y, Z>(
        mut trace: Box<Mesh3DPlotly<X, Y, Z>>,
        opacity: Option<f64>,
    ) -> Box<Mesh3DPlotly<X, Y, Z>>
    where
        X: Serialize + Clone + Default + 'static,
        Y: Serialize + Clone + Default + 'static,
        Z: Serialize + Clone + Default + 'static,
    {
        if let Some(opacity) = opacity {
            trace = trace.opacity(opacity);
        }
        trace
    }

    fn set_flat_shading<X, Y, Z>(
        mut trace: Box<Mesh3DPlotly<X, Y, Z>>,
        flat_shading: Option<bool>,
    ) -> Box<Mesh3DPlotly<X, Y, Z>>
    where
        X: Serialize + Clone + Default + 'static,
        Y: Serialize + Clone + Default + 'static,
        Z: Serialize + Clone + Default + 'static,
    {
        if let Some(flat) = flat_shading {
            trace = trace.flat_shading(flat);
        }
        trace
    }

    fn set_lighting<X, Y, Z>(
        mut trace: Box<Mesh3DPlotly<X, Y, Z>>,
        lighting: Option<&Lighting>,
    ) -> Box<Mesh3DPlotly<X, Y, Z>>
    where
        X: Serialize + Clone + Default + 'static,
        Y: Serialize + Clone + Default + 'static,
        Z: Serialize + Clone + Default + 'static,
    {
        if lighting.is_some() {
            trace = trace.lighting(Self::set_lighting_mesh3d(lighting));
        }
        trace
    }

    fn set_light_position<X, Y, Z>(
        mut trace: Box<Mesh3DPlotly<X, Y, Z>>,
        light_position: Option<(i32, i32, i32)>,
    ) -> Box<Mesh3DPlotly<X, Y, Z>>
    where
        X: Serialize + Clone + Default + 'static,
        Y: Serialize + Clone + Default + 'static,
        Z: Serialize + Clone + Default + 'static,
    {
        if let Some((x, y, z)) = light_position {
            let x_vec = vec![x as f64];
            let y_vec = vec![y as f64];
            let z_vec = vec![z as f64];
            let position = LightPosition::new().x(x_vec).y(y_vec).z(z_vec);
            trace = trace.light_position(position);
        }
        trace
    }

    fn set_delaunay_axis<X, Y, Z>(
        mut trace: Box<Mesh3DPlotly<X, Y, Z>>,
        delaunay_axis: Option<&str>,
    ) -> Box<Mesh3DPlotly<X, Y, Z>>
    where
        X: Serialize + Clone + Default + 'static,
        Y: Serialize + Clone + Default + 'static,
        Z: Serialize + Clone + Default + 'static,
    {
        if let Some(axis) = delaunay_axis {
            let axis = match axis.to_lowercase().as_str() {
                "x" => DelaunayAxis::X,
                "y" => DelaunayAxis::Y,
                "z" => DelaunayAxis::Z,
                _ => DelaunayAxis::Z,
            };
            trace = trace.delaunay_axis(axis);
        }
        trace
    }

    fn set_contour<X, Y, Z>(
        mut trace: Box<Mesh3DPlotly<X, Y, Z>>,
        contour: Option<bool>,
    ) -> Box<Mesh3DPlotly<X, Y, Z>>
    where
        X: Serialize + Clone + Default + 'static,
        Y: Serialize + Clone + Default + 'static,
        Z: Serialize + Clone + Default + 'static,
    {
        if let Some(true) = contour {
            let contour = Contour::new().show(true).width(2);
            trace = trace.contour(contour);
        }
        trace
    }

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
        let column_data = Self::get_numeric_column(data, column);
        column_data
            .into_iter()
            .map(|opt| opt.unwrap_or(0.0) as f64)
            .collect()
    }

    fn set_lighting_mesh3d(lighting: Option<&Lighting>) -> LightingMesh3D {
        let mut lighting_mesh3d = LightingMesh3D::new();

        if let Some(light) = lighting {
            if let Some(ambient) = light.ambient {
                lighting_mesh3d = lighting_mesh3d.ambient(ambient);
            }

            if let Some(diffuse) = light.diffuse {
                lighting_mesh3d = lighting_mesh3d.diffuse(diffuse);
            }

            if let Some(fresnel) = light.fresnel {
                lighting_mesh3d = lighting_mesh3d.fresnel(fresnel);
            }

            if let Some(roughness) = light.roughness {
                lighting_mesh3d = lighting_mesh3d.roughness(roughness);
            }

            if let Some(specular) = light.specular {
                lighting_mesh3d = lighting_mesh3d.specular(specular);
            }
        }

        lighting_mesh3d
    }
}

impl Layout for Mesh3D {}
impl Polar for Mesh3D {}

impl PlotHelper for Mesh3D {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
