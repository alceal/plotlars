use plotly::common::{Anchor, Font, Title};
use plotly::layout::Annotation;

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
    pub(crate) content: String,
    pub(crate) font: String,
    pub(crate) size: usize,
    pub(crate) color: Rgb,
    pub(crate) x: f64,
    pub(crate) y: f64,
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

    pub(crate) fn to_plotly(&self) -> Title {
        Title::with_text(&self.content)
            .font(
                Font::new()
                    .family(self.font.as_str())
                    .size(self.size)
                    .color(self.color.to_plotly()),
            )
            .x(self.x)
            .y(self.y)
    }

    pub(crate) fn to_font(&self) -> Font {
        Font::new()
            .family(self.font.as_str())
            .size(self.size)
            .color(self.color.to_plotly())
    }

    pub(crate) fn has_custom_position(&self) -> bool {
        const EPSILON: f64 = 1e-6;
        (self.x - 0.5).abs() > EPSILON || (self.y - 0.9).abs() > EPSILON
    }

    /// Apply default positioning for plot titles (x=0.5, y=0.95 - centered above)
    pub(crate) fn with_plot_title_defaults(mut self) -> Self {
        const EPSILON: f64 = 1e-6;
        let y_is_default = (self.y - 0.9).abs() < EPSILON;

        if y_is_default {
            self.y = 0.95;
        }

        self
    }

    /// Apply default positioning for subplot titles (x=0.5, y=1.1 - centered above, higher than overall)
    pub(crate) fn with_subplot_title_defaults(mut self) -> Self {
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
    pub(crate) fn with_x_title_defaults(mut self) -> Self {
        const EPSILON: f64 = 1e-6;
        let y_is_default = (self.y - 0.9).abs() < EPSILON;

        if y_is_default {
            self.y = -0.15;
        }

        self
    }

    /// Apply default positioning for y-axis titles (x=-0.08, y=0.5 - left side, vertically centered)
    pub(crate) fn with_y_title_defaults(mut self) -> Self {
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
    pub(crate) fn with_x_title_defaults_for_annotation(mut self) -> Self {
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
    pub(crate) fn with_y_title_defaults_for_annotation(mut self) -> Self {
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

    pub(crate) fn to_axis_annotation(
        &self,
        is_x_axis: bool,
        axis_ref: &str,
        use_domain: bool,
    ) -> Annotation {
        let (x_ref, y_ref) = if use_domain {
            let axis_num = axis_ref.trim_start_matches(['x', 'y']);

            let x_axis = if axis_num.is_empty() {
                "x"
            } else {
                &format!("x{}", axis_num)
            };
            let y_axis = if axis_num.is_empty() {
                "y"
            } else {
                &format!("y{}", axis_num)
            };

            (format!("{} domain", x_axis), format!("{} domain", y_axis))
        } else {
            ("paper".to_string(), "paper".to_string())
        };

        let y_anchor = Anchor::Middle;
        let x_anchor = if is_x_axis {
            Anchor::Center
        } else {
            Anchor::Left
        };

        let effective_size = if self.size == 0 { 12 } else { self.size };

        let mut annotation = Annotation::new()
            .text(&self.content)
            .font(
                Font::new()
                    .family(self.font.as_str())
                    .size(effective_size)
                    .color(self.color.to_plotly()),
            )
            .x_ref(&x_ref)
            .y_ref(&y_ref)
            .x(self.x)
            .y(self.y)
            .x_anchor(x_anchor)
            .y_anchor(y_anchor)
            .show_arrow(false);

        if !is_x_axis {
            annotation = annotation.text_angle(-90.0);
        }

        annotation
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
