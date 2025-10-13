use crate::components::{Rgb, Text};
use std::cmp::Ordering;

/// Controls axis scaling behavior across facets in a faceted plot.
///
/// This enum determines whether facets share the same axis ranges or have independent scales.
/// The behavior is similar to ggplot2's `scales` parameter in `facet_wrap()`.
#[derive(Clone, Default)]
pub enum FacetScales {
    #[default]
    Fixed,
    Free,
    FreeX,
    FreeY,
}

/// A structure representing facet configuration for creating small multiples.
///
/// The `FacetConfig` struct allows customization of faceted plots including grid layout,
/// scale behavior, spacing, title styling, custom ordering, and highlighting options.
/// Faceting splits data by a categorical variable to create multiple subplots arranged
/// in a grid, making it easy to compare patterns across categories.
///
/// # Example
///
/// ```rust
/// use plotlars::{ScatterPlot, FacetConfig, FacetScales, Plot, Rgb, Text};
/// use polars::prelude::*;
///
/// let dataset = df![
///     "species" => &["Adelie", "Adelie", "Chinstrap", "Chinstrap", "Gentoo", "Gentoo"],
///     "body_mass_g" => &[3750, 3800, 3500, 3650, 5000, 5200],
///     "flipper_length_mm" => &[181, 186, 195, 201, 217, 221]
/// ]
/// .unwrap();
///
/// let facet_config = FacetConfig::new()
///     .ncol(3)
///     .scales(FacetScales::Free)
///     .x_gap(0.05)
///     .y_gap(0.08)
///     .title_style(Text::from("").size(14).color(Rgb(50, 50, 50)));
///
/// ScatterPlot::builder()
///     .data(&dataset)
///     .x("body_mass_g")
///     .y("flipper_length_mm")
///     .facet("species")
///     .facet_config(&facet_config)
///     .plot_title("Penguin Measurements by Species")
///     .x_title("body mass (g)")
///     .y_title("flipper length (mm)")
///     .opacity(0.7)
///     .size(10)
///     .color(Rgb(65, 105, 225))
///     .build()
///     .plot();
/// ```
#[derive(Clone, Default)]
pub struct FacetConfig {
    pub(crate) ncol: Option<usize>,
    pub(crate) nrow: Option<usize>,
    pub(crate) scales: FacetScales,
    pub(crate) x_gap: Option<f64>,
    pub(crate) y_gap: Option<f64>,
    pub(crate) title_style: Option<Text>,
    pub(crate) sorter: Option<fn(&str, &str) -> Ordering>,
    pub(crate) highlight_facet: bool,
    pub(crate) unhighlighted_color: Option<Rgb>,
}

impl FacetConfig {
    /// Creates a new `FacetConfig` instance with default values.
    ///
    /// By default, the grid dimensions are automatically calculated, scales are fixed
    /// across all facets, and highlighting is disabled.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of columns in the facet grid.
    ///
    /// When specified, the grid will have exactly this many columns, and the number
    /// of rows will be calculated automatically based on the number of facets. If not
    /// specified, both dimensions are calculated automatically.
    ///
    /// # Argument
    ///
    /// * `ncol` - A `usize` value specifying the number of columns (must be greater than 0).
    ///
    /// # Panics
    ///
    /// Panics if `ncol` is 0.
    pub fn ncol(mut self, ncol: usize) -> Self {
        if ncol == 0 {
            panic!("ncol must be greater than 0");
        }
        self.ncol = Some(ncol);
        self
    }

    /// Sets the number of rows in the facet grid.
    ///
    /// When specified, the grid will have exactly this many rows, and the number
    /// of columns will be calculated automatically based on the number of facets. If not
    /// specified, both dimensions are calculated automatically.
    ///
    /// # Argument
    ///
    /// * `nrow` - A `usize` value specifying the number of rows (must be greater than 0).
    ///
    /// # Panics
    ///
    /// Panics if `nrow` is 0.
    pub fn nrow(mut self, nrow: usize) -> Self {
        if nrow == 0 {
            panic!("nrow must be greater than 0");
        }
        self.nrow = Some(nrow);
        self
    }

    /// Sets the axis scale behavior across facets.
    ///
    /// Controls whether facets share the same axis ranges (`Fixed`) or have independent
    /// scales (`Free`, `FreeX`, or `FreeY`). Fixed scales make it easier to compare values
    /// across facets, while free scales allow each facet to use its optimal range.
    ///
    /// # Argument
    ///
    /// * `scales` - A `FacetScales` enum value specifying the scale behavior.
    pub fn scales(mut self, scales: FacetScales) -> Self {
        self.scales = scales;
        self
    }

    /// Sets the horizontal spacing between facets.
    ///
    /// The gap is specified as a proportion of the plot width, with typical values
    /// ranging from 0.0 (no gap) to 0.2 (20% gap). If not specified, plotly's default
    /// spacing is used.
    ///
    /// # Argument
    ///
    /// * `gap` - A `f64` value from 0.0 to 1.0 representing the relative gap size.
    ///
    /// # Panics
    ///
    /// Panics if `gap` is negative, NaN, or infinite.
    pub fn x_gap(mut self, gap: f64) -> Self {
        if !gap.is_finite() || gap < 0.0 {
            panic!("x_gap must be a finite non-negative number");
        }
        self.x_gap = Some(gap);
        self
    }

    /// Sets the vertical spacing between facets.
    ///
    /// The gap is specified as a proportion of the plot height, with typical values
    /// ranging from 0.0 (no gap) to 0.2 (20% gap). If not specified, plotly's default
    /// spacing is used.
    ///
    /// # Argument
    ///
    /// * `gap` - A `f64` value from 0.0 to 1.0 representing the relative gap size.
    ///
    /// # Panics
    ///
    /// Panics if `gap` is negative, NaN, or infinite.
    pub fn y_gap(mut self, gap: f64) -> Self {
        if !gap.is_finite() || gap < 0.0 {
            panic!("y_gap must be a finite non-negative number");
        }
        self.y_gap = Some(gap);
        self
    }

    /// Sets the styling for facet labels.
    ///
    /// Controls the font, size, and color of the category labels that appear above each
    /// facet. If not specified, plotly's default text styling is used.
    ///
    /// # Argument
    ///
    /// * `style` - A `Text` component or any type that can be converted into `Text`,
    ///   specifying the styling options for facet titles.
    pub fn title_style<T: Into<Text>>(mut self, style: T) -> Self {
        self.title_style = Some(style.into());
        self
    }

    /// Sets a custom sorting function for facet order.
    ///
    /// By default, facets are ordered alphabetically by category name. This method allows
    /// you to specify a custom comparison function to control the order in which facets
    /// appear in the grid.
    ///
    /// # Argument
    ///
    /// * `f` - A function that takes two string slices and returns an `Ordering`,
    ///   following the same signature as `str::cmp`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use plotlars::FacetConfig;
    /// use std::cmp::Ordering;
    ///
    /// // Reverse alphabetical order
    /// let config = FacetConfig::new()
    ///     .sorter(|a, b| b.cmp(a));
    /// ```
    pub fn sorter(mut self, f: fn(&str, &str) -> Ordering) -> Self {
        self.sorter = Some(f);
        self
    }

    /// Enables or disables facet highlighting mode.
    ///
    /// When enabled, each facet shows all data from all categories, but emphasizes
    /// the data for the current facet's category while displaying other categories
    /// in a muted color. This provides visual context by showing the full data
    /// distribution while focusing attention on the current facet.
    ///
    /// # Argument
    ///
    /// * `highlight` - A boolean value: `true` to enable highlighting, `false` to disable.
    pub fn highlight_facet(mut self, highlight: bool) -> Self {
        self.highlight_facet = highlight;
        self
    }

    /// Sets the color for unhighlighted data points in highlighting mode.
    ///
    /// This setting only takes effect when `highlight_facet` is enabled. It specifies
    /// the color used for data points that belong to other categories (not the current
    /// facet's category). If not specified, a default grey color is used.
    ///
    /// # Argument
    ///
    /// * `color` - An `Rgb` value specifying the color for unhighlighted data.
    pub fn unhighlighted_color(mut self, color: Rgb) -> Self {
        self.unhighlighted_color = Some(color);
        self
    }
}
