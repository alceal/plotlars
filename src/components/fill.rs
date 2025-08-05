use plotly::common::Fill as PlotlyFill;

/// An enumeration representing different fill modes for area traces in plots.
///
/// The `Fill` enum specifies how the area under or between traces should be filled
/// in plots like scatter plots, line plots, and polar scatter plots.
///
/// # Example
///
/// ```rust
/// use plotlars::{Fill, Mode, Plot, Rgb, ScatterPolar, Text};
/// use polars::prelude::*;
///
/// let angles: Vec<f64> = (0..=360).step_by(10).map(|x| x as f64).collect();
/// let radii: Vec<f64> = angles.iter()
///     .map(|&angle| 5.0 + 3.0 * (angle * std::f64::consts::PI / 180.0).sin())
///     .collect();
///
/// let dataset = DataFrame::new(vec![
///     Column::new("angle".into(), angles),
///     Column::new("radius".into(), radii),
/// ])
/// .unwrap();
///
/// ScatterPolar::builder()
///     .data(&dataset)
///     .theta("angle")
///     .r("radius")
///     .mode(Mode::Lines)
///     .fill(Fill::ToSelf)  // Fill the area enclosed by the trace
///     .color(Rgb(135, 206, 250))
///     .opacity(0.6)
///     .plot_title(Text::from("Filled Polar Area Chart"))
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/0QAmKuS.png)
#[derive(Clone, Copy)]
pub enum Fill {
    /// Fill area from the trace to y=0 (horizontal axis)
    ToZeroY,
    /// Fill area from the trace to x=0 (vertical axis)
    ToZeroX,
    /// Fill area between this trace and the next trace along the y-direction
    ToNextY,
    /// Fill area between this trace and the next trace along the x-direction
    ToNextX,
    /// Fill the area enclosed by the trace (connecting the last point to the first)
    ToSelf,
    /// Fill area between this trace and the next trace
    ToNext,
    /// Do not fill any area
    None,
}

impl Fill {
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn to_plotly(&self) -> PlotlyFill {
        match self {
            Fill::ToZeroY => PlotlyFill::ToZeroY,
            Fill::ToZeroX => PlotlyFill::ToZeroX,
            Fill::ToNextY => PlotlyFill::ToNextY,
            Fill::ToNextX => PlotlyFill::ToNextX,
            Fill::ToSelf => PlotlyFill::ToSelf,
            Fill::ToNext => PlotlyFill::ToNext,
            Fill::None => PlotlyFill::None,
        }
    }
}
