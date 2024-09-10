use plotly::common::Orientation as OrientationPlotly;

use crate::Rgb;

/// A structure representing a customizable plot legend with properties such as background color, border, font, orientation, and position.
///
/// Examples:
///
/// ```
/// let legend_format = Legend::new()
///     .orientation(Orientation::Horizontal)
///     .border_width(1)
///     .x(0.78)
///     .y(0.825);
///
/// Histogram::builder()
///     .data(&dataset)
///     .x("body_mass_g")
///     .group("species")
///     .colors(vec![Rgb(255, 0, 0), Rgb(0, 255, 0), Rgb(0, 0, 255)])
///     .opacity(0.5)
///     .x_title("Body Mass (g)")
///     .y_title("Frequency")
///     .legend_title("Species")
///     .legend(&legend_format)
///     .build()
///     .plot();
/// ```
///
/// ![example](https://imgur.com/iWGEZs0.png)
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
    ///
    /// # Returns
    ///
    /// Returns a new `Legend` instance with all properties set to `None` or default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the background color of the legend.
    ///
    /// # Arguments
    ///
    /// * `color` - An `Rgb` struct representing the background color.
    ///
    /// # Returns
    ///
    /// Returns the `Legend` instance with the updated background color.
    pub fn background_color(mut self, color: Rgb) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Sets the border color of the legend.
    ///
    /// # Arguments
    ///
    /// * `color` - An `Rgb` struct representing the border color.
    ///
    /// # Returns
    ///
    /// Returns the `Legend` instance with the updated border color.
    pub fn border_color(mut self, color: Rgb) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Sets the border width of the legend.
    ///
    /// # Arguments
    ///
    /// * `width` - A `usize` value representing the width of the border.
    ///
    /// # Returns
    ///
    /// Returns the `Legend` instance with the updated border width.
    pub fn border_width(mut self, width: usize) -> Self {
        self.border_width = Some(width);
        self
    }

    /// Sets the font of the legend labels.
    ///
    /// # Arguments
    ///
    /// * `font` - A value that can be converted into a `String`, representing the font name for the labels.
    ///
    /// # Returns
    ///
    /// Returns the `Legend` instance with the updated font.
    pub fn font(mut self, font: impl Into<String>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the orientation of the legend.
    ///
    /// # Arguments
    ///
    /// * `orientation` - An `Orientation` enum value representing the layout direction of the legend.
    ///
    /// # Returns
    ///
    /// Returns the `Legend` instance with the updated orientation.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    /// Sets the horizontal position of the legend.
    ///
    /// # Arguments
    ///
    /// * `x` - A `f64` value representing the horizontal position of the legend.
    ///
    /// # Returns
    ///
    /// Returns the `Legend` instance with the updated x position.
    pub fn x(mut self, x: f64) -> Self {
        self.x = Some(x);
        self
    }

    /// Sets the vertical position of the legend.
    ///
    /// # Arguments
    ///
    /// * `y` - A `f64` value representing the vertical position of the legend.
    ///
    /// # Returns
    ///
    /// Returns the `Legend` instance with the updated y position.
    pub fn y(mut self, y: f64) -> Self {
        self.y = Some(y);
        self
    }
}

/// Enumeration representing the orientation of the legend.
#[derive(Clone)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    /// Converts `Orientation` to the corresponding `OrientationPlotly` from the `plotly` crate.
    ///
    /// # Returns
    ///
    /// Returns the corresponding `OrientationPlotly`.
    pub fn get_orientation(&self) -> OrientationPlotly {
        match self {
            Self::Horizontal => OrientationPlotly::Horizontal,
            Self::Vertical => OrientationPlotly::Vertical,
        }
    }
}
