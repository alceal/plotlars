use plotly::contour::{
    Coloring as ColoringPlotly,
    Contours as ContoursPlotly,
};

/// A structure representing configurable contour settings.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Coloring, ContourPlot, Contours, Palette, Plot};
///
/// let dataset = df!(
///         "x" => &[0.0, 0.0, 0.0, 2.5, 2.5, 2.5, 5.0, 5.0, 5.0],
///         "y" => &[0.0, 7.5, 15.0, 0.0, 7.5, 15.0, 0.0, 7.5, 15.0],
///         "z" => &[0.0, 5.0, 10.0, 5.0, 2.5, 5.0, 10.0, 0.0, 0.0],
///     )
///     .unwrap();
///
/// ContourPlot::builder()
///     .data(&dataset)
///     .x("x")
///     .y("y")
///     .z("z")
///     .contours(
///         &Contours::new()
///         .coloring(Coloring::HeatMap)
///     )
///     .color_scale(Palette::Viridis)
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/H6dYuky.png)
#[derive(Default)]
pub struct Contours {
    coloring: Option<Coloring>,
    show_lines: Option<bool>,
}

impl Contours {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn to_plotly(&self) -> ContoursPlotly {
        let mut contours = ContoursPlotly::new();

        if let Some(coloring) = &self.coloring {
            contours = contours.coloring(coloring.to_plotly());
        }

        contours
    }

    /// Sets the coloring mode applied to the contour levels.
    ///
    /// # Argument
    ///
    /// * `coloring` - A `Coloring` enum value specifying how the contours should be colored
    pub fn coloring(mut self, coloring: Coloring) -> Self {
        self.coloring = Some(coloring);
        self
    }
    /// Sets whether to draw isoline curves on top of the colored contour regions.
    ///
    /// # Argument
    ///
    /// * `show_lines` - A boolean value indicating whether contour lines should be visible.
    pub fn show_lines(mut self, show_lines: bool) -> Self {
        self.show_lines = Some(show_lines);
        self
    }
}

/// Enumeration representing the coloring strategy applied to contour levels.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{ Coloring, ContourPlot, Contours, Palette, Plot };
///
/// let dataset = df!(
///     "x" => &[0.0, 0.0, 0.0, 2.5, 2.5, 2.5, 5.0, 5.0, 5.0],
///     "y" => &[0.0, 7.5, 15.0, 0.0, 7.5, 15.0, 0.0, 7.5, 15.0],
///     "z" => &[0.0, 5.0, 10.0, 5.0, 2.5, 5.0, 10.0, 0.0, 0.0],
/// ).unwrap();
///
/// ContourPlot::builder()
///     .data(&dataset)
///     .x("x")
///     .y("y")
///     .z("z")
///     .contours(
///         &Contours::new()
///             .coloring(Coloring::Lines)
///     )
///     .color_scale(Palette::Viridis)
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/hFD2A82.png)
pub enum Coloring {
    Fill,
    HeatMap,
    Lines,
    None,
}

impl Coloring {
    pub(crate) fn to_plotly(&self) -> ColoringPlotly {
        match self {
            Coloring::Fill => ColoringPlotly::Fill,
            Coloring::HeatMap => ColoringPlotly::HeatMap,
            Coloring::Lines => ColoringPlotly::Lines,
            Coloring::None => ColoringPlotly::None,
        }
    }
}
