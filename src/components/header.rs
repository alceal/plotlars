use plotly::common::Font;
use plotly::traces::table::Header as HeaderPlotly;

use crate::components::{Rgb, Text};

/// A structure representing header formatting for tables.
///
/// The `Header` struct allows customization of table headers including custom values,
/// height, alignment, font, and fill color.
///
/// # Example
///
/// ```rust
/// use plotlars::{Header, Text, Rgb};
///
/// let header = Header::new()
///     .values(vec!["Full Name", "Years", "Location"])
///     .height(40.0)
///     .align("center")
///     .font(Text::from("Header").size(14).font("Arial"))
///     .fill(Rgb(200, 200, 200));
/// ```
#[derive(Clone, Default)]
pub struct Header {
    pub(crate) values: Option<Vec<String>>,
    pub(crate) height: Option<f64>,
    pub(crate) align: Option<String>,
    pub(crate) font: Option<Text>,
    pub(crate) fill: Option<Rgb>,
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

    /// Sets the font of the header text.
    ///
    /// # Argument
    ///
    /// * `font` - A `Text` struct specifying the font properties.
    pub fn font(mut self, font: Text) -> Self {
        self.font = Some(font);
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

    pub(crate) fn to_plotly<T>(&self, default_values: Vec<T>) -> HeaderPlotly<T>
    where
        T: serde::Serialize + Clone + Default + 'static,
    {
        let mut header = HeaderPlotly::new(default_values);

        if let Some(height) = self.height {
            header = header.height(height);
        }

        if let Some(align) = &self.align {
            header = header.align(align.as_str());
        }

        if let Some(font) = &self.font {
            header = header.font(
                Font::new()
                    .family(font.font.as_str())
                    .size(font.size)
                    .color(font.color.to_plotly()),
            );
        }

        if let Some(fill) = &self.fill {
            header = header.fill(plotly::traces::table::Fill::new().color(fill.to_plotly()));
        }

        header
    }
}
