# Changelog

All notable changes to this project will be documented in this file.

## [0.11.4] - 2025-12-02

### 🚀 Features

- Support mixed subplot grids combining different plot types (2D, 3D, polar, geo, mapbox, domain-based)
- Auto-generate legends for subplot grids when not explicitly configured

## [0.11.3] - 2025-12-02

### 🐛 Bug Fixes

- Fix colorbar positioning in subplot grids to avoid overlapping charts
- Fix colorbar height scaling for traces without explicit colorbar configuration

## [0.11.2] - 2025-11-23

### ⚠️ BREAKING CHANGES

- Remove `ndarray` dependency
  - No longer required as a direct dependency
  - Users relying on ndarray integration should add it to their own Cargo.toml
- Remove `plotly_static` dependency
  - Export features now use plotly's built-in static export capabilities
- Rename export features for consistency
  - `static_export_chromedriver` → `export-chrome`
  - `static_export_geckodriver` → `export-firefox`
  - `static_export_wd_download` → `export-download`
  - `static_export_default` → `export-default`

## [0.11.0] - 2025-11-15

### 🚀 Features

- Dimensions: New component for controlling plot sizing with width, height, and auto_size parameters
  - Integrated into all plot types and SubplotGrid
  - Enables precise control over plot dimensions, particularly useful for multi-plot layouts
- SubplotGrid: Create multi-plot grid layouts with automatic positioning and configurable spacing (cartesian 2D plots only)
  - Regular grids with automatic plot arrangement
  - Irregular grids with custom row/column spanning support
- Faceting: Split data by categorical variables across 14 plot types (BarPlot, BoxPlot, ContourPlot, HeatMap, Histogram, LinePlot, Mesh3D, PieChart, SankeyDiagram, Scatter3dPlot, ScatterPlot, ScatterPolar, SurfacePlot, TimeSeriesPlot)
- Custom Axis Title Positioning: Precisely position axis titles anywhere on the plot

### ⚠️ BREAKING CHANGES

- ColorBar `length()` and `width()` now accept fractions (0.0-1.0) instead of pixels

### 🐛 Bug Fixes

- Fix HeatMap colorbar length and width not being applied to plots
- Fix colorbar extending beyond subplot boundaries in irregular SubplotGrid
- Fix custom legend border rendering without explicit border color

## [0.10.0] - 2025-08-07

### 🚀 Features

- Add CandlestickPlot with Direction styling
- Add DensityMapbox plot
- Add Mesh3D plot implementation
- Add OHLC plot for financial data visualization
- Add ScatterGeo plot with geographic visualization support
- Add ScatterPolar plot implementation
- Add Table plot with Header and Cell components

## [0.9.7] - 2025-08-04

### 🐛 Bug Fixes

- Update dependencies and fix Polars 0.50.0 compatibility

## [0.9.6] - 2025-08-04

### 🚀 Features

- Re-enable write_image with plotly 0.13 (incl features)

### 🐛 Bug Fixes

- Fix incorrect import in the README example (Text unused; Rgb missing)

## [0.9.5] - 2025-07-05

### 📚 Documentation

- Update dependencies to latest versions

## [0.9.4] - 2025-05-29

### 🐛 Bug Fixes

- Update lineplot.rs to use Column instead of Series
- Update polars version

### 📚 Documentation

- Add dependency on plotters in LinePlot doc

## [0.9.3] - 2025-05-17

### 🐛 Bug Fixes

- A workaround fix for polars 0.47.1

## [0.9.1] - 2025-05-02

### 🐛 Bug Fixes

- Several time series with only one y axis

### 📚 Documentation

- Add another example

## [0.9.0] - 2025-05-02

### 🚀 Features

- Add Contour plot
- Add Sankey diagram
- Add surface plot
- Add the secondary y axis

### 📚 Documentation

- Add implemented plots overview with examples to README

### Feat

- Additional trait methods to provide html string

## [0.8.0] - 2025-01-05

### 🚀 Features

- Convert plots into JSON with the `to_json`method
- Add Image plot support for visualizing raster data
- Add PieChart support for visualizing categorical data
- Add Array2DPlot for visualizing 2D arrays of RGB color values
- Add ScatterMap for visualizing geographical data points on an interactive map

### 🐛 Bug Fixes

- Rename Array2DPlot to Array2dPlot for consistency

## [0.7.0] - 2024-11-06

### 🚀 Features

- Add Scatter3dPlot

## [0.6.0] - 2024-11-01

### 🚀 Features

- New `axis_position` method and the old one has been renamed to `axis_side` and the corresponding enum values have been updated
- Add HeatMap

### 📚 Documentation

- Update documentation examples
- Update documentation examples
- Remove reference to vertical and horizontal bar/box plots
- Add important note about using GitHub version of plotlars due to polars issue
- Fix github link

## [0.5.0] - 2024-09-13

### 🚀 Features

- Add new BarPlot struct with orientation field; deprecate VerticalBarPlot and HorizontalBarPlot
- Update BoxPlot struct to handle both vertical and horizontal box plots
- Add `color` argument
- Customize the shape of the marker
- Add optional shape and add line width for line and time series plots

## [0.4.0] - 2024-09-10

### 🚀 Features

- Add Legend module

## [0.3.0] - 2024-09-01

### 🚀 Features

- Implement From trait for Text to convert from &str and String
- Add plot title position
- Add From trait implementation for Text to convert from &String
- Add axis module for customizing plot axes
- Add write_html method

### Chote

- Update features

### Update

- CHANGELOG.md
- Add link to data and fix text
- Update dataset path
- Update Changelog

## [0.2.0] - 2024-08-25

### 🚀 Features

- Add Bar Plot
- Add Box Plot
- Add Histogram plot
- Add Line Plot
- Add Scatter Plot
- Add Time Series Plot
- Add Text module with customizable content, font, size, and color
- Add Rgb struct for representing RGB colors
- Add Mark trait for creating and modifying markers
- Add LineType enum for representing different styles of lines and Line trait
- Add Layout trait for creating Plotly layouts
- Add Plot trait for displaying and rendering generic plots
- Add Trace trait for creating and modifying traces
- Add Polar trait for working with polars dataframes
- Add plot example to README.md

### Update

- README.md
