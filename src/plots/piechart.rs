use bon::bon;

use plotly::{
    common::{Anchor, Domain},
    layout::Annotation,
    Layout as LayoutPlotly, Pie, Trace,
};

use polars::frame::DataFrame;
use serde::Serialize;
use std::collections::HashMap;

use crate::{
    common::{Layout, PlotHelper, Polar},
    components::{FacetConfig, Legend, Rgb, Text},
};

/// A structure representing a pie chart.
///
/// The `PieChart` struct allows for the creation and customization of pie charts, supporting
/// features such as labels, hole size for donut-style charts, slice pulling, rotation, faceting, and customizable plot titles.
/// It is ideal for visualizing proportions and distributions in categorical data.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `labels` - A string slice specifying the column name to be used for slice labels.
/// * `facet` - An optional string slice specifying the column name to be used for creating facets (small multiples).
/// * `facet_config` - An optional reference to a `FacetConfig` struct for customizing facet layout and behavior.
/// * `hole` - An optional `f64` value specifying the size of the hole in the center of the pie chart.
///   A value of `0.0` creates a full pie chart, while a value closer to `1.0` creates a thinner ring.
/// * `pull` - An optional `f64` value specifying the fraction by which each slice should be pulled out from the center.
/// * `rotation` - An optional `f64` value specifying the starting angle (in degrees) of the first slice.
/// * `colors` - An optional vector of `Rgb` values specifying colors for consistent slice colors across facets.
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
/// * `legend_title` - An optional `Text` struct specifying the title of the legend.
/// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot (e.g., positioning, font, etc.).
///
/// # Example
///
/// ## Basic Pie Chart with Customization
///
/// ```rust
/// use plotlars::{PieChart, Plot, Text};
/// use polars::prelude::*;
///
/// let dataset = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
///     .finish()
///     .unwrap()
///     .select([col("species")])
///     .collect()
///     .unwrap();
///
/// PieChart::builder()
///     .data(&dataset)
///     .labels("species")
///     .hole(0.4)
///     .pull(0.01)
///     .rotation(20.0)
///     .plot_title(
///         Text::from("Pie Chart")
///             .font("Arial")
///             .size(18)
///             .x(0.485)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/q44HDwT.png)
#[derive(Clone, Serialize)]
pub struct PieChart {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl PieChart {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        labels: &str,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
        hole: Option<f64>,
        pull: Option<f64>,
        rotation: Option<f64>,
        colors: Option<Vec<Rgb>>,
        plot_title: Option<Text>,
        legend_title: Option<Text>,
        legend: Option<&Legend>,
    ) -> Self {
        let x_title = None;
        let y_title = None;
        let z_title = None;
        let x_axis = None;
        let y_axis = None;
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
                    legend_title,
                    legend,
                );

                let traces = Self::create_faceted_traces(
                    data,
                    labels,
                    facet_column,
                    &config,
                    hole,
                    pull,
                    rotation,
                    colors,
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

                let traces = Self::create_traces(data, labels, hole, pull, rotation, colors);

                (layout, traces)
            }
        };

        Self { traces, layout }
    }

    fn create_traces(
        data: &DataFrame,
        labels: &str,
        hole: Option<f64>,
        pull: Option<f64>,
        rotation: Option<f64>,
        colors: Option<Vec<Rgb>>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

        let color_map = if let Some(ref color_vec) = colors {
            let label_values = Self::get_string_column(data, labels);
            let unique_labels: Vec<String> = label_values
                .iter()
                .filter_map(|s| s.as_ref().map(|v| v.to_string()))
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();

            Some(Self::create_global_color_map(&unique_labels, color_vec))
        } else {
            None
        };

        // Create default domain that reserves 10% space at top for title
        // This matches the default title y-position of 0.9, creating visual separation
        let default_domain = Domain::new().x(&[0.0, 1.0]).y(&[0.0, 0.9]);

        let trace = Self::create_trace(
            data,
            labels,
            hole,
            pull,
            rotation,
            Some(default_domain),
            color_map,
        );

        traces.push(trace);
        traces
    }

    #[allow(clippy::too_many_arguments)]
    fn create_trace(
        data: &DataFrame,
        labels: &str,
        hole: Option<f64>,
        pull: Option<f64>,
        rotation: Option<f64>,
        domain: Option<Domain>,
        color_map: Option<HashMap<String, String>>,
    ) -> Box<dyn Trace + 'static> {
        let labels = Self::get_string_column(data, labels)
            .iter()
            .filter_map(|s| {
                if s.is_some() {
                    Some(s.clone().unwrap().to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        let mut trace = Pie::<u32>::from_labels(&labels);

        if let Some(hole) = hole {
            trace = trace.hole(hole);
        }

        if let Some(pull) = pull {
            trace = trace.pull(pull);
        }

        if let Some(rotation) = rotation {
            trace = trace.rotation(rotation);
        }

        if let Some(domain_val) = domain {
            trace = trace.domain(domain_val);
        }

        if let Some(color_mapping) = color_map {
            let colors: Vec<String> = labels
                .iter()
                .map(|label| {
                    color_mapping
                        .get(label)
                        .cloned()
                        .unwrap_or_else(|| "#636EFA".to_string())
                })
                .collect();
            trace = trace.marker(plotly::common::Marker::new().color_array(colors));
        }

        trace
    }

    #[allow(clippy::too_many_arguments)]
    fn create_faceted_traces(
        data: &DataFrame,
        labels: &str,
        facet_column: &str,
        config: &FacetConfig,
        hole: Option<f64>,
        pull: Option<f64>,
        rotation: Option<f64>,
        colors: Option<Vec<Rgb>>,
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

        let color_map = if let Some(ref color_vec) = colors {
            let label_values = Self::get_string_column(data, labels);
            let unique_labels: Vec<String> = label_values
                .iter()
                .filter_map(|s| s.as_ref().map(|v| v.to_string()))
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();

            Some(Self::create_global_color_map(&unique_labels, color_vec))
        } else {
            None
        };

        let n_facets = facet_categories.len();
        let (ncols, nrows) = Self::calculate_grid_dimensions(n_facets, config.cols, config.rows);

        let facet_categories_non_empty: Vec<String> = facet_categories
            .iter()
            .filter(|facet_value| {
                let facet_data = Self::filter_data_by_group(data, facet_column, facet_value);
                facet_data.height() > 0
            })
            .cloned()
            .collect();

        let mut all_traces = Vec::new();

        for (idx, facet_value) in facet_categories_non_empty.iter().enumerate() {
            let facet_data = Self::filter_data_by_group(data, facet_column, facet_value);

            let domain = Self::calculate_pie_domain(idx, ncols, nrows, config.h_gap, config.v_gap);

            let trace = Self::create_trace(
                &facet_data,
                labels,
                hole,
                pull,
                rotation,
                Some(domain),
                color_map.clone(),
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
    ) -> LayoutPlotly {
        let facet_categories = Self::get_unique_groups(data, facet_column, config.sorter);

        let facet_categories_non_empty: Vec<String> = facet_categories
            .iter()
            .filter(|facet_value| {
                let facet_data = Self::filter_data_by_group(data, facet_column, facet_value);
                facet_data.height() > 0
            })
            .cloned()
            .collect();

        let n_facets = facet_categories_non_empty.len();
        let (ncols, nrows) = Self::calculate_grid_dimensions(n_facets, config.cols, config.rows);

        let mut layout = LayoutPlotly::new();

        if let Some(title) = plot_title {
            layout = layout.title(title.to_plotly());
        }

        let annotations = Self::create_facet_annotations_pie(
            &facet_categories_non_empty,
            ncols,
            nrows,
            config.title_style.as_ref(),
            config.h_gap,
            config.v_gap,
        );
        layout = layout.annotations(annotations);

        layout = layout.legend(Legend::set_legend(legend_title, legend));

        layout
    }

    /// Calculates the grid cell positions for a subplot with reserved space for titles.
    ///
    /// This function computes both the pie chart domain and annotation position,
    /// ensuring that space is reserved above each pie chart for the facet title.
    /// The title space prevents overlap between annotations and adjacent pie charts.
    fn calculate_facet_cell(
        subplot_index: usize,
        ncols: usize,
        nrows: usize,
        x_gap: Option<f64>,
        y_gap: Option<f64>,
    ) -> FacetCell {
        let row = subplot_index / ncols;
        let col = subplot_index % ncols;

        let x_gap_val = x_gap.unwrap_or(0.05);
        let y_gap_val = y_gap.unwrap_or(0.10);

        // Reserve space for facet title (10% of each cell's height)
        const TITLE_HEIGHT_RATIO: f64 = 0.10;
        // Padding ratio creates buffer space above annotation (35% of reserved title space)
        const TITLE_PADDING_RATIO: f64 = 0.35;

        // Calculate total cell dimensions
        let cell_width = (1.0 - x_gap_val * (ncols - 1) as f64) / ncols as f64;
        let cell_height = (1.0 - y_gap_val * (nrows - 1) as f64) / nrows as f64;

        // Calculate cell boundaries
        let cell_x_start = col as f64 * (cell_width + x_gap_val);
        let cell_y_top = 1.0 - row as f64 * (cell_height + y_gap_val);
        let cell_y_bottom = cell_y_top - cell_height;

        // Reserve title space at the top of the cell (maintains 90% pie size)
        let title_height = cell_height * TITLE_HEIGHT_RATIO;
        let pie_y_top = cell_y_top - title_height;

        // Pie chart domain (bottom 90% of the cell - preserved from original)
        let pie_x_start = cell_x_start;
        let pie_x_end = cell_x_start + cell_width;
        let pie_y_start = cell_y_bottom;
        let pie_y_end = pie_y_top;

        // Calculate annotation position with padding buffer
        // Padding creates visual space above annotation without reducing pie size
        let padding_height = title_height * TITLE_PADDING_RATIO;
        let actual_title_height = title_height - padding_height;
        let annotation_x = cell_x_start + cell_width / 2.0;
        let annotation_y = pie_y_top + padding_height + (actual_title_height / 2.0);

        FacetCell {
            pie_x_start,
            pie_x_end,
            pie_y_start,
            pie_y_end,
            annotation_x,
            annotation_y,
        }
    }

    fn calculate_pie_domain(
        subplot_index: usize,
        ncols: usize,
        nrows: usize,
        x_gap: Option<f64>,
        y_gap: Option<f64>,
    ) -> Domain {
        let cell = Self::calculate_facet_cell(subplot_index, ncols, nrows, x_gap, y_gap);
        Domain::new()
            .x(&[cell.pie_x_start, cell.pie_x_end])
            .y(&[cell.pie_y_start, cell.pie_y_end])
    }

    fn create_facet_annotations_pie(
        categories: &[String],
        ncols: usize,
        nrows: usize,
        title_style: Option<&Text>,
        x_gap: Option<f64>,
        y_gap: Option<f64>,
    ) -> Vec<Annotation> {
        categories
            .iter()
            .enumerate()
            .map(|(i, cat)| {
                let cell = Self::calculate_facet_cell(i, ncols, nrows, x_gap, y_gap);

                let mut ann = Annotation::new()
                    .text(cat.as_str())
                    .x_ref("paper")
                    .y_ref("paper")
                    .x_anchor(Anchor::Center)
                    .y_anchor(Anchor::Middle)
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

    fn create_global_color_map(labels: &[String], colors: &[Rgb]) -> HashMap<String, String> {
        labels
            .iter()
            .enumerate()
            .map(|(i, label)| {
                let color_idx = i % colors.len();
                let rgb = &colors[color_idx];
                let color_str = format!("rgb({},{},{})", rgb.0, rgb.1, rgb.2);
                (label.clone(), color_str)
            })
            .collect()
    }
}

/// Helper struct containing calculated positions for a facet cell
struct FacetCell {
    pie_x_start: f64,
    pie_x_end: f64,
    pie_y_start: f64,
    pie_y_end: f64,
    annotation_x: f64,
    annotation_y: f64,
}

impl Layout for PieChart {}
impl Polar for PieChart {}

impl PlotHelper for PieChart {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
