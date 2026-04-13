///
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{ColorBar, HeatMap, Palette, Plot, Text, ValueExponent};
///
/// let dataset = LazyCsvReader::new(PlRefPath::new("data/heatmap.csv"))
///     .finish()
///     .unwrap()
///     .collect()
///     .unwrap();
///
/// HeatMap::builder()
///     .data(&dataset)
///     .x("x")
///     .y("y")
///     .z("z")
///     .color_bar(
///         &ColorBar::new()
///             .length(290)
///             .value_exponent(ValueExponent::None)
///             .separate_thousands(true)
///             .tick_length(5)
///             .tick_step(2500.0)
///     )
///     .color_scale(Palette::Portland)
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/E9LHPAy.png)
#[derive(Clone, Copy)]
pub enum Palette {
    Greys,
    YlGnBu,
    Greens,
    YlOrRd,
    Bluered,
    RdBu,
    Reds,
    Blues,
    Picnic,
    Rainbow,
    Portland,
    Jet,
    Hot,
    Blackbody,
    Earth,
    Electric,
    Viridis,
    Cividis,
}
