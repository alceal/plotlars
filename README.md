# Plotlars

<p align="center">
    <a href="https://crates.io/crates/plotlars">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/plotlars.svg"></a>
    <a href="https://docs.rs/plotlars">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/plotlars">
    </a>
    <a href="https://crates.io/crates/plotlars">
    <img alt="Downloads" src="https://img.shields.io/crates/d/plotlars">
    </a>
    <a href="https://github.com/your-repo/plotlars/blob/main/LICENSE">
    <img alt="License" src="https://img.shields.io/badge/license-MIT-blue.svg">
    </a>
</p>

Plotlars is a versatile Rust library that acts as a wrapper around the Plotly
crate, bridging the gap between the powerful Polars data analysis library and
Plotly. It simplifies the process of creating visualizations from data frames,
allowing developers to focus on data insights rather than the intricacies of
plot creation.

## Implemented Plots Overview

<table>
  <tr>
    <td align="center">
      <img src="https://imgur.com/LMrqAaT.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.Array2dPlot.html">Array 2D</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/HQQvQey.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.BarPlot.html">Bar Plot</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/jdA3g9r.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.BoxPlot.html">Box Plot</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/91y2Kis.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.CandlestickPlot.html">Candlestick</a>
    </td>
  </tr>
  <tr>
    <td align="center">
      <img src="https://imgur.com/VWgxHC8.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.ContourPlot.html">Contour Plot</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/82eLyBm.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.DensityMapbox.html">Density Mapbox</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/5uFih4M.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.HeatMap.html">Heat Map</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/w2oiuIo.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.Histogram.html">Histogram</a>
    </td>
  </tr>
  <tr>
    <td align="center">
      <img src="https://imgur.com/PAtdaHj.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.Image.html">Image</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/PaXG300.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.LinePlot.html">Line Plot</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/bljzmw5.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.Mesh3D.html">Mesh3D</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/Sv8r9VN.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.OhlcPlot.html">OHLC</a>
    </td>
  </tr>
  <tr>
    <td align="center">
      <img src="https://imgur.com/q44HDwT.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.PieChart.html">Pie Chart</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/jvAew8u.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.SankeyDiagram.html">Sankey Diagram</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/WYTQxHA.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.Scatter3dPlot.html">Scatter 3D Plot</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/8PCEbhN.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.ScatterGeo.html">Scatter Geo</a>
    </td>
  </tr>
  <tr>
    <td align="center">
      <img src="https://imgur.com/8MCjVOd.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.ScatterMap.html">Scatter Map</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/9jfO8RU.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.ScatterPlot.html">Scatter Plot</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/kl1pY9c.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.ScatterPolar.html">Scatter Polar</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/RvZwv3O.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.SubplotGrid.html">Subplot Grid Irregular</a>
    </td>
  </tr>
  <tr>
    <td align="center">
      <img src="https://imgur.com/q0K7cyP.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.SubplotGrid.html">Subplot Grid Regular</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/tdVte4l.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.SurfacePlot.html">Surface Plot</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/QDKTeFX.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.Table.html">Table</a>
    </td>
    <td align="center">
      <img src="https://imgur.com/hL27Xcn.png" width="120" height="120"><br>
      <a href="https://docs.rs/plotlars/latest/plotlars/struct.TimeSeriesPlot.html">Time Series</a>
    </td>
  </tr>
</table>

### Plot Types Reference

| Plot Type | Required Params | Facet | Group |
|---|---|---|---|
| [Array2dPlot](https://docs.rs/plotlars/latest/plotlars/struct.Array2dPlot.html) | data | -- | -- |
| [BarPlot](https://docs.rs/plotlars/latest/plotlars/struct.BarPlot.html) | data, labels, values | Yes | Yes |
| [BoxPlot](https://docs.rs/plotlars/latest/plotlars/struct.BoxPlot.html) | data, labels, values | Yes | Yes |
| [CandlestickPlot](https://docs.rs/plotlars/latest/plotlars/struct.CandlestickPlot.html) | data, dates, open, high, low, close | -- | -- |
| [ContourPlot](https://docs.rs/plotlars/latest/plotlars/struct.ContourPlot.html) | data, x, y, z | Yes | -- |
| [DensityMapbox](https://docs.rs/plotlars/latest/plotlars/struct.DensityMapbox.html) | data, lat, lon, z | -- | -- |
| [HeatMap](https://docs.rs/plotlars/latest/plotlars/struct.HeatMap.html) | data, x, y, z | Yes | -- |
| [Histogram](https://docs.rs/plotlars/latest/plotlars/struct.Histogram.html) | data, x | Yes | Yes |
| [Image](https://docs.rs/plotlars/latest/plotlars/struct.Image.html) | path | -- | -- |
| [LinePlot](https://docs.rs/plotlars/latest/plotlars/struct.LinePlot.html) | data, x, y | Yes | -- |
| [Mesh3D](https://docs.rs/plotlars/latest/plotlars/struct.Mesh3D.html) | data, x, y, z | Yes | -- |
| [OhlcPlot](https://docs.rs/plotlars/latest/plotlars/struct.OhlcPlot.html) | data, dates, open, high, low, close | -- | -- |
| [PieChart](https://docs.rs/plotlars/latest/plotlars/struct.PieChart.html) | data, labels | Yes | -- |
| [SankeyDiagram](https://docs.rs/plotlars/latest/plotlars/struct.SankeyDiagram.html) | data, sources, targets, values | Yes | -- |
| [Scatter3dPlot](https://docs.rs/plotlars/latest/plotlars/struct.Scatter3dPlot.html) | data, x, y, z | Yes | Yes |
| [ScatterGeo](https://docs.rs/plotlars/latest/plotlars/struct.ScatterGeo.html) | data, lat, lon | -- | Yes |
| [ScatterMap](https://docs.rs/plotlars/latest/plotlars/struct.ScatterMap.html) | data, latitude, longitude | -- | Yes |
| [ScatterPlot](https://docs.rs/plotlars/latest/plotlars/struct.ScatterPlot.html) | data, x, y | Yes | Yes |
| [ScatterPolar](https://docs.rs/plotlars/latest/plotlars/struct.ScatterPolar.html) | data, theta, r | Yes | Yes |
| [SubplotGrid](https://docs.rs/plotlars/latest/plotlars/struct.SubplotGrid.html) | plots | -- | -- |
| [SurfacePlot](https://docs.rs/plotlars/latest/plotlars/struct.SurfacePlot.html) | data, x, y, z | Yes | -- |
| [Table](https://docs.rs/plotlars/latest/plotlars/struct.Table.html) | data, columns | -- | -- |
| [TimeSeriesPlot](https://docs.rs/plotlars/latest/plotlars/struct.TimeSeriesPlot.html) | data, x, y | Yes | -- |

## Motivation

The creation of Plotlars was driven by the need to simplify the process of
creating complex plots in Rust, particularly when working with the powerful
Polars data manipulation library. Generating visualizations often requires
extensive boilerplate code and deep knowledge of both the plotting library
(Plotly) and the data structure. This complexity can be a significant hurdle,
especially for users who need to focus on analyzing and interpreting data rather
 than wrestling with intricate plotting logic.

To illustrate this, consider the following example where a scatter plot is
created **without Plotlars**:

```rust
use plotly::{
    common::*,
    layout::*,
    Plot,
    Scatter,
};

use polars::prelude::*;

fn main() {
    let dataset = LazyCsvReader::new(PlRefPath::new("data/penguins.csv"))
        .finish().unwrap()
        .select([
            col("species"),
            col("flipper_length_mm").cast(DataType::Int16),
            col("body_mass_g").cast(DataType::Int16),
        ])
        .collect().unwrap();

    let group_column = "species";
    let x = "body_mass_g";
    let y = "flipper_length_mm";

    let groups = dataset
        .column(group_column).unwrap()
        .unique().unwrap();

    let layout = Layout::new()
        .title(Title::with_text("Penguin Flipper Length vs Body Mass"))
        .x_axis(Axis::new().title(Title::with_text("Body Mass (g)")))
        .y_axis(Axis::new().title(Title::with_text("Flipper Length (mm)")))
        .legend(Legend::new().title(Title::with_text("Species")));

    let mut plot = Plot::new();
    plot.set_layout(layout);

    let groups_str = groups.str().unwrap();

    for group in groups_str.into_iter() {
        let group = group.unwrap();

        let data = dataset
            .clone()
            .lazy()
            .filter(col(group_column).eq(lit(group)))
            .collect().unwrap();

        let x = data
            .column(x).unwrap()
            .i16().unwrap()
            .to_vec();

        let y = data
            .column(y).unwrap()
            .i16().unwrap()
            .to_vec();

        let trace = Scatter::default()
            .x(x)
            .y(y)
            .name(group)
            .mode(Mode::Markers)
            .marker(Marker::new().size(10).opacity(0.5));

        plot.add_trace(trace);
    }

    plot.show();
}
```

In this example, creating a scatter plot involves writing substantial code to
manually handle the data and configure the plot, including grouping the data by
category and setting up the plot layout.

Now, compare that to the same plot created **using Plotlars**:

```rust
use plotlars::{
    ScatterPlot,
    Plot,
    Rgb,
};

use polars::prelude::*;

fn main() {
    let dataset = LazyCsvReader::new(PlRefPath::new("data/penguins.csv"))
        .finish().unwrap()
        .select([
            col("species"),
            col("flipper_length_mm").cast(DataType::Int16),
            col("body_mass_g").cast(DataType::Int16),
        ])
        .collect().unwrap();

    ScatterPlot::builder()
        .data(&dataset)
        .x("body_mass_g")
        .y("flipper_length_mm")
        .group("species")
        .opacity(0.5)
        .size(12)
        .colors(vec![
            Rgb(178, 34, 34),
            Rgb(65, 105, 225),
            Rgb(255, 140, 0),
        ])
        .plot_title("Penguin Flipper Length vs Body Mass")
        .x_title("Body Mass (g)")
        .y_title("Flipper Length (mm)")
        .legend_title("Species")
        .build()
        .plot();
}
```

This is the output:

![Plot example](https://imgur.com/QMkmhNh.png)

With Plotlars, the same scatter plot is created with significantly less code.
The library abstracts away the complexities of dealing with individual plot
components and allows the user to specify high-level plot characteristics. This
streamlined approach not only saves time but also reduces the potential for
errors and makes the code more readable and maintainable.

## Installation

```bash
cargo add plotlars
```

## Running the examples

Plotlars comes with several ready‑to‑use demo programs in the `examples/` directory.
You can build and execute any of them with Cargo’s `--example` flag:

```bash
cargo run --example barplot
```

Replace `barplot` with the file name (without the `.rs` extension) of the example you want to run.

## Features

- Seamless Integration with Polars: Leverage the power of Polars for efficient
  data manipulation and analysis.
- Support for Multiple Plot Types: Easily create bar, line, scatter, and other
  plot types.
- Customization: Modify plot appearance with an intuitive API.

## Exporting Plots

Plotlars supports exporting plots to static image files in various formats (PNG, JPG, WEBP, SVG) through the `write_image` method.

### Prerequisites

To use image export functionality, you need to enable one of the export features and have the corresponding WebDriver installed:

- **Default** (recommended - uses any available driver):
  ```bash
  cargo add plotlars --features export-default
  ```

- **Chrome/Chromium**:
  ```bash
  cargo add plotlars --features export-chrome
  ```
  Install ChromeDriver: https://chromedriver.chromium.org/

- **Firefox**:
  ```bash
  cargo add plotlars --features export-firefox
  ```
  Install GeckoDriver: https://github.com/mozilla/geckodriver/releases

### Usage

```rust
use plotlars::{ScatterPlot, Plot};
use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dataset = LazyCsvReader::new(PlRefPath::new("data/penguins.csv"))
        .finish()?
        .collect()?;

    let plot = ScatterPlot::builder()
        .data(&dataset)
        .x("body_mass_g")
        .y("flipper_length_mm")
        .plot_title("Penguin Data")
        .build();

    // Export as PNG (1200x800 pixels, 2x scale for high DPI)
    plot.write_image("output.png", 1200, 800, 2.0)?;

    Ok(())
}
```

## Plotlars in Jupyter Notebooks

Plotlars seamlessly integrates with Jupyter Notebooks, allowing you to leverage
the power of interactive data visualization directly within your notebook
environment. This integration is made possible through the use of the
[evcxr project](https://github.com/evcxr/evcxr), which provides a Jupyter kernel
for the Rust programming language.

![Jupyter notebook](https://imgur.com/zvFDzjj.png)

**With Polars, evcxr, and Plotlars, data science in Rust leaps to the next level
, making powerful data analysis and visualization more accessible and efficient
than ever before.**

## License

This project is licensed under the MIT License. See the LICENSE.txt file for details.

## Acknowledgements

- [Polars](https://github.com/pola-rs/polars): For providing a fast and
  efficient data manipulation library.
- [Plotly](https://github.com/plotly/plotly.rs): For the inspiration and ideas
  behind visualization libraries.
- [Evcxr](https://github.com/evcxr/evcxr): For enabling the use of Rust in
  Jupyter Notebooks.
- Rust Community: For the support and development of an amazing programming
  language.
