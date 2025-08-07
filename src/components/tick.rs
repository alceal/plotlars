use plotly::{common::Ticks, layout::TicksDirection};

/// Enumeration representing the direction of axis ticks.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Axis, Plot, ScatterPlot, TickDirection};
///
/// let x = vec![1];
/// let y  = vec![1];
///
/// let dataset = DataFrame::new(vec![
///     Column::new("x".into(), x),
///     Column::new("y".into(), y),
/// ]).unwrap();
///
/// ScatterPlot::builder()
///     .data(&dataset)
///     .x("x")
///     .y("y")
///     .x_axis(
///         &Axis::new()
///             .tick_direction(TickDirection::OutSide)
///     )
///     .y_axis(
///         &Axis::new()
///             .tick_direction(TickDirection::InSide)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/9DSwJnx.png)
#[derive(Clone)]
pub enum TickDirection {
    OutSide,
    InSide,
    None,
}

impl TickDirection {
    pub(crate) fn to_plotly_tickdirection(&self) -> TicksDirection {
        match self {
            TickDirection::OutSide => TicksDirection::Outside,
            TickDirection::InSide => TicksDirection::Inside,
            TickDirection::None => TicksDirection::Outside,
        }
    }

    pub(crate) fn to_plotly_ticks(&self) -> Ticks {
        match self {
            TickDirection::OutSide => Ticks::Outside,
            TickDirection::InSide => Ticks::Inside,
            TickDirection::None => Ticks::None,
        }
    }
}
