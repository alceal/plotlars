use plotly::common::Orientation as OrientationPlotly;

/// An enumeration representing the orientation of the legend.
///
/// # Example
///
/// ```rust
/// use plotlars::{BarPlot, Legend, Orientation, Plot, Rgb};
///
/// let dataset = df![
///         "animal" => &["giraffe", "giraffe", "orangutan", "orangutan", "monkey", "monkey"],
///         "gender" => &vec!["female", "male", "female", "male", "female", "male"],
///         "value" => &vec![20.0f32, 25.0, 14.0, 18.0, 23.0, 31.0],
///         "error" => &vec![1.0, 0.5, 1.5, 1.0, 0.5, 1.5],
///     ]
///     .unwrap();
///
/// let legend = Legend::new()
///     .orientation(Orientation::Horizontal)
///     .y(1.1)
///     .x(0.3);
///
/// BarPlot::builder()
///     .data(&dataset)
///     .labels("animal")
///     .values("value")
///     .orientation(Orientation::Horizontal)
///     .group("gender")
///     .error("error")
///     .colors(vec![Rgb(255, 127, 80), Rgb(64, 224, 208)])
///     .legend(&legend)
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/6kspyX7.png)
#[derive(Clone)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    pub(crate) fn to_plotly(&self) -> OrientationPlotly {
        match self {
            Self::Horizontal => OrientationPlotly::Horizontal,
            Self::Vertical => OrientationPlotly::Vertical,
        }
    }
}
