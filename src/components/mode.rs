use plotly::common::Mode as PlotlyMode;

/// An enumeration representing different drawing modes for scatter-type plots.
///
/// The `Mode` enum specifies how data points should be displayed in plots like
/// scatter plots, line plots, and polar scatter plots.
///
/// # Example
///
/// ```rust
/// use plotlars::{Line, Mode, Plot, Rgb, ScatterPolar, Shape, Text};
/// use polars::prelude::*;
///
/// // Create sample data - radar chart style
/// let categories = vec![0., 72., 144., 216., 288., 360.];
/// let performance = vec![8.0, 6.5, 7.0, 9.0, 5.5, 8.0];
///
/// let dataset = DataFrame::new(vec![
///     Column::new("category".into(), categories),
///     Column::new("performance".into(), performance),
/// ])
/// .unwrap();
///
/// ScatterPolar::builder()
///     .data(&dataset)
///     .theta("category")
///     .r("performance")
///     .mode(Mode::LinesMarkers)
///     .color(Rgb(255, 0, 0))
///     .shape(Shape::Diamond)
///     .line(Line::Solid)
///     .width(3.0)
///     .size(12)
///     .opacity(0.8)
///     .plot_title(
///         Text::from("Performance Radar Chart")
///             .font("Arial")
///             .size(22)
///             .x(0.5)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/PKDr2RJ.png)
#[derive(Clone, Copy)]
pub enum Mode {
    /// Draw only lines connecting the data points
    Lines,
    /// Draw only markers at each data point
    Markers,
    /// Draw only text labels at each data point
    Text,
    /// Draw both lines and markers
    LinesMarkers,
    /// Draw both lines and text labels
    LinesText,
    /// Draw both markers and text labels
    MarkersText,
    /// Draw lines, markers, and text labels
    LinesMarkersText,
    /// Do not draw any visual elements (useful for invisible traces)
    None,
}

impl Mode {
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn to_plotly(&self) -> PlotlyMode {
        match self {
            Mode::Lines => PlotlyMode::Lines,
            Mode::Markers => PlotlyMode::Markers,
            Mode::Text => PlotlyMode::Text,
            Mode::LinesMarkers => PlotlyMode::LinesMarkers,
            Mode::LinesText => PlotlyMode::LinesText,
            Mode::MarkersText => PlotlyMode::MarkersText,
            Mode::LinesMarkersText => PlotlyMode::LinesMarkersText,
            Mode::None => PlotlyMode::None,
        }
    }
}
