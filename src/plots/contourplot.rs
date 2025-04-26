use bon::bon;

use plotly::{
    Contour,
    Layout as LayoutPlotly,
    Trace,
    contour::Contours,
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper, Polar},
    components::{Axis, ColorBar, Coloring, Palette, Text},
};

/// A structure representing a contour plot.
///
/// The `ContourPlot` struct enables the creation of contour visualizations that display level
/// curves of a three‑dimensional surface on a two‑dimensional plane. It offers extensive
/// configuration options for contour styling, color scaling, axis appearance, legends, and
/// annotations. Users can fine‑tune the contour interval, choose from predefined color palettes,
/// reverse or hide the color scale, and set custom titles for both the plot and its axes in
/// order to improve the readability of complex surfaces.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `x` - A string slice specifying the column name for x‑axis values.
/// * `y` - A string slice specifying the column name for y‑axis values.
/// * `z` - A string slice specifying the column name for z‑axis values whose magnitude
///   determines each contour line.
/// * `color_bar` - An optional reference to a `ColorBar` struct for customizing the color bar
///   appearance.
/// * `color_scale` - An optional `Palette` enum for specifying the color palette (e.g.,
///   `Palette::Viridis`).
/// * `reverse_scale` - An optional boolean to reverse the color scale direction.
/// * `show_scale` - An optional boolean to display the color scale on the plot.
/// * `contours` - An optional reference to a `Contours` struct for configuring the contour
///   interval, size, and coloring.
/// * `plot_title` - An optional `Text` struct for setting the title of the plot.
/// * `x_title` - An optional `Text` struct for labeling the x‑axis.
/// * `y_title` - An optional `Text` struct for labeling the y‑axis.
/// * `x_axis` - An optional reference to an `Axis` struct for customizing x‑axis appearance.
/// * `y_axis` - An optional reference to an `Axis` struct for customizing y‑axis appearance.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Plot, Coloring, Contours, ContourPlot, Palette};
///
/// let dataset = df!(
///         "x" => &[0.0, 0.0, 0.0, 2.5, 2.5, 2.5, 5.0, 5.0, 5.0],
///         "y" => &[0.0, 7.5, 15.0, 0.0, 7.5, 15.0, 0.0, 7.5, 15.0],
///         "z" => &[0.0, 5.0, 10.0, 5.0, 2.5, 5.0, 10.0, 0.0, 0.0],
///     )
///     .unwrap();
///
/// ContourPlot::builder()
///     .data(&dataset)
///     .x("x")
///     .y("y")
///     .z("z")
///     .color_scale(Palette::Viridis)
///     .reverse_scale(true)
///     .coloring(Coloring::Fill)
///     .show_lines(false)
///     .plot_title(
///         Text::from("Contour Plot")
///             .font("Arial")
///             .size(18)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/VWgxHC8.png)
#[derive(Clone, Serialize)]
pub struct ContourPlot {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl ContourPlot {
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
        show_lines: Option<bool>,
        coloring: Option<Coloring>,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
    ) -> Self {
        let legend = None;
        let legend_title = None;
        let z_title = None;
        let z_axis = None;

        let layout = Self::create_layout(
            plot_title,
            x_title,
            y_title,
            z_title,
            legend_title,
            x_axis,
            y_axis,
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
            show_lines,
            coloring,
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
        show_lines: Option<bool>,
        coloring: Option<Coloring>,
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
            show_lines,
            coloring,
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
        show_lines: Option<bool>,
        coloring: Option<Coloring>,
    ) -> Box<dyn Trace + 'static> {
        let x = Self::get_numeric_column(data, x);
        let y = Self::get_numeric_column(data, y);
        let z = Self::get_numeric_column(data, z);

        let mut trace = Contour::new(x, y, z);

        trace = Self::set_color_bar(trace, color_bar);
        trace = Self::set_color_scale(trace, color_scale);
        trace = Self::set_reverse_scale(trace, reverse_scale);
        trace = Self::set_show_scale(trace, show_scale);

        let mut contours = Contours::new();

        contours = Self::set_coloring(contours, coloring);
        contours = Self::set_show_lines(contours, show_lines);

        trace
            .contours(contours)
    }

    fn set_show_lines(
        mut contours: Contours,
        show_lines: Option<bool>,
    ) -> Contours {
        if let Some(show_lines) = show_lines {
            contours = contours.show_lines(show_lines)
        }

        contours
    }

    fn set_coloring(
        mut contours: Contours,
        coloring: Option<Coloring>,
    ) -> Contours {
        if let Some(coloring) = coloring {
            contours = contours.coloring(coloring.to_plotly())
        }

        contours
    }

    fn set_color_bar<X, Y, Z>(
        mut trace: Box<Contour<X, Y, Z>>,
        color_bar: Option<&ColorBar>,
    ) -> Box<Contour<X, Y, Z>>
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

    fn set_color_scale<X, Y, Z>(
        mut trace: Box<Contour<X, Y, Z>>,
        color_scale: Option<Palette>,
    ) -> Box<Contour<X, Y, Z>>
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

    fn set_reverse_scale<X, Y, Z>(
        mut trace: Box<Contour<X, Y, Z>>,
        reverse_scale: Option<bool>,
    ) -> Box<Contour<X, Y, Z>>
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

    fn set_show_scale<X, Y, Z>(
        mut trace: Box<Contour<X, Y, Z>>,
        show_scale: Option<bool>,
    ) -> Box<Contour<X, Y, Z>>
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
}

impl Layout for ContourPlot {}
impl Polar for ContourPlot {}

impl PlotHelper for ContourPlot {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
