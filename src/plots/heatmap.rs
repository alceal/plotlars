use bon::bon;

use plotly::{
    HeatMap as HeatMapPlotly,
    Layout as LayoutPlotly,
    Trace,
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, Marker, PlotHelper, Polar},
    components::{Axis, ColorBar, Palette, Text},
};

/// A structure representing a heat map.
///
/// The `HeatMap` struct enables the creation of heat map visualizations with options for color scaling,
/// axis customization, legend adjustments, and data value formatting. Users can customize the color
/// scale, adjust the color bar, and set titles for the plot and axes, as well as format ticks and scales
/// for improved data readability.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `x` - A string slice specifying the column name for x-axis values.
/// * `y` - A string slice specifying the column name for y-axis values.
/// * `z` - A string slice specifying the column name for z-axis values, which are represented by the color intensity.
/// * `auto_color_scale` - An optional boolean for enabling automatic color scaling based on data.
/// * `color_bar` - An optional reference to a `ColorBar` struct for customizing the color bar appearance.
/// * `color_scale` - An optional `Palette` enum for specifying the color scale (e.g., Viridis).
/// * `reverse_scale` - An optional boolean to reverse the color scale direction.
/// * `show_scale` - An optional boolean to display the color scale on the plot.
/// * `plot_title` - An optional `Text` struct for setting the title of the plot.
/// * `x_title` - An optional `Text` struct for labeling the x-axis.
/// * `y_title` - An optional `Text` struct for labeling the y-axis.
/// * `x_axis` - An optional reference to an `Axis` struct for customizing x-axis appearance.
/// * `y_axis` - An optional reference to an `Axis` struct for customizing y-axis appearance.
///
/// # Example
///
/// ```rust
/// use plotlars::{ColorBar, HeatMap, Palette, Plot, Text, ValueExponent};
///
/// let dataset = LazyCsvReader::new(PlPath::new("data/heatmap.csv"))
///     .finish()
///     .unwrap()
///     .collect()
///     .unwrap();
///
/// HeatMap::builder()
///     .data(&dataset)
///     .x("x")
///     .y("y")
///     .z("z")
///     .color_bar(
///         &ColorBar::new()
///             .length(290)
///             .value_exponent(ValueExponent::None)
///             .separate_thousands(true)
///             .tick_length(5)
///             .tick_step(2500.0)
///     )
///     .plot_title(
///         Text::from("Heat Map")
///             .font("Arial")
///             .size(18)
///     )
///     .color_scale(Palette::Viridis)
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/5uFih4M.png)
#[derive(Clone, Serialize)]
pub struct HeatMap {
    pub traces: Vec<Box<dyn Trace + 'static>>,
    pub layout: LayoutPlotly,
}

#[bon]
impl HeatMap {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        auto_color_scale: Option<bool>,
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        reverse_scale: Option<bool>,
        show_scale: Option<bool>,
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
            None, // y2_title,
            z_title,
            legend_title,
            x_axis,
            y_axis,
            None, // y2_axis,
            z_axis,
            legend,
        );

        let traces = Self::create_traces(
            data,
            x,
            y,
            z,
            auto_color_scale,
            color_bar,
            color_scale,
            reverse_scale,
            show_scale,
        );

        Self { traces, layout }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        auto_color_scale: Option<bool>,
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        reverse_scale: Option<bool>,
        show_scale: Option<bool>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();
        let trace = Self::create_trace(
            data,
            x,
            y,
            z,
            auto_color_scale,
            color_bar,
            color_scale,
            reverse_scale,
            show_scale,
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
        auto_color_scale: Option<bool>,
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        reverse_scale: Option<bool>,
        show_scale: Option<bool>,
    ) -> Box<dyn Trace + 'static> {
        let x = Self::get_string_column(data, x);
        let y = Self::get_string_column(data, y);
        let z = Self::get_numeric_column(data, z);

        let mut trace = HeatMapPlotly::default().x(x).y(y).z(z);

        trace = Self::set_auto_color_scale(trace, auto_color_scale);
        trace = Self::set_color_bar(trace, color_bar);
        trace = Self::set_color_scale(trace, color_scale);
        trace = Self::set_reverse_scale(trace, reverse_scale);
        trace = Self::set_show_scale(trace, show_scale);
        trace
    }

    fn set_auto_color_scale<X, Y, Z>(
        mut trace: Box<HeatMapPlotly<X, Y, Z>>,
        auto_color_scale: Option<bool>,
    ) -> Box<HeatMapPlotly<X, Y, Z>>
    where
        X: Serialize + Clone,
        Y: Serialize + Clone,
        Z: Serialize + Clone,
    {
        if let Some(auto_color_scale) = auto_color_scale {
            trace = trace.auto_color_scale(auto_color_scale);
        }

        trace
    }

    fn set_color_bar<X, Y, Z>(
        mut trace: Box<HeatMapPlotly<X, Y, Z>>,
        color_bar: Option<&ColorBar>,
    ) -> Box<HeatMapPlotly<X, Y, Z>>
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
        mut trace: Box<HeatMapPlotly<X, Y, Z>>,
        color_scale: Option<Palette>,
    ) -> Box<HeatMapPlotly<X, Y, Z>>
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
        mut trace: Box<HeatMapPlotly<X, Y, Z>>,
        reverse_scale: Option<bool>,
    ) -> Box<HeatMapPlotly<X, Y, Z>>
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
        mut trace: Box<HeatMapPlotly<X, Y, Z>>,
        show_scale: Option<bool>,
    ) -> Box<HeatMapPlotly<X, Y, Z>>
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

impl Layout for HeatMap {}
impl Marker for HeatMap {}
impl Polar for HeatMap {}

impl PlotHelper for HeatMap {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
