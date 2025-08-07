use plotly::{common::Font, layout::Legend as LegendPlotly};

use crate::{Orientation, Rgb, Text};

/// A structure representing a customizable plot legend.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Histogram, Legend, Orientation, Plot, Rgb};
///
/// let dataset = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
///     .finish()
///     .unwrap()
///     .select([
///         col("species"),
///         col("sex").alias("gender"),
///         col("flipper_length_mm").cast(DataType::Int16),
///         col("body_mass_g").cast(DataType::Int16),
///     ])
///     .collect()
///     .unwrap();
///
/// let legend = Legend::new()
///     .orientation(Orientation::Horizontal)
///     .border_width(1)
///     .x(0.78)
///     .y(0.825);
///
/// Histogram::builder()
///     .data(&dataset)
///     .x("body_mass_g")
///     .group("species")
///     .colors(vec![
///         Rgb(255, 0, 0),
///         Rgb(0, 255, 0),
///         Rgb(0, 0, 255),
///     ])
///     .opacity(0.5)
///     .x_title("Body Mass (g)")
///     .y_title("Frequency")
///     .legend_title("Species")
///     .legend(&legend)
///     .build()
///     .plot();
/// ```
///
/// ![example](https://imgur.com/GpUsgli.png)
#[derive(Clone, Default)]
pub struct Legend {
    pub(crate) background_color: Option<Rgb>,
    pub(crate) border_color: Option<Rgb>,
    pub(crate) border_width: Option<usize>,
    pub(crate) font: Option<String>,
    pub(crate) orientation: Option<Orientation>,
    pub(crate) x: Option<f64>,
    pub(crate) y: Option<f64>,
}

impl Legend {
    /// Creates a new `Legend` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the background color of the legend.
    ///
    /// # Argument
    ///
    /// * `color` - An `Rgb` struct representing the background color.
    pub fn background_color(mut self, color: Rgb) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Sets the border color of the legend.
    ///
    /// # Argument
    ///
    /// * `color` - An `Rgb` struct representing the border color.
    pub fn border_color(mut self, color: Rgb) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Sets the border width of the legend.
    ///
    /// # Argument
    ///
    /// * `width` - A `usize` value representing the width of the border.
    pub fn border_width(mut self, width: usize) -> Self {
        self.border_width = Some(width);
        self
    }

    /// Sets the font of the legend labels.
    ///
    /// # Argument
    ///
    /// * `font` - A value that can be converted into a `String`, representing the font name for the labels.
    pub fn font(mut self, font: impl Into<String>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the orientation of the legend.
    ///
    /// # Argument
    ///
    /// * `orientation` - An `Orientation` enum value representing the layout direction of the legend.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    /// Sets the horizontal position of the legend.
    ///
    /// # Argument
    ///
    /// * `x` - A `f64` value representing the horizontal position of the legend.
    pub fn x(mut self, x: f64) -> Self {
        self.x = Some(x);
        self
    }

    /// Sets the vertical position of the legend.
    ///
    /// # Argument
    ///
    /// * `y` - A `f64` value representing the vertical position of the legend.
    pub fn y(mut self, y: f64) -> Self {
        self.y = Some(y);
        self
    }

    pub(crate) fn set_legend(title: Option<Text>, format: Option<&Legend>) -> LegendPlotly {
        let mut legend = LegendPlotly::new();

        if let Some(title) = title {
            legend = legend.title(title.to_plotly());
        }

        if let Some(format) = format {
            legend = Self::set_format(legend, format);
        }

        legend
    }

    fn set_format(mut legend: LegendPlotly, format: &Legend) -> LegendPlotly {
        if let Some(color) = format.background_color {
            legend = legend.background_color(color.to_plotly());
        }

        if let Some(color) = format.border_color {
            legend = legend.border_color(color.to_plotly());
        }

        if let Some(width) = format.border_width {
            legend = legend.border_width(width);
        }

        if let Some(font) = &format.font {
            legend = legend.font(Font::new().family(font.as_str()));
        }

        if let Some(orientation) = &format.orientation {
            legend = legend.orientation(orientation.to_plotly());
        }

        if let Some(x) = format.x {
            legend = legend.x(x);
        }

        if let Some(y) = format.y {
            legend = legend.y(y);
        }

        legend
    }
}
