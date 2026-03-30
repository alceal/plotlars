use bon::bon;

use polars::frame::DataFrame;

use crate::{
    components::{FacetConfig, Legend, Rgb, Text},
    ir::data::ColumnData,
    ir::layout::LayoutIR,
    ir::trace::{PieChartIR, TraceIR},
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
/// let dataset = LazyCsvReader::new(PlRefPath::new("data/penguins.csv"))
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
#[derive(Clone)]
#[allow(dead_code)]
pub struct PieChart {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
}

struct FacetCell {
    pie_x_start: f64,
    pie_x_end: f64,
    pie_y_start: f64,
    pie_y_end: f64,
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
        let grid = facet.map(|facet_column| {
            let config = facet_config.cloned().unwrap_or_default();
            let facet_categories =
                crate::data::get_unique_groups(data, facet_column, config.sorter);
            let n_facets = facet_categories.len();
            let (ncols, nrows) =
                crate::faceting::calculate_grid_dimensions(n_facets, config.cols, config.rows);
            crate::ir::facet::GridSpec {
                kind: crate::ir::facet::FacetKind::Domain,
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
                legend_title: legend_title.clone(),
                legend: legend.cloned(),
            }
        });

        let layout = LayoutIR {
            title: plot_title,
            x_title: None,
            y_title: None,
            y2_title: None,
            z_title: None,
            legend_title: if grid.is_some() { None } else { legend_title },
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

        let traces = match facet {
            Some(facet_column) => {
                let config = facet_config.cloned().unwrap_or_default();
                Self::create_ir_traces_faceted(
                    data,
                    labels,
                    facet_column,
                    &config,
                    hole,
                    pull,
                    rotation,
                    colors,
                )
            }
            None => Self::create_ir_traces(data, labels, hole, pull, rotation, colors),
        };
        Self { traces, layout }
    }
    fn create_ir_traces(
        data: &DataFrame,
        labels: &str,
        hole: Option<f64>,
        pull: Option<f64>,
        rotation: Option<f64>,
        colors: Option<Vec<Rgb>>,
    ) -> Vec<TraceIR> {
        vec![TraceIR::PieChart(PieChartIR {
            labels: ColumnData::String(crate::data::get_string_column(data, labels)),
            values: None,
            name: None,
            hole,
            pull,
            rotation,
            colors,
            domain_x: Some((0.0, 1.0)),
            domain_y: Some((0.0, 0.9)),
        })]
    }

    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces_faceted(
        data: &DataFrame,
        labels: &str,
        facet_column: &str,
        config: &FacetConfig,
        hole: Option<f64>,
        pull: Option<f64>,
        rotation: Option<f64>,
        colors: Option<Vec<Rgb>>,
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

        let n_facets = facet_categories.len();
        let (ncols, nrows) =
            crate::faceting::calculate_grid_dimensions(n_facets, config.cols, config.rows);

        let facet_categories_non_empty: Vec<String> = facet_categories
            .iter()
            .filter(|facet_value| {
                let facet_data = crate::data::filter_data_by_group(data, facet_column, facet_value);
                facet_data.height() > 0
            })
            .cloned()
            .collect();

        let mut traces = Vec::new();

        for (idx, facet_value) in facet_categories_non_empty.iter().enumerate() {
            let facet_data = crate::data::filter_data_by_group(data, facet_column, facet_value);

            let cell = Self::calculate_facet_cell(idx, ncols, nrows, config.h_gap, config.v_gap);

            traces.push(TraceIR::PieChart(PieChartIR {
                labels: ColumnData::String(crate::data::get_string_column(&facet_data, labels)),
                values: None,
                name: None,
                hole,
                pull,
                rotation,
                colors: colors.clone(),
                domain_x: Some((cell.pie_x_start, cell.pie_x_end)),
                domain_y: Some((cell.pie_y_start, cell.pie_y_end)),
            }));
        }

        traces
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
        FacetCell {
            pie_x_start,
            pie_x_end,
            pie_y_start,
            pie_y_end,
        }
    }

}

impl crate::Plot for PieChart {
    fn ir_traces(&self) -> &[TraceIR] {
        &self.traces
    }

    fn ir_layout(&self) -> &LayoutIR {
        &self.layout
    }
}
