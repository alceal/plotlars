use plotly::layout::BarMode as BarModePlotly;

/// An enumeration representing how bars are displayed when multiple bar traces share the same axis.
///
/// # Example
///
/// ```rust
/// use plotlars::{BarMode, BarPlot, Plot};
/// use polars::prelude::*;
///
/// let dataset = df![
///         "animal" => &["giraffe", "giraffe", "orangutan", "orangutan", "monkey", "monkey"],
///         "gender" => &["female", "male", "female", "male", "female", "male"],
///         "value" => &[20.0f32, 25.0, 14.0, 18.0, 23.0, 31.0],
///     ]
///     .unwrap();
///
/// BarPlot::builder()
///     .data(&dataset)
///     .labels("animal")
///     .values("value")
///     .group("gender")
///     .mode(BarMode::Stack)
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/HQQvQey.png)
#[derive(Clone)]
pub enum BarMode {
    Stack,
    Group,
    Overlay,
    Relative,
}

impl BarMode {
    pub(crate) fn to_plotly(&self) -> BarModePlotly {
        match self {
            Self::Stack => BarModePlotly::Stack,
            Self::Group => BarModePlotly::Group,
            Self::Overlay => BarModePlotly::Overlay,
            Self::Relative => BarModePlotly::Relative,
        }
    }
}
