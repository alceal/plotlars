use plotly::common::{Direction as DirectionPlotly, Line as LinePlotly};

use crate::components::Rgb;

/// A structure representing the styling for candlestick directions (increasing/decreasing).
///
/// The `Direction` struct allows customization of how candlestick lines appear when the closing price
/// is higher (increasing) or lower (decreasing) than the opening price. This includes setting
/// the line color and width for the candlesticks.
///
/// Note: Fill color is not currently supported by the underlying plotly library.
///
/// # Example
///
/// ```rust
/// use plotlars::{CandlestickPlot, Direction, Plot, Rgb};
/// use polars::prelude::*;
///
/// let dates = vec!["2024-01-01", "2024-01-02", "2024-01-03"];
/// let open_prices = vec![100.0, 102.5, 101.0];
/// let high_prices = vec![103.0, 104.0, 103.5];
/// let low_prices = vec![99.0, 101.5, 100.0];
/// let close_prices = vec![102.5, 101.0, 103.5];
///
/// let stock_data = df! {
///     "date" => dates,
///     "open" => open_prices,
///     "high" => high_prices,
///     "low" => low_prices,
///     "close" => close_prices,
/// }
/// .unwrap();
///
/// let increasing = Direction::new()
///     .line_color(Rgb(0, 150, 255))
///     .line_width(2.0);
///
/// let decreasing = Direction::new()
///     .line_color(Rgb(200, 0, 100))
///     .line_width(2.0);
///
/// CandlestickPlot::builder()
///     .data(&stock_data)
///     .dates("date")
///     .open("open")
///     .high("high")
///     .low("low")
///     .close("close")
///     .increasing(&increasing)
///     .decreasing(&decreasing)
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/SygxOCm.png)
#[derive(Clone, Default)]
pub struct Direction {
    pub(crate) line_color: Option<Rgb>,
    pub(crate) line_width: Option<f64>,
}

impl Direction {
    /// Creates a new `Direction` instance with default settings.
    ///
    /// # Returns
    ///
    /// A new `Direction` instance with no customizations applied.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the line color for the candlestick outline and wicks.
    ///
    /// # Arguments
    ///
    /// * `color` - An `Rgb` color for the candlestick lines.
    ///
    /// # Returns
    ///
    /// The modified `Direction` instance for method chaining.
    pub fn line_color(mut self, color: Rgb) -> Self {
        self.line_color = Some(color);
        self
    }

    /// Sets the line width for the candlestick outline and wicks.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the candlestick lines in pixels.
    ///
    /// # Returns
    ///
    /// The modified `Direction` instance for method chaining.
    pub fn line_width(mut self, width: f64) -> Self {
        self.line_width = Some(width);
        self
    }

    /// Converts the `Direction` to plotly's Direction::Increasing type.
    ///
    /// # Returns
    ///
    /// A `DirectionPlotly::Increasing` instance with the configured settings.
    pub(crate) fn to_plotly_increasing(&self) -> DirectionPlotly {
        let mut line = LinePlotly::new();

        if let Some(line_color) = &self.line_color {
            line = line.color(line_color.to_plotly());
        }

        if let Some(width) = self.line_width {
            line = line.width(width);
        }

        DirectionPlotly::Increasing { line }
    }

    /// Converts the `Direction` to plotly's Direction::Decreasing type.
    ///
    /// # Returns
    ///
    /// A `DirectionPlotly::Decreasing` instance with the configured settings.
    pub(crate) fn to_plotly_decreasing(&self) -> DirectionPlotly {
        let mut line = LinePlotly::new();

        if let Some(line_color) = &self.line_color {
            line = line.color(line_color.to_plotly());
        }

        if let Some(width) = self.line_width {
            line = line.width(width);
        }

        DirectionPlotly::Decreasing { line }
    }
}
