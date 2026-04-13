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
/// let dataset = DataFrame::new(x.len(), vec![
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
