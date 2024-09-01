use plotly::{
    common::{AxisSide, ExponentFormat},
    layout::{AxisType as AxisTypePlotly, TicksDirection},
};

use crate::Rgb;

/// A structure representing an axis with customizable properties such as position, type, color, ticks, and grid lines.
///
/// Examples:
///
/// ![example 1](https://imgur.com/YvfFQfb.png)
/// ![example 2](https://imgur.com/CbFCEB2.png)
#[derive(Default, Clone)]
pub struct Axis {
    pub(crate) show_axis: Option<bool>,
    pub(crate) axis_position: Option<AxisPosition>,
    pub(crate) axis_type: Option<AxisType>,
    pub(crate) value_color: Option<Rgb>,
    pub(crate) value_range: Option<Vec<f64>>,
    pub(crate) value_thousands: Option<bool>,
    pub(crate) value_exponent: Option<ValueExponent>,
    pub(crate) tick_values: Option<Vec<f64>>,
    pub(crate) tick_labels: Option<Vec<String>>,
    pub(crate) tick_direction: Option<TickDirection>,
    pub(crate) tick_length: Option<usize>,
    pub(crate) tick_width: Option<usize>,
    pub(crate) tick_color: Option<Rgb>,
    pub(crate) tick_angle: Option<f64>,
    pub(crate) tick_font: Option<String>,
    pub(crate) show_line: Option<bool>,
    pub(crate) line_color: Option<Rgb>,
    pub(crate) line_width: Option<usize>,
    pub(crate) show_grid: Option<bool>,
    pub(crate) grid_color: Option<Rgb>,
    pub(crate) grid_width: Option<usize>,
    pub(crate) show_zero_line: Option<bool>,
    pub(crate) zero_line_color: Option<Rgb>,
    pub(crate) zero_line_width: Option<usize>,
}

impl Axis {
    /// Creates a new `Axis` instance with default values.
    ///
    /// # Returns
    ///
    /// Returns a new `Axis` instance with all properties set to `None` or default values.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new();
    /// ```
    pub fn new() -> Self {
        Axis::default()
    }

    /// Sets the visibility of the axis.
    ///
    /// # Arguments
    ///
    /// * `bool` - A boolean value indicating whether the axis should be visible.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated visibility.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().show_axis(true);
    /// ```
    pub fn show_axis(mut self, bool: bool) -> Self {
        self.show_axis = Some(bool);
        self
    }

    /// Sets the position of the axis.
    ///
    /// # Arguments
    ///
    /// * `position` - An `AxisPosition` enum value representing the position of the axis.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated position.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().axis_position(AxisPosition::Bottom);
    /// ```
    pub fn axis_position(mut self, position: AxisPosition) -> Self {
        self.axis_position = Some(position);
        self
    }

    /// Sets the type of the axis.
    ///
    /// # Arguments
    ///
    /// * `axis_type` - An `AxisType` enum value representing the type of the axis.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated type.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().axis_type(AxisType::Linear);
    /// ```
    pub fn axis_type(mut self, axis_type: AxisType) -> Self {
        self.axis_type = Some(axis_type);
        self
    }

    /// Sets the color of the axis values.
    ///
    /// # Arguments
    ///
    /// * `color` - An `Rgb` struct representing the color of the axis values.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated value color.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().value_color(Rgb(255, 0, 0));
    /// ```
    pub fn value_color(mut self, color: Rgb) -> Self {
        self.value_color = Some(color);
        self
    }

    /// Sets the range of values displayed on the axis.
    ///
    /// # Arguments
    ///
    /// * `range` - A vector of `f64` values representing the range of the axis.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated value range.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().value_range(vec![0.0, 100.0]);
    /// ```
    pub fn value_range(mut self, range: Vec<f64>) -> Self {
        self.value_range = Some(range);
        self
    }

    /// Sets whether to use thousands separators for values.
    ///
    /// # Arguments
    ///
    /// * `bool` - A boolean value indicating whether to use thousands separators.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated setting.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().value_thousands(true);
    /// ```
    pub fn value_thousands(mut self, bool: bool) -> Self {
        self.value_thousands = Some(bool);
        self
    }

    /// Sets the exponent format for values on the axis.
    ///
    /// # Arguments
    ///
    /// * `exponent` - A `ValueExponent` enum value representing the exponent format.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated exponent format.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().value_exponent(ValueExponent::SmallE);
    /// ```
    pub fn value_exponent(mut self, exponent: ValueExponent) -> Self {
        self.value_exponent = Some(exponent);
        self
    }

    /// Sets the tick values for the axis.
    ///
    /// # Arguments
    ///
    /// * `tick_values` - A vector of `f64` values representing the tick values.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated tick values.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().tick_values(vec![0.0, 50.0, 100.0]);
    /// ```
    pub fn tick_values(mut self, tick_values: Vec<f64>) -> Self {
        self.tick_values = Some(tick_values);
        self
    }

    /// Sets the tick labels for the axis.
    ///
    /// # Arguments
    ///
    /// * `tick_labels` - A vector of values that can be converted into `String`, representing the tick labels.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated tick labels.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().tick_labels(vec!["Low", "Medium", "High"]);
    /// ```
    pub fn tick_labels(mut self, tick_labels: Vec<impl Into<String>>) -> Self {
        self.tick_labels = Some(tick_labels.into_iter().map(|x| x.into()).collect());
        self
    }

    /// Sets the direction of the axis ticks.
    ///
    /// # Arguments
    ///
    /// * `tick_direction` - A `TickDirection` enum value representing the direction of the ticks.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated tick direction.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().tick_direction(TickDirection::OutSide);
    /// ```
    pub fn tick_direction(mut self, tick_direction: TickDirection) -> Self {
        self.tick_direction = Some(tick_direction);
        self
    }

    /// Sets the length of the axis ticks.
    ///
    /// # Arguments
    ///
    /// * `tick_length` - A `usize` value representing the length of the ticks.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated tick length.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().tick_length(10);
    /// ```
    pub fn tick_length(mut self, tick_length: usize) -> Self {
        self.tick_length = Some(tick_length);
        self
    }

    /// Sets the width of the axis ticks.
    ///
    /// # Arguments
    ///
    /// * `tick_width` - A `usize` value representing the width of the ticks.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated tick width.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().tick_width(2);
    /// ```
    pub fn tick_width(mut self, tick_width: usize) -> Self {
        self.tick_width = Some(tick_width);
        self
    }

    /// Sets the color of the axis ticks.
    ///
    /// # Arguments
    ///
    /// * `tick_color` - An `Rgb` struct representing the color of the ticks.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated tick color.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().tick_color(Rgb(0, 0, 0));
    /// ```
    pub fn tick_color(mut self, tick_color: Rgb) -> Self {
        self.tick_color = Some(tick_color);
        self
    }

    /// Sets the angle of the axis ticks.
    ///
    /// # Arguments
    ///
    /// * `tick_angle` - A `f64` value representing the angle of the ticks in degrees.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated tick angle.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().tick_angle(45.0);
    /// ```
    pub fn tick_angle(mut self, tick_angle: f64) -> Self {
        self.tick_angle = Some(tick_angle);
        self
    }

    /// Sets the font of the axis tick labels.
    ///
    /// # Arguments
    ///
    /// * `tick_font` - A value that can be converted into a `String`, representing the font name for the tick labels.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated tick font.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().tick_font("Arial");
    /// ```
    pub fn tick_font(mut self, tick_font: impl Into<String>) -> Self {
        self.tick_font = Some(tick_font.into());
        self
    }

    /// Sets whether to show the axis line.
    ///
    /// # Arguments
    ///
    /// * `bool` - A boolean value indicating whether the axis line should be visible.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated axis line visibility.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().show_line(true);
    /// ```
    pub fn show_line(mut self, bool: bool) -> Self {
        self.show_line = Some(bool);
        self
    }

    /// Sets the color of the axis line.
    ///
    /// # Arguments
    ///
    /// * `color` - An `Rgb` struct representing the color of the axis line.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated axis line color.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().line_color(Rgb(0, 0, 0));
    /// ```
    pub fn line_color(mut self, color: Rgb) -> Self {
        self.line_color = Some(color);
        self
    }

    /// Sets the width of the axis line.
    ///
    /// # Arguments
    ///
    /// * `width` - A `usize` value representing the width of the axis line.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated axis line width.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().line_width(2);
    /// ```
    pub fn line_width(mut self, width: usize) -> Self {
        self.line_width = Some(width);
        self
    }

    /// Sets whether to show the grid lines on the axis.
    ///
    /// # Arguments
    ///
    /// * `bool` - A boolean value indicating whether the grid lines should be visible.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated grid line visibility.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().show_grid(true);
    /// ```
    pub fn show_grid(mut self, bool: bool) -> Self {
        self.show_grid = Some(bool);
        self
    }

    /// Sets the color of the grid lines on the axis.
    ///
    /// # Arguments
    ///
    /// * `color` - An `Rgb` struct representing the color of the grid lines.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated grid line color.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().grid_color(Rgb(200, 200, 200));
    /// ```
    pub fn grid_color(mut self, color: Rgb) -> Self {
        self.grid_color = Some(color);
        self
    }

    /// Sets the width of the grid lines on the axis.
    ///
    /// # Arguments
    ///
    /// * `width` - A `usize` value representing the width of the grid lines.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated grid line width.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().grid_width(1);
    /// ```
    pub fn grid_width(mut self, width: usize) -> Self {
        self.grid_width = Some(width);
        self
    }

    /// Sets whether to show the zero line on the axis.
    ///
    /// # Arguments
    ///
    /// * `bool` - A boolean value indicating whether the zero line should be visible.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated zero line visibility.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().show_zero_line(true);
    /// ```
    pub fn show_zero_line(mut self, bool: bool) -> Self {
        self.show_zero_line = Some(bool);
        self
    }

    /// Sets the color of the zero line on the axis.
    ///
    /// # Arguments
    ///
    /// * `color` - An `Rgb` struct representing the color of the zero line.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated zero line color.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().zero_line_color(Rgb(0, 0, 0));
    /// ```
    pub fn zero_line_color(mut self, color: Rgb) -> Self {
        self.zero_line_color = Some(color);
        self
    }

    /// Sets the width of the zero line on the axis.
    ///
    /// # Arguments
    ///
    /// * `width` - A `usize` value representing the width of the zero line.
    ///
    /// # Returns
    ///
    /// Returns the `Axis` instance with the updated zero line width.
    ///
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new().zero_line_width(2);
    /// ```
    pub fn zero_line_width(mut self, width: usize) -> Self {
        self.zero_line_width = Some(width);
        self
    }
}

impl From<&Axis> for Axis {
    fn from(value: &Axis) -> Self {
        value.clone()
    }
}

/// Enumeration representing the direction of axis ticks.
#[derive(Clone)]
pub enum TickDirection {
    OutSide,
    Inside,
}

impl TickDirection {
    /// Converts `TickDirection` to the corresponding `TicksDirection` from the `plotly` crate.
    ///
    /// # Returns
    ///
    /// Returns the corresponding `TicksDirection`.
    pub fn get_direction(&self) -> TicksDirection {
        match self {
            TickDirection::OutSide => TicksDirection::Outside,
            TickDirection::Inside => TicksDirection::Inside,
        }
    }
}

/// Enumeration representing the position of the axis.
#[derive(Clone)]
pub enum AxisPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl AxisPosition {
    /// Converts `AxisPosition` to the corresponding `AxisSide` from the `plotly` crate.
    ///
    /// # Returns
    ///
    /// Returns the corresponding `AxisSide`.
    pub fn get_position(&self) -> AxisSide {
        match self {
            AxisPosition::Top => AxisSide::Top,
            AxisPosition::Bottom => AxisSide::Bottom,
            AxisPosition::Left => AxisSide::Left,
            AxisPosition::Right => AxisSide::Right,
        }
    }
}

/// Enumeration representing the type of the axis.
#[derive(Clone)]
pub enum AxisType {
    Default,
    Linear,
    Log,
    Date,
    Category,
    MultiCategory,
}

impl AxisType {
    /// Converts `AxisType` to the corresponding `AxisTypePlotly` from the `plotly` crate.
    ///
    /// # Returns
    ///
    /// Returns the corresponding `AxisTypePlotly`.
    pub fn get_type(&self) -> AxisTypePlotly {
        match self {
            AxisType::Default => AxisTypePlotly::Default,
            AxisType::Linear => AxisTypePlotly::Linear,
            AxisType::Log => AxisTypePlotly::Log,
            AxisType::Date => AxisTypePlotly::Date,
            AxisType::Category => AxisTypePlotly::Category,
            AxisType::MultiCategory => AxisTypePlotly::MultiCategory,
        }
    }
}

/// Enumeration representing the format for value exponents on the axis.
#[derive(Clone)]
pub enum ValueExponent {
    None,
    SmallE,
    CapitalE,
    Power,
    SI,
    B,
}

impl ValueExponent {
    /// Converts `ValueExponent` to the corresponding `ExponentFormat` from the `plotly` crate.
    ///
    /// # Returns
    ///
    /// Returns the corresponding `ExponentFormat`.
    pub fn get_exponent(&self) -> ExponentFormat {
        match self {
            ValueExponent::None => ExponentFormat::None,
            ValueExponent::SmallE => ExponentFormat::SmallE,
            ValueExponent::CapitalE => ExponentFormat::CapitalE,
            ValueExponent::Power => ExponentFormat::Power,
            ValueExponent::SI => ExponentFormat::SI,
            ValueExponent::B => ExponentFormat::B,
        }
    }
}
