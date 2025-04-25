use plotly::sankey::Arrangement as ArrangementPlotly;

/// An enumeration representing node arrangement strategies for Sankey diagrams.
///
/// The `Arrangement` enum controls how nodes are positioned relative to each other:
///
/// * `Snap` — If value is `snap` (the default), the node arrangement is assisted by
///   automatic snapping of elements to preserve space between nodes specified via `nodepad`.
/// * `Perpendicular` — Nodes can only move along a line perpendicular to the primary flow.
/// * `Freeform` — Nodes can freely move anywhere on the plane without automatic constraints.
/// * `Fixed` — Nodes remain stationary at their specified positions and are not adjusted by the layout algorithm.
///
/// # Example
///
/// ```rust
/// use plotlars::{Arrangement, SankeyDiagram, Orientation, Plot, Rgb, Text};
/// use polars::prelude::*;
///
/// let dataset = df![
///     "source" => &["A1", "A2", "A1", "B1", "B2", "B2"],
///     "target" => &["B1", "B2", "B2", "C1", "C1", "C2"],
///     "value"  => &[8, 4, 2, 8, 4, 2],
/// ].unwrap();
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
///     .build()
///     .plot();
/// ```
///
/// ![Example Sankey Diagram](https://imgur.com/oCvuAZB.png)
pub enum Arrangement {
    Snap,
    Perpendicular,
    Freeform,
    Fixed,
}

impl Arrangement {
    pub(crate) fn to_plotly(&self) -> ArrangementPlotly {
        match self {
            Arrangement::Snap => ArrangementPlotly::Snap,
            Arrangement::Perpendicular => ArrangementPlotly::Perpendicular,
            Arrangement::Freeform => ArrangementPlotly::Freeform,
            Arrangement::Fixed => ArrangementPlotly::Fixed,
        }
    }
}
