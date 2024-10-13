#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main, deprecated)]

#[macro_use]
mod macros;

mod aesthetics;
mod axis;
mod colors;
mod legends;
mod shapes;
mod texts;
mod traces;
mod traits;

pub use crate::aesthetics::{line::LineType, orientation::Orientation};
pub use crate::axis::{Axis, AxisSide, AxisType, TickDirection, ValueExponent};
pub use crate::colors::Rgb;
pub use crate::legends::colorbar::ColorBar;
pub use crate::legends::legend::Legend;
pub use crate::shapes::Shape;
pub use crate::texts::Text;
pub use crate::traces::barplot::BarPlot;
pub use crate::traces::boxplot::BoxPlot;
pub use crate::traces::heatmap::HeatMap;
pub use crate::traces::histogram::Histogram;
pub use crate::traces::lineplot::LinePlot;
pub use crate::traces::scatterplot::ScatterPlot;
pub use crate::traces::timeseriesplot::TimeSeriesPlot;
pub use crate::traits::plot::Plot;
