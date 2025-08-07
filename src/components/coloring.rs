use plotly::contour::Coloring as ColoringPlotly;

/// Enumeration representing the coloring strategy applied to contour levels.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Coloring, ContourPlot, Palette, Plot};
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
///     .coloring(Coloring::Lines)
///     .color_scale(Palette::Viridis)
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/hFD2A82.png)
pub enum Coloring {
    Fill,
    HeatMap,
    Lines,
    None,
}

impl Coloring {
    pub(crate) fn to_plotly(&self) -> ColoringPlotly {
        match self {
            Coloring::Fill => ColoringPlotly::Fill,
            Coloring::HeatMap => ColoringPlotly::HeatMap,
            Coloring::Lines => ColoringPlotly::Lines,
            Coloring::None => ColoringPlotly::None,
        }
    }
}
