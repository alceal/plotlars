use plotly::common::{ColorScale, ColorScalePalette};

///
///
/// # Example
///
/// ```rust
/// use plotlars::{ColorBar, HeatMap, Palette, Plot, Text, ValueExponent};
///
/// let dataset = LazyCsvReader::new("../data/heatmap.csv")
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
        ColorScale::Palette(match self {
            Palette::Greys => ColorScalePalette::Greys,
            Palette::YlGnBu => ColorScalePalette::YlGnBu,
            Palette::Greens => ColorScalePalette::Greens,
            Palette::YlOrRd => ColorScalePalette::YlOrRd,
            Palette::Bluered => ColorScalePalette::Bluered,
            Palette::RdBu => ColorScalePalette::RdBu,
            Palette::Reds => ColorScalePalette::Reds,
            Palette::Blues => ColorScalePalette::Blues,
            Palette::Picnic => ColorScalePalette::Picnic,
            Palette::Rainbow => ColorScalePalette::Rainbow,
            Palette::Portland => ColorScalePalette::Portland,
            Palette::Jet => ColorScalePalette::Jet,
            Palette::Hot => ColorScalePalette::Hot,
            Palette::Blackbody => ColorScalePalette::Blackbody,
            Palette::Earth => ColorScalePalette::Earth,
            Palette::Electric => ColorScalePalette::Electric,
            Palette::Viridis => ColorScalePalette::Viridis,
            Palette::Cividis => ColorScalePalette::Cividis,
        })
    }
}
