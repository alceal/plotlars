use bon::bon;
use std::collections::{
    HashMap,
    hash_map::Entry,
};

use plotly::{
    sankey::{Link, Node}, Layout as LayoutPlotly, Sankey, Trace
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper, Polar},
    components::{Arrangement, Orientation, Rgb, Text},
};

/// A structure representing a Sankey diagram.
///
/// The `SankeyDiagram` struct enables the creation of Sankey diagrams, which visualize flows
/// between discrete nodes with link widths proportional to the magnitude of the flow. It
/// offers extensive configuration options for flow orientation, node arrangement, spacing,
/// thickness, and coloring, as well as axis and title customization. Users can specify a
/// single uniform color or per-item colors for both nodes and links, adjust padding between
/// nodes, set node thickness, and supply custom titles and axis labels to produce clear,
/// publication-quality flow visualizations.
///
/// # Arguments
///
/// * `data` – A reference to the `DataFrame` containing the data to be plotted.
/// * `sources` – A string slice naming the column in `data` that contains the source node for each flow.
/// * `targets` – A string slice naming the column in `data` that contains the target node for each flow.
/// * `values` – A string slice naming the column in `data` that contains the numeric value of each flow.
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
///
/// # Example
///
/// ```rust
/// use plotlars::{Arrangement, SankeyDiagram, Orientation, Plot, Rgb, Text};
///
/// let dataset = df![
///         "source" => ["A1", "A2", "A1", "B1", "B2", "B2"],
///         "target" => &["B1", "B2", "B2", "C1", "C1", "C2"],
///         "value" => &[8, 4, 2, 8, 4, 2],
///     ]
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
        orientation: Option<Orientation>,
        arrangement: Option<Arrangement>,
        pad: Option<usize>,
        thickness: Option<usize>,
        node_color: Option<Rgb>,
        node_colors: Option<Vec<Rgb>>,
        link_color: Option<Rgb>,
        link_colors: Option<Vec<Rgb>>,
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
    ) -> Box<dyn Trace + 'static> {
        let sources = Self::get_string_column(data, sources);
        let targets = Self::get_string_column(data, targets);
        let values = Self::get_numeric_column(data, values);

        let (labels_unique, label_to_idx) = Self::build_label_index(&sources, &targets);

        let sources_idx = Self::column_to_indices(&sources, &label_to_idx);
        let targets_idx = Self::column_to_indices(&targets, &label_to_idx);

        let mut node = Node::new()
            .label(labels_unique);

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

        let mut trace = Sankey::new()
            .node(node)
            .link(link);

        trace = Self::set_orientation(trace, orientation);
        trace = Self::set_arrangement(trace, arrangement);
        trace
    }

    fn set_thickness(
        mut node: Node,
        thickness: Option<usize>,
    ) -> Node     {
        if let Some(thickness) = thickness {
            node = node.thickness(thickness);
        }

        node
    }

    fn set_pad(
        mut node: Node,
        pad: Option<usize>,
    ) -> Node     {
        if let Some(pad) = pad {
            node = node.pad(pad);
        }

        node
    }

    fn set_link_colors<V>(
        mut link: Link<V>,
        colors: Option<Vec<Rgb>>,
    ) -> Link<V>
    where
        V: Serialize + Clone,
    {
        if let Some(colors) = colors {
            link = link.color_array(
                colors
                    .iter()
                    .map(|color| color.to_plotly())
                    .collect()
            );
        }

        link
    }

    fn set_link_color<V>(
        mut link: Link<V>,
        color: Option<Rgb>,
    ) -> Link<V>
    where
        V: Serialize + Clone,
    {
        if let Some(color) = color {
            link = link.color(color);
        }

        link
    }

    fn set_node_colors(
        mut node: Node,
        colors: Option<Vec<Rgb>>,
    ) -> Node {
        if let Some(colors) = colors {
            node = node.color_array(
                colors
                    .iter()
                    .map(|color| color.to_plotly())
                    .collect()
            );
        }

        node
    }

    fn set_node_color(
        mut node: Node,
        color: Option<Rgb>,
    ) -> Node {
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
