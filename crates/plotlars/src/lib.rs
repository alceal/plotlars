#![doc = include_str!("../../../README.md")]
#![allow(clippy::needless_doctest_main)]

// ── Section 1: Compile-time guards ──────────────────────────────────────

#[cfg(all(feature = "plotly", feature = "plotters"))]
compile_error!(
    "Only one plotlars backend can be enabled at a time. \
     Choose either features = [\"plotly\"] or features = [\"plotters\"]."
);

#[cfg(not(any(feature = "plotly", feature = "plotters")))]
compile_error!(
    "A plotlars backend must be enabled. \
     Add features = [\"plotly\"] or features = [\"plotters\"] to your Cargo.toml."
);

// ── Section 2: Unconditional core re-exports ────────────────────────────

pub use plotlars_core::policy::{set_unsupported_option_policy, UnsupportedOptionPolicy};
pub use plotlars_core::Plot as PlotData;

// I/O re-exports
pub use plotlars_core::io::{CsvReader, ParquetReader, PlotlarsError};

#[cfg(feature = "format-json")]
pub use plotlars_core::io::JsonReader;

#[cfg(feature = "format-excel")]
pub use plotlars_core::io::ExcelReader;

/// Re-exported Polars prelude for advanced data manipulation.
///
/// Users who need to filter, cast, join, or otherwise transform DataFrames
/// can import from here instead of adding `polars` to their `Cargo.toml`:
///
/// ```rust
/// use plotlars::polars::prelude::*;
/// ```
pub mod polars {
    pub use polars::prelude;
}

// Component re-exports
pub use plotlars_core::components::arrangement::Arrangement;
pub use plotlars_core::components::axis::{Axis, AxisSide, AxisType};
pub use plotlars_core::components::bar_mode::BarMode;
pub use plotlars_core::components::cell::Cell;
pub use plotlars_core::components::color::Rgb;
pub use plotlars_core::components::colorbar::ColorBar;
pub use plotlars_core::components::coloring::Coloring;
pub use plotlars_core::components::dimensions::Dimensions;
pub use plotlars_core::components::direction::Direction;
pub use plotlars_core::components::exponent::ValueExponent;
pub use plotlars_core::components::facet::{FacetConfig, FacetScales};
pub use plotlars_core::components::fill::Fill;
pub use plotlars_core::components::header::Header;
pub use plotlars_core::components::intensity::IntensityMode;
pub use plotlars_core::components::legend::Legend;
pub use plotlars_core::components::lighting::Lighting;
pub use plotlars_core::components::line::Line;
pub use plotlars_core::components::mode::Mode;
pub use plotlars_core::components::orientation::Orientation;
pub use plotlars_core::components::palette::Palette;
pub use plotlars_core::components::shape::Shape;
pub use plotlars_core::components::text::Text;
pub use plotlars_core::components::tick::TickDirection;

// ── Section 3a: Plotly backend ──────────────────────────────────────────

#[cfg(feature = "plotly")]
pub use plotlars_plotly::PlotlyExt as Plot;

#[cfg(feature = "plotly")]
pub use plotlars_plotly::SubplotGrid;

#[cfg(feature = "plotly")]
pub use plotlars_core::plots::array2dplot::Array2dPlot;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::barplot::BarPlot;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::boxplot::BoxPlot;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::candlestick::CandlestickPlot;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::contourplot::ContourPlot;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::density_mapbox::DensityMapbox;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::heatmap::HeatMap;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::histogram::Histogram;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::image::Image;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::lineplot::LinePlot;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::mesh3d::Mesh3D;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::ohlc::OhlcPlot;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::piechart::PieChart;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::sankeydiagram::SankeyDiagram;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::scatter3dplot::Scatter3dPlot;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::scattergeo::ScatterGeo;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::scattermap::ScatterMap;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::scatterplot::ScatterPlot;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::scatterpolar::ScatterPolar;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::surfaceplot::SurfacePlot;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::table::Table;
#[cfg(feature = "plotly")]
pub use plotlars_core::plots::timeseriesplot::TimeSeriesPlot;

// ── Section 3b: Plotters backend ────────────────────────────────────────

#[cfg(feature = "plotters")]
pub use plotlars_plotters::PlottersExt as Plot;

#[cfg(feature = "plotters")]
pub use plotlars_core::plots::barplot::BarPlot;
#[cfg(feature = "plotters")]
pub use plotlars_core::plots::boxplot::BoxPlot;
#[cfg(feature = "plotters")]
pub use plotlars_core::plots::candlestick::CandlestickPlot;
#[cfg(feature = "plotters")]
pub use plotlars_core::plots::heatmap::HeatMap;
#[cfg(feature = "plotters")]
pub use plotlars_core::plots::histogram::Histogram;
#[cfg(feature = "plotters")]
pub use plotlars_core::plots::lineplot::LinePlot;
#[cfg(feature = "plotters")]
pub use plotlars_core::plots::scatterplot::ScatterPlot;
#[cfg(feature = "plotters")]
pub use plotlars_core::plots::timeseriesplot::TimeSeriesPlot;
