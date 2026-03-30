use crate::components::Rgb;

/// A structure representing cell formatting for tables.
///
/// The `Cell` struct allows customization of table cells including height,
/// alignment, and fill color.
///
/// # Example
///
/// ```rust
/// use plotlars::{Table, Cell, Plot, Text, Rgb};
/// use polars::prelude::*;
///
/// let dataset = df![
///     "product" => &["Laptop", "Mouse", "Keyboard", "Monitor"],
///     "price" => &[999.99, 29.99, 79.99, 299.99],
///     "stock" => &[15, 250, 87, 42]
/// ]
/// .unwrap();
///
/// let cell = Cell::new()
///     .height(30.0)
///     .align("left")
///     .fill(Rgb(240, 240, 240));
///
/// Table::builder()
///     .data(&dataset)
///     .columns(vec!["product", "price", "stock"])
///     .cell(&cell)
///     .plot_title(Text::from("Product Inventory"))
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/FYYcWRH.png)
#[derive(Clone, Default)]
pub struct Cell {
    pub height: Option<f64>,
    pub align: Option<String>,
    pub fill: Option<Rgb>,
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

    /// Sets the fill color of the cells.
    ///
    /// # Argument
    ///
    /// * `fill` - An `Rgb` value specifying the background color.
    pub fn fill(mut self, fill: Rgb) -> Self {
        self.fill = Some(fill);
        self
    }
}
