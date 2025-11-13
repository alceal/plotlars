use bon::bon;
use std::collections::{hash_map::Entry, HashMap};

use plotly::{
    common::{Anchor, Domain},
    layout::Annotation,
    sankey::{Link, Node},
    Layout as LayoutPlotly, Sankey, Trace,
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper, Polar},
    components::{Arrangement, FacetConfig, Legend, Orientation, Rgb, Text},
};

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
/// let dataset = LazyCsvReader::new(PlPath::new("data/sankey_flow.csv"))
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
#[derive(Clone, Serialize)]
pub struct SankeyDiagram {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
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
                );

                let traces = Self::create_traces(
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
                );

                (layout, traces)
            }
        };

        Self { traces, layout }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
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
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

        let trace = Self::create_trace(
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
            None,
        );

        traces.push(trace);
        traces
    }

    #[allow(clippy::too_many_arguments)]
    fn create_trace(
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
        domain: Option<Domain>,
    ) -> Box<dyn Trace + 'static> {
        let sources = Self::get_string_column(data, sources);
        let targets = Self::get_string_column(data, targets);
        let values = Self::get_numeric_column(data, values);

        let (labels_unique, label_to_idx) = Self::build_label_index(&sources, &targets);

        let sources_idx = Self::column_to_indices(&sources, &label_to_idx);
        let targets_idx = Self::column_to_indices(&targets, &label_to_idx);

        let mut node = Node::new().label(labels_unique);

        node = Self::set_pad(node, pad);
        node = Self::set_thickness(node, thickness);
        node = Self::set_node_color(node, node_color);
        node = Self::set_node_colors(node, node_colors);

        let mut link = Link::new()
            .source(sources_idx)
            .target(targets_idx)
            .value(values);

        link = Self::set_link_color(link, link_color);
        link = Self::set_link_colors(link, link_colors);

        let mut trace = Sankey::new().node(node).link(link);

        trace = Self::set_orientation(trace, orientation);
        trace = Self::set_arrangement(trace, arrangement);

        if let Some(domain_val) = domain {
            trace = trace.domain(domain_val);
        }

        trace
    }

    #[allow(clippy::too_many_arguments)]
    fn create_faceted_traces(
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

        let n_facets = facet_categories.len();
        let (ncols, nrows) = Self::calculate_grid_dimensions(n_facets, config.cols, config.rows);

        // Filter out facets with no data to prevent empty diagrams
        let facet_categories_non_empty: Vec<String> = facet_categories
            .iter()
            .filter(|facet_value| {
                let facet_data = Self::filter_data_by_group(data, facet_column, facet_value);
                facet_data.height() > 0
            })
            .cloned()
            .collect();

        let mut all_traces = Vec::new();

        // Need to clone Vec colors for reuse across facets
        let node_colors_cloned = node_colors.clone();
        let link_colors_cloned = link_colors.clone();

        for (idx, facet_value) in facet_categories_non_empty.iter().enumerate() {
            let facet_data = Self::filter_data_by_group(data, facet_column, facet_value);

            let domain =
                Self::calculate_sankey_domain(idx, ncols, nrows, config.h_gap, config.v_gap);

            let trace = Self::create_trace(
                &facet_data,
                sources,
                targets,
                values,
                orientation.clone(),
                arrangement.clone(),
                pad,
                thickness,
                node_color,
                node_colors_cloned.clone(),
                link_color,
                link_colors_cloned.clone(),
                Some(domain),
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

        // Filter out facets with no data
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

        let annotations = Self::create_facet_annotations_sankey(
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
        // Padding ratio creates buffer space above annotation (35% of reserved title space)
        const TITLE_PADDING_RATIO: f64 = 0.35;

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

        // Calculate annotation position with padding buffer
        let padding_height = title_height * TITLE_PADDING_RATIO;
        let actual_title_height = title_height - padding_height;
        let annotation_x = cell_x_start + cell_width / 2.0;
        let annotation_y = sankey_y_top + padding_height + (actual_title_height / 2.0);

        FacetCell {
            domain_x_start: sankey_x_start,
            domain_x_end: sankey_x_end,
            domain_y_start: sankey_y_start,
            domain_y_end: sankey_y_end,
            annotation_x,
            annotation_y,
        }
    }

    fn calculate_sankey_domain(
        subplot_index: usize,
        ncols: usize,
        nrows: usize,
        x_gap: Option<f64>,
        y_gap: Option<f64>,
    ) -> Domain {
        let cell = Self::calculate_facet_cell(subplot_index, ncols, nrows, x_gap, y_gap);
        Domain::new()
            .x(&[cell.domain_x_start, cell.domain_x_end])
            .y(&[cell.domain_y_start, cell.domain_y_end])
    }

    fn create_facet_annotations_sankey(
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

    fn set_thickness(mut node: Node, thickness: Option<usize>) -> Node {
        if let Some(thickness) = thickness {
            node = node.thickness(thickness);
        }

        node
    }

    fn set_pad(mut node: Node, pad: Option<usize>) -> Node {
        if let Some(pad) = pad {
            node = node.pad(pad);
        }

        node
    }

    fn set_link_colors<V>(mut link: Link<V>, colors: Option<Vec<Rgb>>) -> Link<V>
    where
        V: Serialize + Clone,
    {
        if let Some(colors) = colors {
            link = link.color_array(colors.iter().map(|color| color.to_plotly()).collect());
        }

        link
    }

    fn set_link_color<V>(mut link: Link<V>, color: Option<Rgb>) -> Link<V>
    where
        V: Serialize + Clone,
    {
        if let Some(color) = color {
            link = link.color(color);
        }

        link
    }

    fn set_node_colors(mut node: Node, colors: Option<Vec<Rgb>>) -> Node {
        if let Some(colors) = colors {
            node = node.color_array(colors.iter().map(|color| color.to_plotly()).collect());
        }

        node
    }

    fn set_node_color(mut node: Node, color: Option<Rgb>) -> Node {
        if let Some(color) = color {
            node = node.color(color);
        }

        node
    }

    fn set_arrangement(
        mut trace: Box<Sankey<Option<f32>>>,
        arrangement: Option<Arrangement>,
    ) -> Box<Sankey<Option<f32>>> {
        if let Some(arrangement) = arrangement {
            trace = trace.arrangement(arrangement.to_plotly())
        }

        trace
    }

    fn set_orientation(
        mut trace: Box<Sankey<Option<f32>>>,
        orientation: Option<Orientation>,
    ) -> Box<Sankey<Option<f32>>> {
        if let Some(orientation) = orientation {
            trace = trace.orientation(orientation.to_plotly())
        }

        trace
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

/// Helper struct containing calculated positions for a facet cell
struct FacetCell {
    domain_x_start: f64,
    domain_x_end: f64,
    domain_y_start: f64,
    domain_y_end: f64,
    annotation_x: f64,
    annotation_y: f64,
}

impl Layout for SankeyDiagram {}
impl Polar for SankeyDiagram {}

impl PlotHelper for SankeyDiagram {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
