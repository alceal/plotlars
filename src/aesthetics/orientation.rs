use plotly::common::Orientation as OrientationPlotly;

/// Enumeration representing the orientation of the legend.
#[derive(Clone)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    /// Converts `Orientation` to the corresponding `OrientationPlotly` from the `plotly` crate.
    ///
    /// # Returns
    ///
    /// Returns the corresponding `OrientationPlotly`.
    pub fn get_orientation(&self) -> OrientationPlotly {
        match self {
            Self::Horizontal => OrientationPlotly::Horizontal,
            Self::Vertical => OrientationPlotly::Vertical,
        }
    }
}
