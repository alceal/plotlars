use bon::bon;
use std::collections::{hash_map::Entry, HashMap};

use crate::{
    components::{Arrangement, FacetConfig, Legend, Orientation, Rgb, Text},
    ir::data::ColumnData,
    ir::layout::LayoutIR,
    ir::trace::{SankeyDiagramIR, TraceIR},
};
use polars::frame::DataFrame;

/// A structure representing a Sankey diagram.
///
/// The `SankeyDiagram` struct enables the creation of Sankey diagrams, which visualize flows
/// between discrete nodes with link widths proportional to the magnitude of the flow. It
/// offers extensive configuration options for flow orientation, node arrangement, spacing,
/// thickness, and coloring, as well as axis and title customization. Users can specify a
/// single uniform color or per-item colors for both nodes and links, adjust padding between
/// nodes, set node thickness, and supply custom titles and axis labels to produce clear,
/// publication-quality flow visualizations. Faceting support allows creating multiple Sankey
/// diagrams in a grid layout for comparing flows across categories.
///
/// # Arguments
///
/// * `data` – A reference to the `DataFrame` containing the data to be plotted.
/// * `sources` – A string slice naming the column in `data` that contains the source node for each flow.
/// * `targets` – A string slice naming the column in `data` that contains the target node for each flow.
/// * `values` – A string slice naming the column in `data` that contains the numeric value of each flow.
/// * `facet` – An optional string slice naming the column in `data` to be used for creating facets (small multiples).
/// * `facet_config` – An optional reference to a `FacetConfig` struct for customizing facet layout and behavior.
/// * `orientation` – An optional `Orientation` enum to set the overall direction of the diagram
///   (e.g. `Orientation::Horizontal` or `Orientation::Vertical`).
/// * `arrangement` – An optional `Arrangement` enum to choose the node-layout algorithm
///   (e.g. `Arrangement::Snap`, `Arrangement::Perpendicular`, etc.).
/// * `pad` – An optional `usize` specifying the padding (in pixels) between adjacent nodes.
/// * `thickness` – An optional `usize` defining the uniform thickness (in pixels) of all nodes.
/// * `node_color` – An optional `Rgb` value to apply a single uniform color to every node.
/// * `node_colors` – An optional `Vec<Rgb>` supplying individual colors for each node in order.
/// * `link_color` – An optional `Rgb` value to apply a single uniform color to every link.
/// * `link_colors` – An optional `Vec<Rgb>` supplying individual colors for each link in order.
/// * `plot_title` – An optional `Text` struct for setting the overall title of the plot.
/// * `legend_title` – An optional `Text` struct specifying the title of the legend.
/// * `legend` – An optional reference to a `Legend` struct for customizing the legend of the plot.
///
/// # Example
///
/// ```rust
/// use plotlars::{Arrangement, SankeyDiagram, Orientation, Plot, Rgb, Text};
/// use polars::prelude::*;
///
/// let dataset = LazyCsvReader::new(PlRefPath::new("data/sankey_flow.csv"))
///     .finish()
///     .unwrap()
///     .collect()
///     .unwrap();
///
/// SankeyDiagram::builder()
///     .data(&dataset)
///     .sources("source")
///     .targets("target")
///     .values("value")
///     .orientation(Orientation::Horizontal)
///     .arrangement(Arrangement::Freeform)
///     .node_colors(vec![
///         Rgb(222, 235, 247),
///         Rgb(198, 219, 239),
///         Rgb(158, 202, 225),
///         Rgb(107, 174, 214),
///         Rgb( 66, 146, 198),
///         Rgb( 33, 113, 181),
///     ])
///     .link_colors(vec![
///         Rgb(222, 235, 247),
///         Rgb(198, 219, 239),
///         Rgb(158, 202, 225),
///         Rgb(107, 174, 214),
///         Rgb( 66, 146, 198),
///         Rgb( 33, 113, 181),
///     ])
///     .pad(20)
///     .thickness(30)
///     .plot_title(
///         Text::from("Sankey Diagram")
///             .font("Arial")
///             .size(18)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/jvAew8u.png)
#[derive(Clone)]
#[allow(dead_code)]
pub struct SankeyDiagram {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
}

struct FacetCell {
    domain_x_start: f64,
    domain_x_end: f64,
    domain_y_start: f64,
    domain_y_end: f64,
}

#[bon]
impl SankeyDiagram {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        sources: &str,
        targets: &str,
        values: &str,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
        orientation: Option<Orientation>,
        arrangement: Option<Arrangement>,
        pad: Option<usize>,
        thickness: Option<usize>,
        node_color: Option<Rgb>,
        node_colors: Option<Vec<Rgb>>,
        link_color: Option<Rgb>,
        link_colors: Option<Vec<Rgb>>,
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
                    sources,
                    targets,
                    values,
                    facet_column,
                    &config,
                    orientation,
                    arrangement,
                    pad,
                    thickness,
                    node_color,
                    node_colors,
                    link_color,
                    link_colors,
                )
            }
            None => Self::create_ir_traces(
                data,
                sources,
                targets,
                values,
                orientation,
                arrangement,
                pad,
                thickness,
                node_color,
                node_colors,
                link_color,
                link_colors,
            ),
        };
        Self { traces, layout }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces(
        data: &DataFrame,
        sources: &str,
        targets: &str,
        values: &str,
        orientation: Option<Orientation>,
        arrangement: Option<Arrangement>,
        pad: Option<usize>,
        thickness: Option<usize>,
        node_color: Option<Rgb>,
        node_colors: Option<Vec<Rgb>>,
        link_color: Option<Rgb>,
        link_colors: Option<Vec<Rgb>>,
    ) -> Vec<TraceIR> {
        let sources_col = crate::data::get_string_column(data, sources);
        let targets_col = crate::data::get_string_column(data, targets);
        let values_data = crate::data::get_numeric_column(data, values);

        let (labels_unique, label_to_idx) = Self::build_label_index(&sources_col, &targets_col);

        let sources_idx = Self::column_to_indices(&sources_col, &label_to_idx);
        let targets_idx = Self::column_to_indices(&targets_col, &label_to_idx);

        let resolved_node_colors = Self::resolve_node_colors(node_color, node_colors);
        let resolved_link_colors = Self::resolve_link_colors(link_color, link_colors);

        vec![TraceIR::SankeyDiagram(SankeyDiagramIR {
            sources: sources_idx,
            targets: targets_idx,
            values: ColumnData::Numeric(values_data),
            node_labels: labels_unique.iter().map(|s| s.to_string()).collect(),
            orientation,
            arrangement,
            pad,
            thickness,
            node_colors: resolved_node_colors,
            link_colors: resolved_link_colors,
            domain_x: None,
            domain_y: None,
        })]
    }

    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces_faceted(
        data: &DataFrame,
        sources: &str,
        targets: &str,
        values: &str,
        facet_column: &str,
        config: &FacetConfig,
        orientation: Option<Orientation>,
        arrangement: Option<Arrangement>,
        pad: Option<usize>,
        thickness: Option<usize>,
        node_color: Option<Rgb>,
        node_colors: Option<Vec<Rgb>>,
        link_color: Option<Rgb>,
        link_colors: Option<Vec<Rgb>>,
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

        let resolved_node_colors = Self::resolve_node_colors(node_color, node_colors);
        let resolved_link_colors = Self::resolve_link_colors(link_color, link_colors);

        let mut traces = Vec::new();

        for (idx, facet_value) in facet_categories_non_empty.iter().enumerate() {
            let facet_data = crate::data::filter_data_by_group(data, facet_column, facet_value);

            let cell = Self::calculate_facet_cell(idx, ncols, nrows, config.h_gap, config.v_gap);

            let sources_col = crate::data::get_string_column(&facet_data, sources);
            let targets_col = crate::data::get_string_column(&facet_data, targets);
            let values_data = crate::data::get_numeric_column(&facet_data, values);

            let (labels_unique, label_to_idx) = Self::build_label_index(&sources_col, &targets_col);

            let sources_idx = Self::column_to_indices(&sources_col, &label_to_idx);
            let targets_idx = Self::column_to_indices(&targets_col, &label_to_idx);

            traces.push(TraceIR::SankeyDiagram(SankeyDiagramIR {
                sources: sources_idx,
                targets: targets_idx,
                values: ColumnData::Numeric(values_data),
                node_labels: labels_unique.iter().map(|s| s.to_string()).collect(),
                orientation: orientation.clone(),
                arrangement: arrangement.clone(),
                pad,
                thickness,
                node_colors: resolved_node_colors.clone(),
                link_colors: resolved_link_colors.clone(),
                domain_x: Some((cell.domain_x_start, cell.domain_x_end)),
                domain_y: Some((cell.domain_y_start, cell.domain_y_end)),
            }));
        }

        traces
    }

    fn resolve_node_colors(
        node_color: Option<Rgb>,
        node_colors: Option<Vec<Rgb>>,
    ) -> Option<Vec<Rgb>> {
        if let Some(colors) = node_colors {
            Some(colors)
        } else if let Some(color) = node_color {
            Some(vec![color])
        } else {
            None
        }
    }

    fn resolve_link_colors(
        link_color: Option<Rgb>,
        link_colors: Option<Vec<Rgb>>,
    ) -> Option<Vec<Rgb>> {
        if let Some(colors) = link_colors {
            Some(colors)
        } else if let Some(color) = link_color {
            Some(vec![color])
        } else {
            None
        }
    }
    /// Calculates the grid cell positions for a subplot with reserved space for titles.
    ///
    /// This function computes both the sankey diagram domain and annotation position,
    /// ensuring that space is reserved above each diagram for the facet title.
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

        // Reserve title space at the top of the cell
        let title_height = cell_height * TITLE_HEIGHT_RATIO;
        let sankey_y_top = cell_y_top - title_height;

        // Sankey diagram domain (bottom 90% of the cell)
        let sankey_x_start = cell_x_start;
        let sankey_x_end = cell_x_start + cell_width;
        let sankey_y_start = cell_y_bottom;
        let sankey_y_end = sankey_y_top;

        FacetCell {
            domain_x_start: sankey_x_start,
            domain_x_end: sankey_x_end,
            domain_y_start: sankey_y_start,
            domain_y_end: sankey_y_end,
        }
    }

    fn build_label_index<'a>(
        sources: &'a [Option<String>],
        targets: &'a [Option<String>],
    ) -> (Vec<&'a str>, HashMap<&'a str, usize>) {
        let mut label_to_idx: HashMap<&'a str, usize> = HashMap::new();
        let mut labels_unique: Vec<&'a str> = Vec::new();

        let iter = sources
            .iter()
            .chain(targets.iter())
            .filter_map(|opt| opt.as_deref());

        for lbl in iter {
            if let Entry::Vacant(v) = label_to_idx.entry(lbl) {
                let next_id = labels_unique.len();
                v.insert(next_id);
                labels_unique.push(lbl);
            }
        }

        (labels_unique, label_to_idx)
    }

    fn column_to_indices(
        column: &[Option<String>],
        label_to_idx: &HashMap<&str, usize>,
    ) -> Vec<usize> {
        column
            .iter()
            .filter_map(|opt| opt.as_deref())
            .map(|lbl| *label_to_idx.get(lbl).expect("label must exist in map"))
            .collect()
    }
}

impl crate::Plot for SankeyDiagram {
    fn ir_traces(&self) -> &[TraceIR] {
        &self.traces
    }

    fn ir_layout(&self) -> &LayoutIR {
        &self.layout
    }
}
