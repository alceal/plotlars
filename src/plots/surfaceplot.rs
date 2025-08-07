use bon::bon;
use indexmap::IndexSet;
use ordered_float::OrderedFloat;

use plotly::{Layout as LayoutPlotly, Surface, Trace, surface::Position};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper, Polar},
    components::{ColorBar, Lighting, Palette, Text},
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
///   *fresnel* and light position.  Leaving it `None` applies Plotly’s
///   default Phong shading.
/// * `opacity` – An optional `f64` in `[0.0, 1.0]` that sets the global
///   transparency of the surface (1 = opaque, 0 = fully transparent).
/// * `plot_title` – An optional `Text` that customizes the title (content,
///   font, size, alignment).
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
/// let x_base: Vec<f64> = Array::linspace(-10.0, 10.0, n).into_raw_vec();
/// let y_base: Vec<f64> = Array::linspace(-10.0, 10.0, n).into_raw_vec();
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
#[derive(Clone, Serialize)]
pub struct SurfacePlot {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
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
        plot_title: Option<Text>,
    ) -> Self {
        let legend = None;
        let legend_title = None;
        let x_title = None;
        let y_title = None;
        let z_title = None;
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
            color_bar,
            color_scale,
            reverse_scale,
            show_scale,
            lighting,
            opacity,
        );

        Self { traces, layout }
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

        trace
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
}
