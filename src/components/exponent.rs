use plotly::common::ExponentFormat;

/// An enumeration representing the format for value exponents on the axis.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Axis, Plot, TimeSeriesPlot, ValueExponent};
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
///     .y_axis(
///         &Axis::new()
///             .value_exponent(ValueExponent::SmallE)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/I6gYYkb.png)
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
    pub(crate) fn to_plotly(&self) -> ExponentFormat {
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
