use serde_json::Value;

use crate::components::{Orientation, Rgb, Text, TickDirection, ValueExponent};

/// A structure representing a color bar component for visualizations.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{ColorBar, HeatMap, Orientation, Palette, Plot, Text, ValueExponent};
///
/// let dataset = LazyCsvReader::new(PlRefPath::new("data/heatmap.csv"))
///     .finish()
///     .unwrap()
///     .collect()
///     .unwrap();
///
/// HeatMap::builder()
///     .data(&dataset)
///     .x("x")
///     .y("y")
///     .z("z")
///     .color_bar(
///         &ColorBar::new()
///             .orientation(Orientation::Horizontal)
///             .length(0.7)
///             .value_exponent(ValueExponent::None)
///             .separate_thousands(true)
///             .tick_length(5)
///             .tick_step(2500.0)
///             .tick_angle(90.0)
///             .y(-0.6)
///     )
///     .color_scale(Palette::Viridis)
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/yZ4KFEU.png)
#[derive(Clone, Default)]
pub struct ColorBar {
    pub background_color: Option<Rgb>,
    pub border_color: Option<Rgb>,
    pub border_width: Option<usize>,
    pub tick_step: Option<f64>,
    pub value_exponent: Option<ValueExponent>,
    pub length: Option<f64>,
    pub n_ticks: Option<usize>,
    pub orientation: Option<Orientation>,
    pub outline_color: Option<Rgb>,
    pub outline_width: Option<usize>,
    pub separate_thousands: Option<bool>,
    pub width: Option<f64>,
    pub tick_angle: Option<f64>,
    pub tick_color: Option<Rgb>,
    pub tick_font: Option<String>,
    pub tick_length: Option<usize>,
    pub tick_labels: Option<Vec<String>>,
    pub tick_values: Option<Vec<f64>>,
    pub tick_width: Option<usize>,
    pub tick_direction: Option<TickDirection>,
    pub title: Option<Text>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

impl ColorBar {
    /// Creates a new `ColorBar` instance with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the original f64 values for length and width.
    /// These are used for JSON post-processing to inject proper fractions.
    pub fn get_fraction_values(&self) -> (Option<f64>, Option<f64>) {
        (self.length, self.width)
    }

    /// Post-processes a trace JSON value to inject f64 length/width values into the colorbar.
    /// This bypasses plotly.rs's usize limitation for fraction-based sizing.
    pub fn patch_trace_json(trace_json: &mut Value, length: Option<f64>, thickness: Option<f64>) {
        // Navigate to marker.colorbar or colorbar depending on trace type
        let colorbar_obj = if let Some(marker) = trace_json.get_mut("marker") {
            marker.get_mut("colorbar")
        } else {
            trace_json.get_mut("colorbar")
        };

        if let Some(colorbar) = colorbar_obj {
            if let Some(len) = length {
                colorbar["len"] = Value::from(len);
                colorbar["lenmode"] = Value::from("fraction");
            }
            if let Some(thick) = thickness {
                colorbar["thickness"] = Value::from(thick);
                colorbar["thicknessmode"] = Value::from("fraction");
            }
        }
    }

    /// Sets the background color of the color bar.
    ///
    /// # Arguments
    ///
    /// * `color` - An `Rgb` value representing the desired background color.
    pub fn background_color(mut self, color: Rgb) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Sets the border color of the color bar.
    ///
    /// # Arguments
    ///
    /// * `color` - An `Rgb` value representing the desired border color.
    pub fn border_color(mut self, color: Rgb) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Sets the width of the color bar's border.
    ///
    /// # Arguments
    ///
    /// * `width` - A `usize` value specifying the border width in pixels.
    pub fn border_width(mut self, width: usize) -> Self {
        self.border_width = Some(width);
        self
    }

    /// Sets the step size between ticks on the color bar.
    ///
    /// # Arguments
    ///
    /// * `step` - A `f64` value representing the step size between ticks.
    pub fn tick_step(mut self, step: f64) -> Self {
        self.tick_step = Some(step);
        self
    }

    /// Sets the exponent format for values on the axis.
    ///
    /// # Argument
    ///
    /// * `exponent` - A `ValueExponent` enum value representing the exponent format.
    pub fn value_exponent(mut self, exponent: ValueExponent) -> Self {
        self.value_exponent = Some(exponent);
        self
    }

    /// Sets the length of the color bar.
    ///
    /// # Argument
    ///
    /// * `length` - A `f64` value between 0.0 and 1.0 specifying the length as a fraction of the subplot height.
    pub fn length(mut self, length: f64) -> Self {
        self.length = Some(length);
        self
    }

    /// Sets the number of ticks on the color bar.
    ///
    /// # Arguments
    ///
    /// * `n` - A `usize` value representing the number of ticks.
    pub fn n_ticks(mut self, n: usize) -> Self {
        self.n_ticks = Some(n);
        self
    }

    /// Sets the orientation of the color bar.
    ///
    /// # Arguments
    ///
    /// * `orientation` - An `Orientation` enum value specifying the orientation (e.g., horizontal or vertical).
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    /// Sets the outline color of the color bar.
    ///
    /// # Arguments
    ///
    /// * `color` - An `Rgb` value representing the desired outline color.
    pub fn outline_color(mut self, color: Rgb) -> Self {
        self.outline_color = Some(color);
        self
    }

    /// Sets the outline width of the color bar.
    ///
    /// # Arguments
    ///
    /// * `width` - A `usize` value specifying the outline width in pixels.
    pub fn outline_width(mut self, width: usize) -> Self {
        self.outline_width = Some(width);
        self
    }

    /// Specifies whether to separate thousands in tick labels.
    ///
    /// # Arguments
    ///
    /// * `separate_thousands` - A `bool` indicating whether to separate thousands.
    pub fn separate_thousands(mut self, separate_thousands: bool) -> Self {
        self.separate_thousands = Some(separate_thousands);
        self
    }

    /// Sets the width of the color bar.
    ///
    /// # Argument
    ///
    /// * `width` - A `f64` value between 0.0 and 1.0 specifying the width as a fraction of the subplot width.
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the angle of the tick labels on the color bar.
    ///
    /// # Arguments
    ///
    /// * `angle` - A `f64` value representing the angle in degrees.
    pub fn tick_angle(mut self, angle: f64) -> Self {
        self.tick_angle = Some(angle);
        self
    }

    /// Sets the color of the ticks on the color bar.
    ///
    /// # Arguments
    ///
    /// * `color` - An `Rgb` value representing the tick color.
    pub fn tick_color(mut self, color: Rgb) -> Self {
        self.tick_color = Some(color);
        self
    }

    /// Sets the font of the tick labels on the color bar.
    ///
    /// # Arguments
    ///
    /// * `font` - A string representing the font family.
    pub fn tick_font(mut self, font: impl Into<String>) -> Self {
        self.tick_font = Some(font.into());
        self
    }

    /// Sets the length of the axis ticks.
    ///
    /// # Argument
    ///
    /// * `length` - A `usize` value representing the length of the ticks.
    pub fn tick_length(mut self, length: usize) -> Self {
        self.tick_length = Some(length);
        self
    }

    /// Sets the tick labels for the axis.
    ///
    /// # Argument
    ///
    /// * `labels` - A vector of values that can be converted into `String`, representing the tick labels.
    pub fn tick_labels(mut self, labels: Vec<impl Into<String>>) -> Self {
        self.tick_labels = Some(labels.into_iter().map(|x| x.into()).collect());
        self
    }

    /// Sets the tick values for the axis.
    ///
    /// # Argument
    ///
    /// * `values` - A vector of `f64` values representing the tick values.
    pub fn tick_values(mut self, values: Vec<f64>) -> Self {
        self.tick_values = Some(values);
        self
    }

    /// Sets the width of the ticks on the color bar.
    ///
    /// # Arguments
    ///
    /// * `width` - A `usize` value specifying the tick width in pixels.
    pub fn tick_width(mut self, width: usize) -> Self {
        self.tick_width = Some(width);
        self
    }

    /// Sets the direction of the axis ticks.
    ///
    /// # Argument
    ///
    /// * `direction` - A `TickDirection` enum value representing the direction of the ticks.
    pub fn tick_direction(mut self, direction: TickDirection) -> Self {
        self.tick_direction = Some(direction);
        self
    }

    /// Sets the title of the color bar.
    ///
    /// # Arguments
    ///
    /// * `title` - A value that can be converted into `Text`, representing the title.
    pub fn title<T: Into<Text>>(mut self, title: T) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the x-coordinate position of the text.
    ///
    /// # Argument
    ///
    /// * `x` - A `f64` value specifying the horizontal position.
    pub fn x(mut self, x: f64) -> Self {
        self.x = Some(x);
        self
    }

    /// Sets the y-coordinate position of the text.
    ///
    /// # Argument
    ///
    /// * `y` - A `f64` value specifying the vertical position.
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
        let cb = ColorBar::new();
        assert!(cb.background_color.is_none());
        assert!(cb.border_color.is_none());
        assert!(cb.border_width.is_none());
        assert!(cb.tick_step.is_none());
        assert!(cb.value_exponent.is_none());
        assert!(cb.length.is_none());
        assert!(cb.n_ticks.is_none());
        assert!(cb.orientation.is_none());
        assert!(cb.outline_color.is_none());
        assert!(cb.outline_width.is_none());
        assert!(cb.separate_thousands.is_none());
        assert!(cb.width.is_none());
        assert!(cb.tick_angle.is_none());
        assert!(cb.tick_color.is_none());
        assert!(cb.tick_font.is_none());
        assert!(cb.tick_length.is_none());
        assert!(cb.tick_labels.is_none());
        assert!(cb.tick_values.is_none());
        assert!(cb.tick_width.is_none());
        assert!(cb.tick_direction.is_none());
        assert!(cb.title.is_none());
        assert!(cb.x.is_none());
        assert!(cb.y.is_none());
    }

    #[test]
    fn test_title() {
        let cb = ColorBar::new().title(Text::from("Scale"));
        assert!(cb.title.is_some());
    }

    #[test]
    fn test_length() {
        let cb = ColorBar::new().length(0.5);
        assert!((cb.length.unwrap() - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_thickness() {
        let cb = ColorBar::new().width(20.0);
        assert!((cb.width.unwrap() - 20.0).abs() < 1e-6);
    }

    #[test]
    fn test_builder_chaining() {
        let cb = ColorBar::new()
            .background_color(Rgb(255, 255, 255))
            .border_width(2)
            .length(0.7)
            .tick_step(100.0)
            .separate_thousands(true)
            .x(0.5)
            .y(-0.3);

        let bg = cb.background_color.unwrap();
        assert_eq!(bg.0, 255);
        assert_eq!(cb.border_width, Some(2));
        assert!((cb.length.unwrap() - 0.7).abs() < 1e-6);
        assert!((cb.tick_step.unwrap() - 100.0).abs() < 1e-6);
        assert_eq!(cb.separate_thousands, Some(true));
        assert!((cb.x.unwrap() - 0.5).abs() < 1e-6);
        assert!((cb.y.unwrap() - (-0.3)).abs() < 1e-6);
    }
}
