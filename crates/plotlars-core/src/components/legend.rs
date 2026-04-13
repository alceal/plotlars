use crate::components::{Orientation, Rgb};

/// A structure representing a customizable plot legend.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Histogram, Legend, Orientation, Plot, Rgb};
///
/// let dataset = LazyCsvReader::new(PlRefPath::new("data/penguins.csv"))
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
#[derive(Clone)]
pub struct Legend {
    pub background_color: Option<Rgb>,
    pub border_color: Option<Rgb>,
    pub border_width: Option<usize>,
    pub font: Option<String>,
    pub orientation: Option<Orientation>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

impl Default for Legend {
    fn default() -> Self {
        Self {
            background_color: Some(Rgb(255, 255, 255)),
            border_color: None,
            border_width: None,
            font: None,
            orientation: None,
            x: None,
            y: None,
        }
    }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let legend = Legend::new();
        let bg = legend.background_color.unwrap();
        assert_eq!(bg.0, 255);
        assert_eq!(bg.1, 255);
        assert_eq!(bg.2, 255);
        assert!(legend.border_color.is_none());
        assert!(legend.border_width.is_none());
        assert!(legend.font.is_none());
        assert!(legend.orientation.is_none());
        assert!(legend.x.is_none());
        assert!(legend.y.is_none());
    }

    #[test]
    fn test_background_color() {
        let legend = Legend::new().background_color(Rgb(200, 200, 200));
        let bg = legend.background_color.unwrap();
        assert_eq!(bg.0, 200);
        assert_eq!(bg.1, 200);
        assert_eq!(bg.2, 200);
    }

    #[test]
    fn test_border_color() {
        let legend = Legend::new().border_color(Rgb(0, 0, 0));
        let bc = legend.border_color.unwrap();
        assert_eq!(bc.0, 0);
        assert_eq!(bc.1, 0);
        assert_eq!(bc.2, 0);
    }

    #[test]
    fn test_border_width() {
        let legend = Legend::new().border_width(2);
        assert_eq!(legend.border_width, Some(2));
    }

    #[test]
    fn test_orientation() {
        let legend = Legend::new().orientation(Orientation::Horizontal);
        assert!(legend.orientation.is_some());
    }

    #[test]
    fn test_builder_chaining() {
        let legend = Legend::new()
            .background_color(Rgb(100, 100, 100))
            .border_color(Rgb(50, 50, 50))
            .border_width(3)
            .font("Arial")
            .orientation(Orientation::Vertical)
            .x(0.5)
            .y(0.8);

        let bg = legend.background_color.unwrap();
        assert_eq!(bg.0, 100);
        let bc = legend.border_color.unwrap();
        assert_eq!(bc.0, 50);
        assert_eq!(legend.border_width, Some(3));
        assert_eq!(legend.font, Some("Arial".to_string()));
        assert!(legend.orientation.is_some());
        assert!((legend.x.unwrap() - 0.5).abs() < 1e-6);
        assert!((legend.y.unwrap() - 0.8).abs() < 1e-6);
    }
}
