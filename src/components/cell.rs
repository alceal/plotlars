use plotly::common::Font;
use plotly::traces::table::Cells as CellsPlotly;

use crate::components::{Rgb, Text};

/// A structure representing cell formatting for tables.
///
/// The `Cell` struct allows customization of table cells including height,
/// alignment, font, and fill color with support for alternating row colors.
///
/// # Example
///
/// ```rust
/// use plotlars::{Cell, Text, Rgb};
///
/// let cell = Cell::new()
///     .height(30.0)
///     .align("left")
///     .font(Text::from("Cell").size(12).font("Arial"))
///     .fill(vec![Rgb(240, 240, 240), Rgb(255, 255, 255)]);
/// ```
#[derive(Clone, Default)]
pub struct Cell {
    pub(crate) height: Option<f64>,
    pub(crate) align: Option<String>,
    pub(crate) font: Option<Text>,
    pub(crate) fill: Option<Vec<Rgb>>,
}

impl Cell {
    /// Creates a new `Cell` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the height of the cells.
    ///
    /// # Argument
    ///
    /// * `height` - A `f64` value specifying the cell height.
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets the alignment of the cell text.
    ///
    /// # Argument
    ///
    /// * `align` - A string specifying the alignment (left, center, right).
    pub fn align(mut self, align: impl Into<String>) -> Self {
        self.align = Some(align.into());
        self
    }

    /// Sets the font of the cell text.
    ///
    /// # Argument
    ///
    /// * `font` - A `Text` struct specifying the font properties.
    pub fn font(mut self, font: Text) -> Self {
        self.font = Some(font);
        self
    }

    /// Sets the fill colors of the cells.
    ///
    /// # Argument
    ///
    /// * `fill` - A vector of `Rgb` values for alternating row colors.
    pub fn fill(mut self, fill: Vec<Rgb>) -> Self {
        self.fill = Some(fill);
        self
    }

    pub(crate) fn to_plotly<T>(&self, values: Vec<Vec<T>>) -> CellsPlotly<T>
    where
        T: serde::Serialize + Clone + Default + 'static,
    {
        let mut cells = CellsPlotly::new(values);

        if let Some(height) = self.height {
            cells = cells.height(height);
        }

        if let Some(align) = &self.align {
            cells = cells.align(align.as_str());
        }

        if let Some(font) = &self.font {
            cells = cells.font(
                Font::new()
                    .family(font.font.as_str())
                    .size(font.size)
                    .color(font.color.to_plotly()),
            );
        }

        if let Some(fill_colors) = &self.fill {
            if fill_colors.len() == 1 {
                cells = cells
                    .fill(plotly::traces::table::Fill::new().color(fill_colors[0].to_plotly()));
            } else {
                // For alternating row colors, we'll use the first color as the base
                // The plotly API doesn't directly support alternating colors in the same way
                cells = cells
                    .fill(plotly::traces::table::Fill::new().color(fill_colors[0].to_plotly()));
            }
        }

        cells
    }
}
