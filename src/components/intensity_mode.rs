use plotly::traces::mesh3d::IntensityMode as PlotlyIntensityMode;

/// An enumeration representing the source of intensity values for mesh coloring.
///
/// The `IntensityMode` enum specifies whether intensity values should be taken
/// from vertices or cells (faces) of a 3D mesh.
///
/// # Example
///
/// ```rust
/// use plotlars::{ColorBar, IntensityMode, Mesh3D, Palette, Plot};
/// use polars::prelude::*;
///
/// // Create sample mesh data with intensity values
/// let x = vec![0.0, 1.0, 2.0, 0.0];
/// let y = vec![0.0, 0.0, 1.0, 2.0];
/// let z = vec![0.0, 2.0, 0.0, 1.0];
/// let intensity = vec![0.0, 0.5, 0.8, 1.0];
///
/// let dataset = DataFrame::new(vec![
///     Column::new("x".into(), x),
///     Column::new("y".into(), y),
///     Column::new("z".into(), z),
///     Column::new("intensity".into(), intensity),
/// ])
/// .unwrap();
///
/// Mesh3D::builder()
///     .data(&dataset)
///     .x("x")
///     .y("y")
///     .z("z")
///     .intensity("intensity")
///     .intensity_mode(IntensityMode::Vertex)
///     .color_scale(Palette::Viridis)
///     .color_bar(
///         &ColorBar::new()
///             .x(0.65)  // Position color bar extremely close to the plot
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/J4qIyU2.png)
#[derive(Clone, Copy)]
pub enum IntensityMode {
    /// Use intensity values from mesh vertices
    Vertex,
    /// Use intensity values from mesh cells (faces)
    Cell,
}

impl IntensityMode {
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn to_plotly(&self) -> PlotlyIntensityMode {
        match self {
            IntensityMode::Vertex => PlotlyIntensityMode::Vertex,
            IntensityMode::Cell => PlotlyIntensityMode::Cell,
        }
    }
}
