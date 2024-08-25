use crate::colors::Rgb;

/// A structure representing text with customizable content, font, size, and color.
#[derive(Default)]
pub struct Text {
    pub content: String,
    pub font: String,
    pub size: usize,
    pub color: Rgb,
}

impl Text {
    /// Creates a new `Text` instance from the given content.
    ///
    /// # Arguments
    ///
    /// * `content` - A value that can be converted into a `String`, representing the textual content.
    ///
    /// # Returns
    ///
    /// Returns a `Text` instance with the specified content and default font, size, and color.
    ///
    /// # Example
    ///
    /// ```
    /// let text = Text::from("Hello, World!");
    /// ```
    pub fn from(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    /// Sets the font of the `Text` instance.
    ///
    /// # Arguments
    ///
    /// * `font` - A value that can be converted into a `String`, representing the font name.
    ///
    /// # Returns
    ///
    /// Returns the `Text` instance with the updated font.
    ///
    /// # Example
    ///
    /// ```
    /// let text = Text::from("Hello, World!").font("Arial");
    /// ```
    pub fn font(mut self, font: impl Into<String>) -> Self {
        self.font = font.into();
        self
    }

    /// Sets the size of the `Text` instance.
    ///
    /// # Arguments
    ///
    /// * `size` - A `usize` value representing the font size.
    ///
    /// # Returns
    ///
    /// Returns the `Text` instance with the updated size.
    ///
    /// # Example
    ///
    /// ```
    /// let text = Text::from("Hello, World!").size(24);
    /// ```
    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }

    /// Sets the color of the `Text` instance.
    ///
    /// # Arguments
    ///
    /// * `color` - An `Rgb` struct representing the color of the text.
    ///
    /// # Returns
    ///
    /// Returns the `Text` instance with the updated color.
    ///
    /// # Example
    ///
    /// ```
    /// let text = Text::from("Hello, World!").color(Rgb(255, 0, 0));
    /// ```
    pub fn color(mut self, color: Rgb) -> Self {
        self.color = color;
        self
    }
}
