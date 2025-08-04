use plotly::color::{Color, Rgb as RgbPlotly};

use serde::Serialize;

/// A structure representing an RGB color with red, green, and blue components.
///
/// # Example
///
/// ```rust
/// use plotlars::{Axis, BarPlot, Legend, Orientation, Plot, Rgb};
///
/// let dataset = df![
///         "label" => &["", "", ""],
///         "color" => &["red", "green", "blue"],
///         "value" => &[1, 1, 1],
///     ]
///     .unwrap();
///
/// let axis = Axis::new()
///     .show_axis(false);
///
/// let legend = Legend::new()
///     .orientation(Orientation::Horizontal)
///     .x(0.3);
///
/// BarPlot::builder()
///     .data(&dataset)
///     .labels("label")
///     .values("value")
///     .group("color")
///     .colors(vec![
///         Rgb(255, 0, 0),
///         Rgb(0, 255, 0),
///         Rgb(0, 0, 255),
///     ])
///     .x_axis(&axis)
///     .y_axis(&axis)
///     .legend(&legend)
///     .build()
///     .plot();
/// ```
/// ![example](https://imgur.com/HPmtj9I.png)
#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct Rgb(
    /// Red component
    pub u8,
    /// Green component
    pub u8,
    /// Blue component
    pub u8,
);

impl Rgb {
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn to_plotly(&self) -> RgbPlotly {
        RgbPlotly::new(self.0, self.1, self.2)
    }
}

impl Color for Rgb {}
