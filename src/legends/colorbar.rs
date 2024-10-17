use plotly::{color::Rgb as RgbPlotly, common::ColorBar as ColorBarPlotly};

use crate::{Orientation, Rgb};

#[derive(Clone, Default)]
pub struct ColorBar {
    pub(crate) background_color: Option<Rgb>,
    pub(crate) border_color: Option<Rgb>,
    pub(crate) border_width: Option<usize>,
    pub(crate) orientation: Option<Orientation>,
    pub(crate) x: Option<f64>,
    pub(crate) y: Option<f64>,
}

impl ColorBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn background_color(mut self, color: Rgb) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn border_color(mut self, color: Rgb) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn border_width(mut self, width: usize) -> Self {
        self.border_width = Some(width);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    // pub fn value_exponent(mut self, exponent: ValueExponent) -> Self {
    //     self.value_exponent = Some(exponent);
    //     self
    // }

    pub fn x(mut self, x: f64) -> Self {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: f64) -> Self {
        self.y = Some(y);
        self
    }

    pub fn to_plotly(&self) -> ColorBarPlotly {
        let mut color_bar = ColorBarPlotly::new();

        if let Some(color) = &self.background_color {
            color_bar = color_bar.background_color(RgbPlotly::new(color.0, color.1, color.2));
        }

        if let Some(color) = &self.border_color {
            color_bar = color_bar.border_color(RgbPlotly::new(color.0, color.1, color.2));
        }

        if let Some(width) = self.border_width {
            color_bar = color_bar.border_width(width);
        }

        if let Some(orientation) = &self.orientation {
            color_bar = color_bar.orientation(orientation.get_orientation());
        }

        if let Some(x) = self.x {
            color_bar = color_bar.x(x);
        }

        if let Some(y) = self.y {
            color_bar = color_bar.y(y);
        }

        color_bar
    }
}
