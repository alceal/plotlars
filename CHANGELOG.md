# Changelog

All notable changes to this project will be documented in this file.

## [0.9.0] - 2025-04-27

### 🚀 Features

- Add Contour plot
- Add Sankey diagram
- Add surface plot
- Add the secondary y axis

### 🚜 Refactor

- Remove contours struct

### 📚 Documentation

- Add implemented plots overview with examples to README

### ⚙️ Miscellaneous Tasks

- Bump plotlars version to 0.8.1 in Cargo files
- Update CHANGELOG for version 0.8.1 with new features and documentation
- Update Rust version and edition and bon crate
- Remove fmt hook
- Add fmt
- Format imports
- Refactor dataframe from documentation
- Format code
- Remove empty line
- Update with the new plots
- Update bon
- Update documentation
- Update

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

### ⚙️ Miscellaneous Tasks

- Update dependencies to latest versions
- Add image
- Clean up comments in PieChart builder example for clarity
- Bump plotlars version to 0.8.0 in Cargo files

## [0.7.0] - 2024-11-06

### 🚀 Features

- Add Scatter3dPlot

### 🚜 Refactor

- Move set_axis and set_legend to Axis and Legend modules

### ⚙️ Miscellaneous Tasks

- Update CHANGELOG
- Bump to version 0.6.0
- Update dependencies
- Remove kaleido dependency
- Bump to version 0.7.0

## [0.6.0] - 2024-11-01

### 🚀 Features

- New `axis_position` method and the old one has been renamed to `axis_side` and the corresponding enum values have been updated
- Add HeatMap

### 🚜 Refactor

- [**breaking**] Remove deprecated structures VerticalBarPlot, HorizontalBarPlot, VerticalBoxPlot, HorizontalBoxPlot
- A deep refactoring

### 📚 Documentation

- Update documentation examples
- Update documentation examples
- Remove reference to vertical and horizontal bar/box plots
- Add important note about using GitHub version of plotlars due to polars issue
- Fix github link

### ⚙️ Miscellaneous Tasks

- Update dependencies in Cargo.toml
- Remove the important note
- Remove Polars dtype-categorical feature
- Bump to version 0.5.3
- Refactor code

## [0.5.0] - 2024-09-13

### 🚀 Features

- Add new BarPlot struct with orientation field; deprecate VerticalBarPlot and HorizontalBarPlot
- Update BoxPlot struct to handle both vertical and horizontal box plots
- Add `color` argument
- Customize the shape of the marker
- Add optional shape and add line width for line and time series plots

### ⚙️ Miscellaneous Tasks

- Deprecate VerticalBarPlot and HorizontalBarPlot
- Update documentation
- Update CHANGELOG.md
- Update version to 0.5.0

## [0.4.0] - 2024-09-10

### 🚀 Features

- Add Legend module

### 🚜 Refactor

- Refactor Axis module to use Self instead of Axis in new() method

### ⚙️ Miscellaneous Tasks

- Add examples for Axis module
- Update plotlars dependency to version 0.3.1
- Update documentation
- Update plotlars dependency to version 0.3.2
- Add Jupyter section
- Update plotlars dependency to version 0.3.3
- Update Jupyter section and add image to README
- Update documentation
- Update dependencies
- Update plotlars to version 0.4.0
- Update plotlars to version 0.4.0

## [0.3.0] - 2024-09-01

### 🚀 Features

- Implement From trait for Text to convert from &str and String
- Add plot title position
- Add From trait implementation for Text to convert from &String
- Add axis module for customizing plot axes
- Add write_html method

### ⚙️ Miscellaneous Tasks

- Update patch
- Update marker.rs to use #[doc(hidden)] attribute
- Add data for examples
- Update plotlars library to version 0.2.2
- Update documentation link in Cargo.toml
- Update Polars and Plotly library links in README.md and lib.rs
- Add images to documentation
- Update text
- Update plotlars library to version 0.2.3
- The crate documentation is the README file
- Add .markdownlint.json
- Reformat the text
- Justfile to .gitignore
- Remove justfile
- Fix typo
- Fix typo
- Change field visibility
- Update plotlars dependency to version 0.3.0
- Update bon dependency to version 2.1.0
- Add version 0.3.0 and fix typos

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

### ⚙️ Miscellaneous Tasks

- Update Cargo.toml with keywords and categories for better package discoverability
- Update Cargo.toml with version and dependencies
- Remove unnecessary main.rs file
- Add traces module with various plot types
- Add aesthetics module with line and mark submodules
- Add marker macro for creating and modifying markers
- Add macros module
- Add traits module
- Add Plotlars library for creating visualizations from Polars data frames
- Add .gitignore rules for notebook and main.rs files

### Update

- README.md

<!-- generated by git-cliff -->
