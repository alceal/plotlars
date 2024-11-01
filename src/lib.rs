#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

mod common;
mod components;
mod plots;

pub use crate::common::plot::Plot;
pub use crate::components::axis::{Axis, AxisSide, AxisType};
pub use crate::components::color::Rgb;
pub use crate::components::exponent::ValueExponent;
pub use crate::components::legend::Legend;
pub use crate::components::line::Line;
pub use crate::components::orientation::Orientation;
pub use crate::components::shape::Shape;
pub use crate::components::text::Text;
pub use crate::components::tick::TickDirection;
pub use crate::plots::barplot::BarPlot;
pub use crate::plots::boxplot::BoxPlot;
pub use crate::plots::histogram::Histogram;
pub use crate::plots::lineplot::LinePlot;
pub use crate::plots::scatterplot::ScatterPlot;
pub use crate::plots::timeseriesplot::TimeSeriesPlot;
