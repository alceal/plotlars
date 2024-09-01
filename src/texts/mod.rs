use crate::colors::Rgb;

/// A structure representing text with customizable content, font, size, and color.
pub struct Text {
    pub(crate) content: String,
    pub(crate) font: String,
    pub(crate) size: usize,
    pub(crate) color: Rgb,
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Default for Text {
    fn default() -> Self {
        Text {
            content: String::new(),
            font: String::new(),
            size: 0,
            color: Rgb::default(),
            x: 0.5,
            y: 0.9,
        }
    }
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

    /// Sets the x-coordinate for the object.
    ///
    /// # Arguments
    ///
    /// * `x` - A `f64` value representing the x-coordinate.
    ///
    /// # Returns
    ///
    /// Returns the modified object with the updated x-coordinate.
    pub fn x(mut self, x: f64) -> Self {
        self.x = x;
        self
    }

    /// Sets the y-coordinate for the object.
    ///
    /// # Arguments
    ///
    /// * `y` - A `f64` value representing the y-coordinate.
    ///
    /// # Returns
    ///
    /// Returns the modified object with the updated y-coordinate.
    pub fn y(mut self, y: f64) -> Self {
        self.y = y;
        self
    }
}

impl From<&str> for Text {
    fn from(content: &str) -> Self {
        Self::from(content.to_string())
    }
}

impl From<String> for Text {
    fn from(content: String) -> Self {
        Self::from(content)
    }
}

impl From<&String> for Text {
    fn from(content: &String) -> Self {
        Self::from(content)
    }
}
