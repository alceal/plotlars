use plotly::common::{ColorScale, ColorScalePalette};

///
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{ColorBar, HeatMap, Palette, Plot, Text, ValueExponent};
///
/// let dataset = LazyCsvReader::new(PlPath::new("data/heatmap.csv"))
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

impl Palette {
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn to_plotly(&self) -> ColorScale {
        match self {
            Palette::Greys => ColorScale::Palette(ColorScalePalette::Greys),
            Palette::YlGnBu => ColorScale::Palette(ColorScalePalette::YlGnBu),
            Palette::Greens => ColorScale::Palette(ColorScalePalette::Greens),
            Palette::YlOrRd => ColorScale::Palette(ColorScalePalette::YlOrRd),
            Palette::Bluered => ColorScale::Palette(ColorScalePalette::Bluered),
            Palette::RdBu => ColorScale::Palette(ColorScalePalette::RdBu),
            Palette::Reds => ColorScale::Palette(ColorScalePalette::Reds),
            Palette::Blues => ColorScale::Palette(ColorScalePalette::Blues),
            Palette::Picnic => ColorScale::Palette(ColorScalePalette::Picnic),
            Palette::Rainbow => ColorScale::Palette(ColorScalePalette::Rainbow),
            Palette::Portland => ColorScale::Palette(ColorScalePalette::Portland),
            Palette::Jet => ColorScale::Palette(ColorScalePalette::Jet),
            Palette::Hot => ColorScale::Palette(ColorScalePalette::Hot),
            Palette::Blackbody => ColorScale::Palette(ColorScalePalette::Blackbody),
            Palette::Earth => ColorScale::Palette(ColorScalePalette::Earth),
            Palette::Electric => ColorScale::Palette(ColorScalePalette::Electric),
            Palette::Viridis => ColorScale::Palette(ColorScalePalette::Viridis),
            Palette::Cividis => ColorScale::Palette(ColorScalePalette::Cividis),
        }
    }
}
