use bon::bon;

use plotly::{contour::Contours, Contour, Layout as LayoutPlotly, Trace};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper, Polar},
    components::{Axis, ColorBar, Coloring, FacetConfig, Legend, Palette, Text},
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
/// * `facet` - An optional string slice specifying the column name to be used for faceting (creating multiple subplots).
/// * `facet_config` - An optional reference to a `FacetConfig` struct for customizing facet behavior (grid dimensions, scales, gaps, etc.).
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
/// use plotlars::{Coloring, ContourPlot, Palette, Plot, Text};
/// use polars::prelude::*;
///
/// let dataset = LazyCsvReader::new(PlRefPath::new("data/contour_surface.csv"))
///     .finish()
///     .unwrap()
///     .collect()
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
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
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
        legend: Option<&Legend>,
    ) -> Self {
        let legend_title = None;
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
                    y,
                    z,
                    facet_column,
                    &config,
                    color_bar,
                    color_scale,
                    reverse_scale,
                    show_scale,
                    show_lines,
                    coloring,
                );

                (layout, traces)
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
                    None,
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

                (layout, traces)
            }
        };

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

        trace.contours(contours)
    }

    fn set_show_lines(mut contours: Contours, show_lines: Option<bool>) -> Contours {
        if let Some(show_lines) = show_lines {
            contours = contours.show_lines(show_lines)
        }

        contours
    }

    fn set_coloring(mut contours: Contours, coloring: Option<Coloring>) -> Contours {
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
        show_lines: Option<bool>,
        coloring: Option<Coloring>,
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

        let mut all_traces = Vec::new();

        let z_range = Self::calculate_global_z_range(data, z);

        for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
            let facet_data = Self::filter_data_by_group(data, facet_column, facet_value);

            let x_axis = Self::get_axis_reference(facet_idx, "x");
            let y_axis = Self::get_axis_reference(facet_idx, "y");

            let show_scale_for_facet = if facet_idx == 0 {
                show_scale
            } else {
                Some(false)
            };

            let trace = Self::create_trace_with_axes(
                &facet_data,
                x,
                y,
                z,
                z_range,
                color_bar,
                color_scale,
                reverse_scale,
                show_scale_for_facet,
                show_lines,
                coloring,
                Some(&x_axis),
                Some(&y_axis),
            );

            all_traces.push(trace);
        }

        all_traces
    }

    #[allow(clippy::too_many_arguments)]
    fn create_trace_with_axes(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        z_range: Option<(f64, f64)>,
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        reverse_scale: Option<bool>,
        show_scale: Option<bool>,
        show_lines: Option<bool>,
        coloring: Option<Coloring>,
        x_axis: Option<&str>,
        y_axis: Option<&str>,
    ) -> Box<dyn Trace + 'static> {
        let x = Self::get_numeric_column(data, x);
        let y = Self::get_numeric_column(data, y);
        let z = Self::get_numeric_column(data, z);

        let mut trace = Contour::new(x, y, z);

        if let Some((z_min, z_max)) = z_range {
            trace = trace.zmin(Some(z_min as f32)).zmax(Some(z_max as f32));
        }

        trace = Self::set_color_bar(trace, color_bar);
        trace = Self::set_color_scale(trace, color_scale);
        trace = Self::set_reverse_scale(trace, reverse_scale);
        trace = Self::set_show_scale(trace, show_scale);

        let mut contours = Contours::new();
        contours = Self::set_coloring(contours, coloring);
        contours = Self::set_show_lines(contours, show_lines);

        trace = trace.contours(contours);

        if let Some(axis) = x_axis {
            trace = trace.x_axis(axis);
        }

        if let Some(axis) = y_axis {
            trace = trace.y_axis(axis);
        }

        trace
    }

    fn calculate_global_z_range(data: &DataFrame, z: &str) -> Option<(f64, f64)> {
        let z_data = Self::get_numeric_column(data, z);

        let mut z_min = f64::INFINITY;
        let mut z_max = f64::NEG_INFINITY;
        let mut found_valid = false;

        for val in z_data.iter().flatten() {
            let val_f64 = *val as f64;
            if !val_f64.is_nan() {
                z_min = z_min.min(val_f64);
                z_max = z_max.max(val_f64);
                found_valid = true;
            }
        }

        if found_valid {
            Some((z_min, z_max))
        } else {
            None
        }
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
        crate::faceting::create_axis_faceted_layout::<Self>(
            data,
            facet_column,
            config,
            plot_title,
            x_title,
            y_title,
            legend_title,
            x_axis,
            y_axis,
            legend,
        )
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
