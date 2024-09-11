#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

#[macro_use]
mod macros;

mod aesthetics;
mod axis;
mod colors;
mod legend;
mod texts;
mod traces;
mod traits;

pub use crate::aesthetics::{line::LineType, orientation::Orientation};
pub use crate::axis::{Axis, AxisPosition, AxisType, TickDirection, ValueExponent};
pub use crate::colors::Rgb;
pub use crate::legend::Legend;
pub use crate::texts::Text;
// pub use crate::traces::barplot::{BarPlot, HorizontalBarPlot, VerticalBarPlot};
pub use crate::traces::barplot::BarPlot;
pub use crate::traces::boxplot::{HorizontalBoxPlot, VerticalBoxPlot};
pub use crate::traces::histogram::Histogram;
pub use crate::traces::lineplot::LinePlot;
pub use crate::traces::scatterplot::ScatterPlot;
pub use crate::traces::timeseriesplot::TimeSeriesPlot;
pub use crate::traits::plot::Plot;
