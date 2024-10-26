use plotly::common::DashType;

/// An enumeration representing different styles of lines that can be used in plots.
///
/// # Example
///
/// ```rust
/// use plotlars::{Legend, Line, Plot, Rgb, TimeSeriesPlot};
///
/// let dataset = LazyCsvReader::new("data/revenue_and_cost.csv")
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
///     .legend(
///         &Legend::new()
///             .x(0.05)
///             .y(0.9)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/y6ZyypZ.png)
#[derive(Clone, Copy)]
pub enum Line {
    Solid,
    Dot,
    Dash,
    LongDash,
    DashDot,
    LongDashDot,
}

impl Line {
    pub(crate) fn get_line_type(&self) -> DashType {
        match self {
            Line::Solid => DashType::Solid,
            Line::Dot => DashType::Dot,
            Line::Dash => DashType::Dash,
            Line::LongDash => DashType::LongDash,
            Line::DashDot => DashType::DashDot,
            Line::LongDashDot => DashType::LongDashDot,
        }
    }
}
