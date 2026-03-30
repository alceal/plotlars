use crate::components::Rgb;

/// A structure representing text with customizable content, font, size, and color.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Axis, BarPlot, Plot, Text, Rgb};
///
/// let dataset = df![
///         "label" => &[""],
///         "value" => &[0],
///     ]
///     .unwrap();
///
/// let axis = Axis::new()
///     .tick_values(vec![]);
///
/// BarPlot::builder()
///     .data(&dataset)
///     .labels("label")
///     .values("value")
///     .plot_title(
///         Text::from("Title")
///             .x(0.1)
///             .color(Rgb(178, 34, 34))
///             .size(30)
///             .font("Zapfino")
///     )
///     .x_title(
///         Text::from("X")
///             .color(Rgb(65, 105, 225))
///             .size(20)
///             .font("Marker Felt")
///     )
///     .y_title(
///         Text::from("Y")
///             .color(Rgb(255, 140, 0))
///             .size(20)
///             .font("Arial Black")
///     )
///     .x_axis(&axis)
///     .y_axis(&axis)
///     .build()
///     .plot();
/// ```
/// ![Example](https://imgur.com/4outoUQ.png)
#[derive(Clone)]
pub struct Text {
    pub content: String,
    pub font: String,
    pub size: usize,
    pub color: Rgb,
    pub x: f64,
    pub y: f64,
}

impl Default for Text {
    /// Provides default values for the `Text` struct.
    ///
    /// - `content`: An empty string.
    /// - `font`: An empty string.
    /// - `size`: `12` (reasonable default for visibility).
    /// - `color`: Default `Rgb` value.
    /// - `x`: `0.5`.
    /// - `y`: `0.9`.
    fn default() -> Self {
        Text {
            content: String::new(),
            font: String::new(),
            size: 12,
            color: Rgb::default(),
            x: 0.5,
            y: 0.9,
        }
    }
}

impl Text {
    /// Creates a new `Text` instance from the given content.
    ///
    /// # Argument
    ///
    /// * `content` - A value that can be converted into a `String`, representing the textual content.
    pub fn from(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    /// Sets the font of the text.
    ///
    /// # Argument
    ///
    /// * `font` - A value that can be converted into a `String`, representing the font name.
    pub fn font(mut self, font: impl Into<String>) -> Self {
        self.font = font.into();
        self
    }

    /// Sets the size of the text.
    ///
    /// # Argument
    ///
    /// * `size` - A `usize` value specifying the font size.
    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }

    /// Sets the color of the text.
    ///
    /// # Argument
    ///
    /// * `color` - An `Rgb` value specifying the color of the text.
    pub fn color(mut self, color: Rgb) -> Self {
        self.color = color;
        self
    }

    /// Sets the x-coordinate position of the text.
    ///
    /// # Argument
    ///
    /// * `x` - A `f64` value specifying the horizontal position.
    pub fn x(mut self, x: f64) -> Self {
        self.x = x;
        self
    }

    /// Sets the y-coordinate position of the text.
    ///
    /// # Argument
    ///
    /// * `y` - A `f64` value specifying the vertical position.
    pub fn y(mut self, y: f64) -> Self {
        self.y = y;
        self
    }

    pub fn has_custom_position(&self) -> bool {
        const EPSILON: f64 = 1e-6;
        (self.x - 0.5).abs() > EPSILON || (self.y - 0.9).abs() > EPSILON
    }

    /// Apply default positioning for plot titles (x=0.5, y=0.95 - centered above)
    pub fn with_plot_title_defaults(mut self) -> Self {
        const EPSILON: f64 = 1e-6;
        let y_is_default = (self.y - 0.9).abs() < EPSILON;

        if y_is_default {
            self.y = 0.95;
        }

        self
    }

    /// Apply default positioning for subplot titles (x=0.5, y=1.1 - centered above, higher than overall)
    pub fn with_subplot_title_defaults(mut self) -> Self {
        const EPSILON: f64 = 1e-6;
        let y_is_default = (self.y - 0.9).abs() < EPSILON;
        let y_is_plot_default = (self.y - 0.95).abs() < EPSILON;

        // Override both Text::default (0.9) and plot_title default (0.95)
        if y_is_default || y_is_plot_default {
            self.y = 1.1;
        }

        self
    }

    /// Apply default positioning for x-axis titles (x=0.5, y=-0.15 - centered below)
    pub fn with_x_title_defaults(mut self) -> Self {
        const EPSILON: f64 = 1e-6;
        let y_is_default = (self.y - 0.9).abs() < EPSILON;

        if y_is_default {
            self.y = -0.15;
        }

        self
    }

    /// Apply default positioning for y-axis titles (x=-0.08, y=0.5 - left side, vertically centered)
    pub fn with_y_title_defaults(mut self) -> Self {
        const EPSILON: f64 = 1e-6;
        let x_is_default = (self.x - 0.5).abs() < EPSILON;
        let y_is_default = (self.y - 0.9).abs() < EPSILON;

        if x_is_default {
            self.x = -0.08;
        }

        if y_is_default {
            self.y = 0.5;
        }

        self
    }

    /// Apply default positioning for x-axis title annotations
    /// Used when user sets custom position and annotation mode is triggered
    /// Ensures unset coordinates get appropriate axis defaults
    pub fn with_x_title_defaults_for_annotation(mut self) -> Self {
        const EPSILON: f64 = 1e-6;
        let x_is_default = (self.x - 0.5).abs() < EPSILON;
        let y_is_default = (self.y - 0.9).abs() < EPSILON;

        if x_is_default {
            self.x = 0.5;
        }

        if y_is_default {
            self.y = -0.15;
        }

        self
    }

    /// Apply default positioning for y-axis title annotations
    /// Used when user sets custom position and annotation mode is triggered
    /// Ensures unset coordinates get appropriate axis defaults
    pub fn with_y_title_defaults_for_annotation(mut self) -> Self {
        const EPSILON: f64 = 1e-6;
        let x_is_default = (self.x - 0.5).abs() < EPSILON;
        let y_is_default = (self.y - 0.9).abs() < EPSILON;

        if x_is_default {
            self.x = -0.08;
        }

        if y_is_default {
            self.y = 0.5;
        }

        self
    }
}

impl From<&str> for Text {
    fn from(content: &str) -> Self {
        Self::from(content.to_string())
    }
}

impl From<String> for Text {
    fn from(content: String) -> Self {
        Self::from(content)
    }
}

impl From<&String> for Text {
    fn from(content: &String) -> Self {
        Self::from(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::Rgb;

    fn assert_float_eq(a: f64, b: f64) {
        assert!(
            (a - b).abs() < 1e-6,
            "expected {b}, got {a} (diff {})",
            (a - b).abs()
        );
    }

    #[test]
    fn test_from_str() {
        let t = Text::from("hello");
        assert_eq!(t.content, "hello");
        assert_eq!(t.font, "");
        assert_eq!(t.size, 12);
        assert_eq!(t.color.0, 0);
        assert_eq!(t.color.1, 0);
        assert_eq!(t.color.2, 0);
        assert_float_eq(t.x, 0.5);
        assert_float_eq(t.y, 0.9);
    }

    #[test]
    fn test_from_string() {
        let t: Text = String::from("world").into();
        assert_eq!(t.content, "world");
    }

    #[test]
    fn test_from_ref_string() {
        let s = String::from("ref");
        let t: Text = (&s).into();
        assert_eq!(t.content, "ref");
    }

    #[test]
    fn test_default_values() {
        let t = Text::default();
        assert_eq!(t.content, "");
        assert_eq!(t.font, "");
        assert_eq!(t.size, 12);
        assert_eq!(t.color.0, 0);
        assert_eq!(t.color.1, 0);
        assert_eq!(t.color.2, 0);
        assert_float_eq(t.x, 0.5);
        assert_float_eq(t.y, 0.9);
    }

    #[test]
    fn test_font() {
        let t = Text::from("x").font("Arial");
        assert_eq!(t.font, "Arial");
        assert_eq!(t.size, 12);
    }

    #[test]
    fn test_size() {
        let t = Text::from("x").size(20);
        assert_eq!(t.size, 20);
        assert_eq!(t.font, "");
    }

    #[test]
    fn test_color() {
        let t = Text::from("x").color(Rgb(1, 2, 3));
        assert_eq!(t.color.0, 1);
        assert_eq!(t.color.1, 2);
        assert_eq!(t.color.2, 3);
    }

    #[test]
    fn test_x() {
        let t = Text::from("x").x(0.1);
        assert_float_eq(t.x, 0.1);
        assert_float_eq(t.y, 0.9);
    }

    #[test]
    fn test_y() {
        let t = Text::from("x").y(0.2);
        assert_float_eq(t.y, 0.2);
        assert_float_eq(t.x, 0.5);
    }

    #[test]
    fn test_builder_chaining() {
        let t = Text::from("chained")
            .font("Courier")
            .size(24)
            .color(Rgb(10, 20, 30))
            .x(0.3)
            .y(0.7);
        assert_eq!(t.content, "chained");
        assert_eq!(t.font, "Courier");
        assert_eq!(t.size, 24);
        assert_eq!(t.color.0, 10);
        assert_eq!(t.color.1, 20);
        assert_eq!(t.color.2, 30);
        assert_float_eq(t.x, 0.3);
        assert_float_eq(t.y, 0.7);
    }

    #[test]
    fn test_has_custom_position_default() {
        let t = Text::default();
        assert!(!t.has_custom_position());
    }

    #[test]
    fn test_has_custom_position_x_changed() {
        let t = Text::from("x").x(0.3);
        assert!(t.has_custom_position());
    }

    #[test]
    fn test_has_custom_position_epsilon() {
        let t = Text::from("x").x(0.5 + 1e-7);
        assert!(!t.has_custom_position());
    }

    #[test]
    fn test_with_plot_title_defaults() {
        let t = Text::from("title").with_plot_title_defaults();
        assert_float_eq(t.y, 0.95);

        let t2 = Text::from("title").y(0.7).with_plot_title_defaults();
        assert_float_eq(t2.y, 0.7);
    }

    #[test]
    fn test_with_x_title_defaults() {
        let t = Text::from("x").with_x_title_defaults();
        assert_float_eq(t.y, -0.15);
    }

    #[test]
    fn test_with_y_title_defaults() {
        let t = Text::from("y").with_y_title_defaults();
        assert_float_eq(t.x, -0.08);
        assert_float_eq(t.y, 0.5);

        let t2 = Text::from("y").x(0.2).y(0.3).with_y_title_defaults();
        assert_float_eq(t2.x, 0.2);
        assert_float_eq(t2.y, 0.3);
    }
}
