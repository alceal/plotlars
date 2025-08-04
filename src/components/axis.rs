use plotly::{
    common::{AxisSide as AxisSidePlotly, Font},
    layout::{Axis as AxisPlotly, AxisType as AxisTypePlotly},
};

use crate::components::{Rgb, Text, TickDirection, ValueExponent};

/// A structure representing a customizable axis.
///
/// # Example
///
/// ```rust
/// use plotlars::{Axis, Plot, Rgb, ScatterPlot, Text, TickDirection};
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
    pub(crate) show_axis: Option<bool>,
    pub(crate) axis_side: Option<AxisSide>,
    pub(crate) axis_position: Option<f64>,
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
    /// # Argument
    ///
    /// * `range` - A vector of `f64` values representing the range of the axis.
    pub fn value_range(mut self, range: Vec<f64>) -> Self {
        self.value_range = Some(range);
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

    pub(crate) fn set_axis(
        title: Option<Text>,
        format: &Self,
        overlaying: Option<&str>,
    ) -> AxisPlotly {
        let mut axis = AxisPlotly::new();

        if let Some(title) = title {
            axis = axis.title(title.to_plotly());
        }
        axis = Self::set_format(axis, format, overlaying);

        axis
    }

    fn set_format(mut axis: AxisPlotly, format: &Self, overlaying: Option<&str>) -> AxisPlotly {
        if let Some(overlaying) = overlaying {
            axis = axis.overlaying(overlaying);
        }

        if let Some(visible) = format.show_axis {
            axis = axis.visible(visible.to_owned());
        }

        if let Some(axis_position) = &format.axis_side {
            axis = axis.side(axis_position.to_plotly());
        }

        if let Some(axis_type) = &format.axis_type {
            axis = axis.type_(axis_type.to_plotly());
        }

        if let Some(color) = format.value_color {
            axis = axis.color(color.to_plotly());
        }

        if let Some(range) = &format.value_range {
            axis = axis.range(range.to_owned());
        }

        if let Some(thousands) = format.value_thousands {
            axis = axis.separate_thousands(thousands.to_owned());
        }

        if let Some(exponent) = &format.value_exponent {
            axis = axis.exponent_format(exponent.to_plotly());
        }

        if let Some(range_values) = &format.tick_values {
            axis = axis.tick_values(range_values.to_owned());
        }

        if let Some(tick_text) = &format.tick_labels {
            axis = axis.tick_text(tick_text.to_owned());
        }

        if let Some(tick_direction) = &format.tick_direction {
            axis = axis.ticks(tick_direction.to_plotly_tickdirection());
        }

        if let Some(tick_length) = format.tick_length {
            axis = axis.tick_length(tick_length.to_owned());
        }

        if let Some(tick_width) = format.tick_width {
            axis = axis.tick_width(tick_width.to_owned());
        }

        if let Some(color) = format.tick_color {
            axis = axis.tick_color(color.to_plotly());
        }

        if let Some(tick_angle) = format.tick_angle {
            axis = axis.tick_angle(tick_angle.to_owned());
        }

        if let Some(font) = &format.tick_font {
            axis = axis.tick_font(Font::new().family(font.as_str()));
        }

        if let Some(show_line) = format.show_line {
            axis = axis.show_line(show_line.to_owned());
        }

        if let Some(color) = format.line_color {
            axis = axis.line_color(color.to_plotly());
        }

        if let Some(line_width) = format.line_width {
            axis = axis.line_width(line_width.to_owned());
        }

        if let Some(show_grid) = format.show_grid {
            axis = axis.show_grid(show_grid.to_owned());
        }

        if let Some(color) = format.grid_color {
            axis = axis.grid_color(color.to_plotly());
        }

        if let Some(grid_width) = format.grid_width {
            axis = axis.grid_width(grid_width.to_owned());
        }

        if let Some(show_zero_line) = format.show_zero_line {
            axis = axis.zero_line(show_zero_line.to_owned());
        }

        if let Some(color) = format.zero_line_color {
            axis = axis.zero_line_color(color.to_plotly());
        }

        if let Some(zero_line_width) = format.zero_line_width {
            axis = axis.zero_line_width(zero_line_width.to_owned());
        }

        if let Some(axis_position) = format.axis_position {
            axis = axis.position(axis_position.to_owned());
        }

        axis
    }
}

/// Enumeration representing the position of the axis.
///
/// # Example
///
/// ```rust
/// use plotlars::{Axis, AxisSide, Legend, Line, Plot, Rgb, Shape, Text, TimeSeriesPlot};
///
/// let dataset = LazyCsvReader::new(PlPath::new("data/revenue_and_cost.csv"))
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
///     .with_shape(true)
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

impl AxisSide {
    pub(crate) fn to_plotly(&self) -> AxisSidePlotly {
        match self {
            AxisSide::Top => AxisSidePlotly::Top,
            AxisSide::Bottom => AxisSidePlotly::Bottom,
            AxisSide::Left => AxisSidePlotly::Left,
            AxisSide::Right => AxisSidePlotly::Right,
        }
    }
}

/// Enumeration representing the type of the axis.
///
/// # Example
///
/// ```rust
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
/// let dataset = DataFrame::new(vec![
///     Series::new("linear_values".into(), linear_values),
///     Series::new("logarithms".into(), logarithms),
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

impl AxisType {
    pub(crate) fn to_plotly(&self) -> AxisTypePlotly {
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
