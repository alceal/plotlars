use bon::bon;

use crate::{
    components::{Axis, ColorBar, Coloring, FacetConfig, Legend, Palette, Text},
    ir::data::ColumnData,
    ir::layout::LayoutIR,
    ir::trace::{ContourPlotIR, TraceIR},
};
use polars::frame::DataFrame;

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
#[derive(Clone)]
#[allow(dead_code)]
pub struct ContourPlot {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
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
        let grid = facet.map(|facet_column| {
            let config = facet_config.cloned().unwrap_or_default();
            let facet_categories =
                crate::data::get_unique_groups(data, facet_column, config.sorter);
            let n_facets = facet_categories.len();
            let (ncols, nrows) =
                crate::faceting::calculate_grid_dimensions(n_facets, config.cols, config.rows);
            crate::ir::facet::GridSpec {
                kind: crate::ir::facet::FacetKind::Axis,
                rows: nrows,
                cols: ncols,
                h_gap: config.h_gap,
                v_gap: config.v_gap,
                scales: config.scales.clone(),
                n_facets,
                facet_categories,
                title_style: config.title_style.clone(),
                x_title: x_title.clone(),
                y_title: y_title.clone(),
                x_axis: x_axis.cloned(),
                y_axis: y_axis.cloned(),
                legend_title: None,
                legend: legend.cloned(),
            }
        });

        let layout = LayoutIR {
            title: plot_title.clone(),
            x_title: if grid.is_some() {
                None
            } else {
                x_title.clone()
            },
            y_title: if grid.is_some() {
                None
            } else {
                y_title.clone()
            },
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
            axes_2d: if grid.is_some() {
                None
            } else {
                Some(crate::ir::layout::Axes2dIR {
                    x_axis: x_axis.cloned(),
                    y_axis: y_axis.cloned(),
                    y2_axis: None,
                })
            },
            scene_3d: None,
            polar: None,
            mapbox: None,
            grid,
            annotations: vec![],
        };

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
                    show_lines,
                    coloring,
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
                show_lines,
                coloring,
            ),
        };

        Self { traces, layout }
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
        show_lines: Option<bool>,
        coloring: Option<Coloring>,
    ) -> Vec<TraceIR> {
        vec![TraceIR::ContourPlot(ContourPlotIR {
            x: ColumnData::Numeric(crate::data::get_numeric_column(data, x)),
            y: ColumnData::Numeric(crate::data::get_numeric_column(data, y)),
            z: ColumnData::Numeric(crate::data::get_numeric_column(data, z)),
            color_scale,
            color_bar: color_bar.cloned(),
            coloring,
            show_lines,
            show_labels: None,
            n_contours: None,
            reverse_scale,
            show_scale,
            z_min: None,
            z_max: None,
            subplot_ref: None,
        })]
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
        show_lines: Option<bool>,
        coloring: Option<Coloring>,
    ) -> Vec<TraceIR> {
        const MAX_FACETS: usize = 8;

        let facet_categories = crate::data::get_unique_groups(data, facet_column, config.sorter);

        if facet_categories.len() > MAX_FACETS {
            panic!(
                "Facet column '{}' has {} unique values, but plotly.rs supports maximum {} subplots",
                facet_column,
                facet_categories.len(),
                MAX_FACETS
            );
        }

        let z_range = Self::calculate_global_z_range(data, z);

        let mut traces = Vec::new();

        for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
            let facet_data = crate::data::filter_data_by_group(data, facet_column, facet_value);

            let subplot_ref = format!(
                "{}{}",
                crate::faceting::get_axis_reference(facet_idx, "x"),
                crate::faceting::get_axis_reference(facet_idx, "y")
            );

            let show_scale_for_facet = if facet_idx == 0 {
                show_scale
            } else {
                Some(false)
            };

            let (z_min, z_max) = match z_range {
                Some((zmin, zmax)) => (Some(zmin), Some(zmax)),
                None => (None, None),
            };

            traces.push(TraceIR::ContourPlot(ContourPlotIR {
                x: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, x)),
                y: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, y)),
                z: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, z)),
                color_scale,
                color_bar: color_bar.cloned(),
                coloring,
                show_lines,
                show_labels: None,
                n_contours: None,
                reverse_scale,
                show_scale: show_scale_for_facet,
                z_min,
                z_max,
                subplot_ref: Some(subplot_ref),
            }));
        }

        traces
    }

    fn calculate_global_z_range(data: &DataFrame, z: &str) -> Option<(f64, f64)> {
        let z_data = crate::data::get_numeric_column(data, z);

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
}

impl crate::Plot for ContourPlot {
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
            "x" => [1.0, 2.0, 3.0],
            "y" => [4.0, 5.0, 6.0],
            "z" => [7.0, 8.0, 9.0]
        ]
        .unwrap();
        let plot = ContourPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .z("z")
            .build();
        assert_eq!(plot.ir_traces().len(), 1);
        assert!(matches!(plot.ir_traces()[0], TraceIR::ContourPlot(_)));
    }

    #[test]
    fn test_layout_has_axes() {
        let df = df![
            "x" => [1.0, 2.0],
            "y" => [3.0, 4.0],
            "z" => [5.0, 6.0]
        ]
        .unwrap();
        let plot = ContourPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .z("z")
            .build();
        assert!(plot.ir_layout().axes_2d.is_some());
    }

    #[test]
    fn test_layout_title() {
        let df = df![
            "x" => [1.0],
            "y" => [2.0],
            "z" => [3.0]
        ]
        .unwrap();
        let plot = ContourPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .z("z")
            .plot_title("Contour")
            .build();
        assert!(plot.ir_layout().title.is_some());
    }
}
