# Plotlars

<p align="center">
    <a href="https://crates.io/crates/plotlars">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/plotlars.svg"></a>
    <a href="https://docs.rs/plotlars">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/plotlars">
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

| Plot | Example | Plot | Example | Plot | Example |
|------|:---------:|------|:---------:|------|:---------:|
| [Array 2D](https://docs.rs/plotlars/latest/plotlars/struct.Array2dPlot.html) | <img src="https://imgur.com/LMrqAaT.png" width="100" height="100"> | [Bar Plot](https://docs.rs/plotlars/latest/plotlars/struct.BarPlot.html) | <img src="https://imgur.com/xKHucCp.png" width="100" height="100"> | [Box Plot](https://docs.rs/plotlars/latest/plotlars/struct.BoxPlot.html) | <img src="https://imgur.com/uj1LY90.png" width="100" height="100"> |
| [Candlestick](https://docs.rs/plotlars/latest/plotlars/struct.CandlestickPlot.html) | <img src="https://imgur.com/91y2Kis.png" width="100" height="100"> | [Contour Plot](https://docs.rs/plotlars/latest/plotlars/struct.ContourPlot.html) | <img src="https://imgur.com/VWgxHC8.png" width="100" height="100"> | [Density Mapbox](https://docs.rs/plotlars/latest/plotlars/struct.DensityMapbox.html) | <img src="https://imgur.com/82eLyBm.png" width="100" height="100"> |
| [Heat Map](https://docs.rs/plotlars/latest/plotlars/struct.HeatMap.html) | <img src="https://imgur.com/5uFih4M.png" width="100" height="100"> | [Histogram](https://docs.rs/plotlars/latest/plotlars/struct.Histogram.html) | <img src="https://imgur.com/w2oiuIo.png" width="100" height="100"> | [Image](https://docs.rs/plotlars/latest/plotlars/struct.Image.html) | <img src="https://imgur.com/PAtdaHj.png" width="100" height="100"> |
| [Line Plot](https://docs.rs/plotlars/latest/plotlars/struct.LinePlot.html) | <img src="https://imgur.com/PaXG300.png" width="100" height="100"> | [Mesh3D](https://docs.rs/plotlars/latest/plotlars/struct.Mesh3D.html) | <img src="https://imgur.com/bljzmw5.png" width="100" height="100"> | [OHLC](https://docs.rs/plotlars/latest/plotlars/struct.OhlcPlot.html) | <img src="https://imgur.com/Sv8r9VN.png" width="100" height="100"> |
| [Pie Chart](https://docs.rs/plotlars/latest/plotlars/struct.PieChart.html) | <img src="https://imgur.com/q44HDwT.png" width="100" height="100"> | [Sankey Diagram](https://docs.rs/plotlars/latest/plotlars/struct.SankeyDiagram.html) | <img src="https://imgur.com/jvAew8u.png" width="100" height="100"> | [Scatter 3D Plot](https://docs.rs/plotlars/latest/plotlars/struct.Scatter3dPlot.html) | <img src="https://imgur.com/WYTQxHA.png" width="100" height="100"> |
| [Scatter Geo](https://docs.rs/plotlars/latest/plotlars/struct.ScatterGeo.html) | <img src="https://imgur.com/8PCEbhN.png" width="100" height="100"> | [Scatter Map](https://docs.rs/plotlars/latest/plotlars/struct.ScatterMap.html) | <img src="https://imgur.com/8MCjVOd.png" width="100" height="100"> | [Scatter Plot](https://docs.rs/plotlars/latest/plotlars/struct.ScatterPlot.html) | <img src="https://imgur.com/9jfO8RU.png" width="100" height="100"> |
| [Scatter Polar](https://docs.rs/plotlars/latest/plotlars/struct.ScatterPolar.html) | <img src="https://imgur.com/kl1pY9c.png" width="100" height="100"> | [Surface Plot](https://docs.rs/plotlars/latest/plotlars/struct.SurfacePlot.html) | <img src="https://imgur.com/tdVte4l.png" width="100" height="100"> | [Table](https://docs.rs/plotlars/latest/plotlars/struct.Table.html) | <img src="https://imgur.com/QDKTeFX.png" width="100" height="100"> |
| [Time Series](https://docs.rs/plotlars/latest/plotlars/struct.TimeSeriesPlot.html) | <img src="https://imgur.com/hL27Xcn.png" width="100" height="100"> | | | | |

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
    let dataset = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
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
    let dataset = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
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
