use crate::components::{Rgb, TickDirection, ValueExponent};

/// A structure representing a customizable axis.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Axis, Plot, Rgb, ScatterPlot, Text, TickDirection};
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
/// let axis = Axis::new()
///     .show_line(true)
///     .tick_direction(TickDirection::OutSide)
///     .value_thousands(true)
///     .show_grid(false);
///
/// ScatterPlot::builder()
///     .data(&dataset)
///     .x("body_mass_g")
///     .y("flipper_length_mm")
///     .group("species")
///     .colors(vec![
///         Rgb(255, 0, 0),
///         Rgb(0, 255, 0),
///         Rgb(0, 0, 255),
///     ])
///     .opacity(0.5)
///     .size(20)
///     .plot_title(
///         Text::from("Scatter Plot")
///             .font("Arial")
///             .size(20)
///             .x(0.045)
///     )
///     .x_title("body mass (g)")
///     .y_title("flipper length (mm)")
///     .legend_title("species")
///     .x_axis(&axis)
///     .y_axis(&axis)
///     .build()
///     .plot();
/// ```
///
/// ![example](https://imgur.com/P24E1ND.png)
#[derive(Default, Clone)]
pub struct Axis {
    pub show_axis: Option<bool>,
    pub axis_side: Option<AxisSide>,
    pub axis_position: Option<f64>,
    pub axis_type: Option<AxisType>,
    pub value_color: Option<Rgb>,
    pub value_range: Option<[f64; 2]>,
    pub value_thousands: Option<bool>,
    pub value_exponent: Option<ValueExponent>,
    pub tick_values: Option<Vec<f64>>,
    pub tick_labels: Option<Vec<String>>,
    pub tick_direction: Option<TickDirection>,
    pub tick_length: Option<usize>,
    pub tick_width: Option<usize>,
    pub tick_color: Option<Rgb>,
    pub tick_angle: Option<f64>,
    pub tick_font: Option<String>,
    pub show_line: Option<bool>,
    pub line_color: Option<Rgb>,
    pub line_width: Option<usize>,
    pub show_grid: Option<bool>,
    pub grid_color: Option<Rgb>,
    pub grid_width: Option<usize>,
    pub show_zero_line: Option<bool>,
    pub zero_line_color: Option<Rgb>,
    pub zero_line_width: Option<usize>,
}

impl Axis {
    /// Creates a new `Axis` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the visibility of the axis.
    ///
    /// # Argument
    ///
    /// * `bool` - A boolean value indicating whether the axis should be visible.
    pub fn show_axis(mut self, bool: bool) -> Self {
        self.show_axis = Some(bool);
        self
    }

    /// Sets the side of the axis.
    ///
    /// # Argument
    ///
    /// * `side` - An `AxisSide` enum value representing the side of the axis.
    pub fn axis_side(mut self, side: AxisSide) -> Self {
        self.axis_side = Some(side);
        self
    }

    /// Sets the position of the axis.
    ///
    /// # Argument
    ///
    /// * `position` - A `f64` value representing the position of the axis.
    pub fn axis_position(mut self, position: f64) -> Self {
        self.axis_position = Some(position);
        self
    }

    /// Sets the type of the axis.
    ///
    /// # Argument
    ///
    /// * `axis_type` - An `AxisType` enum value representing the type of the axis.
    pub fn axis_type(mut self, axis_type: AxisType) -> Self {
        self.axis_type = Some(axis_type);
        self
    }

    /// Sets the color of the axis values.
    ///
    /// # Argument
    ///
    /// * `color` - An `Rgb` struct representing the color of the axis values.
    pub fn value_color(mut self, color: Rgb) -> Self {
        self.value_color = Some(color);
        self
    }

    /// Sets the range of values displayed on the axis.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value of the axis range.
    /// * `max` - The maximum value of the axis range.
    pub fn value_range(mut self, min: f64, max: f64) -> Self {
        self.value_range = Some([min, max]);
        self
    }

    /// Sets whether to use thousands separators for values.
    ///
    /// # Argument
    ///
    /// * `bool` - A boolean value indicating whether to use thousands separators.
    pub fn value_thousands(mut self, bool: bool) -> Self {
        self.value_thousands = Some(bool);
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

    /// Sets the tick values for the axis.
    ///
    /// # Argument
    ///
    /// * `values` - A vector of `f64` values representing the tick values.
    pub fn tick_values(mut self, values: Vec<f64>) -> Self {
        self.tick_values = Some(values);
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

    /// Sets the direction of the axis ticks.
    ///
    /// # Argument
    ///
    /// * `direction` - A `TickDirection` enum value representing the direction of the ticks.
    pub fn tick_direction(mut self, direction: TickDirection) -> Self {
        self.tick_direction = Some(direction);
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

    /// Sets the width of the axis ticks.
    ///
    /// # Argument
    ///
    /// * `width` - A `usize` value representing the width of the ticks.
    pub fn tick_width(mut self, width: usize) -> Self {
        self.tick_width = Some(width);
        self
    }

    /// Sets the color of the axis ticks.
    ///
    /// # Argument
    ///
    /// * `color` - An `Rgb` struct representing the color of the ticks.
    pub fn tick_color(mut self, color: Rgb) -> Self {
        self.tick_color = Some(color);
        self
    }

    /// Sets the angle of the axis ticks.
    ///
    /// # Argument
    ///
    /// * `angle` - A `f64` value representing the angle of the ticks in degrees.
    pub fn tick_angle(mut self, angle: f64) -> Self {
        self.tick_angle = Some(angle);
        self
    }

    /// Sets the font of the axis tick labels.
    ///
    /// # Argument
    ///
    /// * `font` - A value that can be converted into a `String`, representing the font name for the tick labels.
    pub fn tick_font(mut self, font: impl Into<String>) -> Self {
        self.tick_font = Some(font.into());
        self
    }

    /// Sets whether to show the axis line.
    ///
    /// # Argument
    ///
    /// * `bool` - A boolean value indicating whether the axis line should be visible.
    pub fn show_line(mut self, bool: bool) -> Self {
        self.show_line = Some(bool);
        self
    }

    /// Sets the color of the axis line.
    ///
    /// # Argument
    ///
    /// * `color` - An `Rgb` struct representing the color of the axis line.
    pub fn line_color(mut self, color: Rgb) -> Self {
        self.line_color = Some(color);
        self
    }

    /// Sets the width of the axis line.
    ///
    /// # Argument
    ///
    /// * `width` - A `usize` value representing the width of the axis line.
    pub fn line_width(mut self, width: usize) -> Self {
        self.line_width = Some(width);
        self
    }

    /// Sets whether to show the grid lines on the axis.
    ///
    /// # Argument
    ///
    /// * `bool` - A boolean value indicating whether the grid lines should be visible.
    pub fn show_grid(mut self, bool: bool) -> Self {
        self.show_grid = Some(bool);
        self
    }

    /// Sets the color of the grid lines on the axis.
    ///
    /// # Argument
    ///
    /// * `color` - An `Rgb` struct representing the color of the grid lines.
    pub fn grid_color(mut self, color: Rgb) -> Self {
        self.grid_color = Some(color);
        self
    }

    /// Sets the width of the grid lines on the axis.
    ///
    /// # Argument
    ///
    /// * `width` - A `usize` value representing the width of the grid lines.
    pub fn grid_width(mut self, width: usize) -> Self {
        self.grid_width = Some(width);
        self
    }

    /// Sets whether to show the zero line on the axis.
    ///
    /// # Argument
    ///
    /// * `bool` - A boolean value indicating whether the zero line should be visible.
    pub fn show_zero_line(mut self, bool: bool) -> Self {
        self.show_zero_line = Some(bool);
        self
    }

    /// Sets the color of the zero line on the axis.
    ///
    /// # Argument
    ///
    /// * `color` - An `Rgb` struct representing the color of the zero line.
    pub fn zero_line_color(mut self, color: Rgb) -> Self {
        self.zero_line_color = Some(color);
        self
    }

    /// Sets the width of the zero line on the axis.
    ///
    /// # Argument
    ///
    /// * `width` - A `usize` value representing the width of the zero line.
    pub fn zero_line_width(mut self, width: usize) -> Self {
        self.zero_line_width = Some(width);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_all_none() {
        let a = Axis::new();
        assert!(a.show_axis.is_none());
        assert!(a.show_grid.is_none());
        assert!(a.value_range.is_none());
        assert!(a.tick_labels.is_none());
        assert!(a.show_line.is_none());
    }

    #[test]
    fn test_show_axis() {
        let a = Axis::new().show_axis(true);
        assert_eq!(a.show_axis, Some(true));
        assert!(a.show_grid.is_none());
        assert!(a.value_range.is_none());
    }

    #[test]
    fn test_show_grid() {
        let a = Axis::new().show_grid(false);
        assert_eq!(a.show_grid, Some(false));
        assert!(a.show_axis.is_none());
    }

    #[test]
    fn test_value_range() {
        let a = Axis::new().value_range(0.0, 100.0);
        let range = a.value_range.unwrap();
        assert!((range[0] - 0.0).abs() < 1e-6);
        assert!((range[1] - 100.0).abs() < 1e-6);
    }

    #[test]
    fn test_tick_labels_converts() {
        let a = Axis::new().tick_labels(vec!["a", "b"]);
        let labels = a.tick_labels.unwrap();
        assert_eq!(labels, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_builder_chaining() {
        let a = Axis::new()
            .show_axis(true)
            .show_grid(false)
            .show_line(true)
            .value_range(1.0, 50.0)
            .tick_labels(vec!["x", "y", "z"]);
        assert_eq!(a.show_axis, Some(true));
        assert_eq!(a.show_grid, Some(false));
        assert_eq!(a.show_line, Some(true));
        assert_eq!(a.value_range.unwrap().len(), 2);
        assert_eq!(a.tick_labels.unwrap().len(), 3);
    }
}

/// Enumeration representing the position of the axis.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Axis, AxisSide, Legend, Line, Plot, Rgb, Shape, Text, TimeSeriesPlot};
///
/// let dataset = LazyCsvReader::new(PlRefPath::new("data/revenue_and_cost.csv"))
///     .finish()
///     .unwrap()
///     .select([
///         col("Date").cast(DataType::String),
///         col("Revenue").cast(DataType::Int32),
///         col("Cost").cast(DataType::Int32),
///     ])
///     .collect()
///     .unwrap();
///
/// TimeSeriesPlot::builder()
///     .data(&dataset)
///     .x("Date")
///     .y("Revenue")
///     .additional_series(vec!["Cost"])
///     .size(8)
///     .colors(vec![Rgb(255, 0, 0), Rgb(0, 255, 0)])
///     .lines(vec![Line::Dash, Line::Solid])
///     .shapes(vec![Shape::Circle, Shape::Square])
///     .plot_title(
///         Text::from("Time Series Plot")
///             .font("Arial")
///             .size(18)
///     )
///     .y_axis(
///         &Axis::new()
///             .axis_side(AxisSide::Right)
///     )
///     .legend(
///         &Legend::new()
///             .x(0.05)
///             .y(0.9)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/Ok0c5R5.png)
#[derive(Clone)]
pub enum AxisSide {
    Top,
    Bottom,
    Left,
    Right,
}

/// Enumeration representing the type of the axis.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Axis, AxisType, LinePlot, Plot};
///
/// let linear_values = vec![
///     1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
///     20, 30, 40, 50, 60, 70, 80, 90, 100,
///     200, 300, 400, 500, 600, 700, 800, 900, 1000
/// ];
///
/// let logarithms = vec![
///     0.0000, 0.3010, 0.4771, 0.6021, 0.6990,
///     0.7782, 0.8451, 0.9031, 0.9542, 1.0000,
///     1.3010, 1.4771, 1.6021, 1.6990, 1.7782,
///     1.8451, 1.9031, 1.9542, 2.0000,
///     2.3010, 2.4771, 2.6021, 2.6990,
///     2.7782, 2.8451, 2.9031, 2.9542, 3.0000
/// ];
///
/// let dataset = DataFrame::new(linear_values.len(), vec![
///     Column::new("linear_values".into(), linear_values),
///     Column::new("logarithms".into(), logarithms),
/// ]).unwrap();
///
/// let axis = Axis::new()
///     .axis_type(AxisType::Log)
///     .show_line(true);
///
/// LinePlot::builder()
///     .data(&dataset)
///     .x("linear_values")
///     .y("logarithms")
///     .y_title("log₁₀ x")
///     .x_title("x")
///     .y_axis(&axis)
///     .x_axis(&axis)
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/rjNNO5q.png)
#[derive(Clone)]
pub enum AxisType {
    Default,
    Linear,
    Log,
    Date,
    Category,
    MultiCategory,
}
