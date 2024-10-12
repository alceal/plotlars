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

> ## <span style="color: red; font-weight: bold;">⚠️ Important Note</span>
>
> **Due to an issue in the `polars` crate, you must install the `plotlars` crate
> from its GitHub repository to ensure proper functionality.**
>
> To include the GitHub version of `plotlars` with the fix in your project, add the following line to your `Cargo.toml` file, specifying the `fix/polars-issue` branch:
>
> ```toml
> [dependencies]
> plotlars = { git = "https://github.com/user/plotlars", branch = "fix/polars-issue" }
> ```
>
> If you’re using a Rust notebook, you can include the dependency using:
>```
> :dep plotlars = { git = "https://github.com/alceal/plotlars", branch = "fix/polars-issue" }
> ```

Plotlars is a versatile Rust library that acts as a wrapper around the Plotly
crate, bridging the gap between the powerful Polars data analysis library and
Plotly. It simplifies the process of creating visualizations from data frames,
allowing developers to focus on data insights rather than the intricacies of
plot creation.

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
    let dataset = LazyCsvReader::new("data/penguins.csv")
        .finish().unwrap()
        .select([
            col("species").cast(
                DataType::Categorical(
                    None,
                    CategoricalOrdering::default()
                )
            ),
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

    for group in groups.iter() {
        let group = group.get_str().unwrap();

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
    Text,
};

use polars::prelude::*;

fn main() {
    let dataset = LazyCsvReader::new("data/penguins.csv")
        .finish().unwrap()
        .select([
            col("species").cast(
                DataType::Categorical(
                    None,
                    CategoricalOrdering::default()
                )
            ),
            col("flipper_length_mm").cast(DataType::Int16),
            col("body_mass_g").cast(DataType::Int16),
        ])
        .collect().unwrap();

    ScatterPlot::builder()
        .data(&dataset)
        .x("body_mass_g")
        .y("flipper_length_mm")
        .group("species")
        .size(10)
        .opacity(0.5)
        .plot_title("Penguin Flipper Length vs Body Mass")
        .x_title("Body Mass (g)")
        .y_title("Flipper Length (mm)")
        .legend_title("Species")
        .build()
        .plot();
}
```

This is the output:

![Plot example](https://imgur.com/PkQ9fsc.png)

With Plotlars, the same scatter plot is created with significantly less code.
The library abstracts away the complexities of dealing with individual plot
components and allows the user to specify high-level plot characteristics. This
streamlined approach not only saves time but also reduces the potential for
errors and makes the code more readable and maintainable.

## Installation

```bash
cargo add plotlars
```

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
