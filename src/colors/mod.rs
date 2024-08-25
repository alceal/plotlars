use plotly::color::Color;
use serde::Serialize;

/// A structure representing an RGB color with red, green, and blue components.
#[derive(Debug, Default, Serialize, Clone)]
pub struct Rgb(
    /// Red component
    pub u8,
    /// Green component
    pub u8,
    /// Blue component
    pub u8,
);

impl Color for Rgb {}
