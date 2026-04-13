use crate::components::Rgb;

/// A structure representing header formatting for tables.
///
/// The `Header` struct allows customization of table headers including custom values,
/// height, alignment, font, and fill color.
///
/// # Example
///
/// ```rust
/// use plotlars::{Table, Header, Plot, Text, Rgb};
/// use polars::prelude::*;
///
/// let dataset = df![
///     "name" => &["Alice", "Bob", "Charlie"],
///     "age" => &[25, 30, 35],
///     "city" => &["New York", "London", "Tokyo"]
/// ]
/// .unwrap();
///
/// let header = Header::new()
///     .values(vec!["Full Name", "Years", "Location"])
///     .height(40.0)
///     .align("center")
///     .font("Arial")
///     .fill(Rgb(200, 200, 200));
///
/// Table::builder()
///     .data(&dataset)
///     .columns(vec!["name", "age", "city"])
///     .header(&header)
///     .plot_title(Text::from("Employee Information"))
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/J2XhcUt.png)
#[derive(Clone, Default)]
pub struct Header {
    pub values: Option<Vec<String>>,
    pub height: Option<f64>,
    pub align: Option<String>,
    pub font: Option<String>,
    pub fill: Option<Rgb>,
}

impl Header {
    /// Creates a new `Header` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets custom header values.
    ///
    /// # Argument
    ///
    /// * `values` - A vector of string slices representing custom header names.
    pub fn values(mut self, values: Vec<&str>) -> Self {
        self.values = Some(values.into_iter().map(|s| s.to_string()).collect());
        self
    }

    /// Sets the height of the header.
    ///
    /// # Argument
    ///
    /// * `height` - A `f64` value specifying the header height.
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets the alignment of the header text.
    ///
    /// # Argument
    ///
    /// * `align` - A string specifying the alignment (left, center, right).
    pub fn align(mut self, align: impl Into<String>) -> Self {
        self.align = Some(align.into());
        self
    }

    /// Sets the font family of the header text.
    ///
    /// # Argument
    ///
    /// * `font` - A string slice specifying the font family name.
    pub fn font(mut self, font: &str) -> Self {
        self.font = Some(font.to_string());
        self
    }

    /// Sets the fill color of the header.
    ///
    /// # Argument
    ///
    /// * `fill` - An `Rgb` value specifying the background color.
    pub fn fill(mut self, fill: Rgb) -> Self {
        self.fill = Some(fill);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let header = Header::new();
        assert!(header.values.is_none());
        assert!(header.height.is_none());
        assert!(header.align.is_none());
        assert!(header.font.is_none());
        assert!(header.fill.is_none());
    }

    #[test]
    fn test_values() {
        let header = Header::new().values(vec!["A", "B"]);
        let values = header.values.unwrap();
        assert_eq!(values, vec!["A".to_string(), "B".to_string()]);
    }

    #[test]
    fn test_height() {
        let header = Header::new().height(30.0);
        assert!((header.height.unwrap() - 30.0).abs() < 1e-6);
    }

    #[test]
    fn test_align() {
        let header = Header::new().align("left");
        assert_eq!(header.align, Some("left".to_string()));
    }

    #[test]
    fn test_fill() {
        let header = Header::new().fill(Rgb(200, 200, 200));
        let fill = header.fill.unwrap();
        assert_eq!(fill.0, 200);
        assert_eq!(fill.1, 200);
        assert_eq!(fill.2, 200);
    }
}
