#![doc = include_str!("../../../README.md")]
#![allow(clippy::needless_doctest_main)]

// Re-export PlotlyExt as Plot for backward compatibility
pub use plotlars_plotly::PlotlyExt as Plot;

// Re-export core Plot trait as PlotData for advanced users
pub use plotlars_core::Plot as PlotData;

// Re-export SubplotGrid from plotly crate
pub use plotlars_plotly::SubplotGrid;

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

// Plot type re-exports
pub use plotlars_core::plots::array2dplot::Array2dPlot;
pub use plotlars_core::plots::barplot::BarPlot;
pub use plotlars_core::plots::boxplot::BoxPlot;
pub use plotlars_core::plots::candlestick::CandlestickPlot;
pub use plotlars_core::plots::contourplot::ContourPlot;
pub use plotlars_core::plots::density_mapbox::DensityMapbox;
pub use plotlars_core::plots::heatmap::HeatMap;
pub use plotlars_core::plots::histogram::Histogram;
pub use plotlars_core::plots::image::Image;
pub use plotlars_core::plots::lineplot::LinePlot;
pub use plotlars_core::plots::mesh3d::Mesh3D;
pub use plotlars_core::plots::ohlc::OhlcPlot;
pub use plotlars_core::plots::piechart::PieChart;
pub use plotlars_core::plots::sankeydiagram::SankeyDiagram;
pub use plotlars_core::plots::scatter3dplot::Scatter3dPlot;
pub use plotlars_core::plots::scattergeo::ScatterGeo;
pub use plotlars_core::plots::scattermap::ScatterMap;
pub use plotlars_core::plots::scatterplot::ScatterPlot;
pub use plotlars_core::plots::scatterpolar::ScatterPolar;
pub use plotlars_core::plots::surfaceplot::SurfacePlot;
pub use plotlars_core::plots::table::Table;
pub use plotlars_core::plots::timeseriesplot::TimeSeriesPlot;
