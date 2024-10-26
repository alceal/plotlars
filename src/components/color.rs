use plotly::color::Color;
use serde::Serialize;

/// A structure representing an RGB color with red, green, and blue components.
///
/// # Example
///
/// ```rust
/// use plotlars::{Axis, BarPlot, Legend, Orientation, Plot, Rgb};
///
/// let label = vec!["", "", ""];
/// let color = vec!["red", "green", "blue"];
/// let value = vec![1, 1, 1];
///
/// let df = DataFrame::new(vec![
///     Series::new("label".into(), label),
///     Series::new("color".into(), color),
///     Series::new("value".into(), value),
/// ]).unwrap();
///
/// let axis = Axis::new()
///     .show_axis(false);
///
/// let legend = Legend::new()
///     .orientation(Orientation::Horizontal)
///     .x(0.3);
///
/// BarPlot::builder()
///     .data(&df)
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

impl Color for Rgb {}
