/// A structure representing plot dimensions and sizing behavior.
///
/// The `Dimensions` struct allows customization of plot size including width, height,
/// and auto-sizing behavior. It is particularly useful when creating subplot grids or
/// when you need precise control over plot dimensions.
///
/// # Example
///
/// ```rust
/// use plotlars::{
///     Axis, BarPlot, BoxPlot, Dimensions, Legend, Line, Orientation, Plot, Rgb, ScatterPlot, Shape,
///     SubplotGrid, Text, TickDirection, TimeSeriesPlot,
/// };
/// use polars::prelude::*;
///
/// let penguins_dataset = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
///     .finish()
///     .unwrap()
///     .select([
///         col("species"),
///         col("sex").alias("gender"),
///         col("flipper_length_mm").cast(DataType::Int16),
///         col("body_mass_g").cast(DataType::Int16),
///     ])
///     .collect()
///     .unwrap();
///
/// let temperature_dataset = LazyCsvReader::new(PlPath::new("data/debilt_2023_temps.csv"))
///     .with_has_header(true)
///     .with_try_parse_dates(true)
///     .finish()
///     .unwrap()
///     .with_columns(vec![
///         (col("tavg") / lit(10)).alias("tavg"),
///         (col("tmin") / lit(10)).alias("tmin"),
///         (col("tmax") / lit(10)).alias("tmax"),
///     ])
///     .collect()
///     .unwrap();
///
/// let animals_dataset = LazyCsvReader::new(PlPath::new("data/animal_statistics.csv"))
///     .finish()
///     .unwrap()
///     .collect()
///     .unwrap();
///
/// let axis = Axis::new()
///     .show_line(true)
///     .tick_direction(TickDirection::OutSide)
///     .value_thousands(true);
///
/// let plot1 = TimeSeriesPlot::builder()
///     .data(&temperature_dataset)
///     .x("date")
///     .y("tavg")
///     .additional_series(vec!["tmin", "tmax"])
///     .colors(vec![Rgb(128, 128, 128), Rgb(0, 122, 255), Rgb(255, 128, 0)])
///     .lines(vec![Line::Solid, Line::Dot, Line::Dot])
///     .plot_title(
///         Text::from("De Bilt Temperature 2023")
///             .font("Arial Bold")
///             .size(16),
///     )
///     .y_title(Text::from("temperature (°C)").size(13).x(-0.08))
///     .legend(&Legend::new().x(0.1).y(0.9))
///     .build();
///
/// let plot2 = ScatterPlot::builder()
///     .data(&penguins_dataset)
///     .x("body_mass_g")
///     .y("flipper_length_mm")
///     .group("species")
///     .sort_groups_by(|a, b| {
///         if a.len() == b.len() {
///             a.cmp(b)
///         } else {
///             a.len().cmp(&b.len())
///         }
///     })
///     .opacity(0.6)
///     .size(10)
///     .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
///     .shapes(vec![Shape::Circle, Shape::Square, Shape::Diamond])
///     .plot_title(Text::from("Penguin Morphology").font("Arial Bold").size(16))
///     .x_title(Text::from("body mass (g)").size(13))
///     .y_title(Text::from("flipper length (mm)").size(13).x(-0.11))
///     .legend_title(Text::from("Species").size(12))
///     .x_axis(&axis.clone().value_range(vec![2500.0, 6500.0]))
///     .y_axis(&axis.clone().value_range(vec![170.0, 240.0]))
///     .legend(&Legend::new().x(0.85).y(0.4))
///     .build();
///
/// let plot3 = BarPlot::builder()
///     .data(&animals_dataset)
///     .labels("animal")
///     .values("value")
///     .orientation(Orientation::Vertical)
///     .group("gender")
///     .sort_groups_by(|a, b| a.len().cmp(&b.len()))
///     .error("error")
///     .colors(vec![Rgb(255, 127, 80), Rgb(64, 224, 208)])
///     .plot_title(Text::from("Animal Statistics").font("Arial Bold").size(16))
///     .x_title(Text::from("animal").size(13))
///     .y_title(Text::from("value").size(13))
///     .legend_title(Text::from("Gender").size(12))
///     .legend(
///         &Legend::new()
///             .orientation(Orientation::Horizontal)
///             .x(0.35)
///             .y(0.9),
///     )
///     .build();
///
/// let plot4 = BoxPlot::builder()
///     .data(&penguins_dataset)
///     .labels("species")
///     .values("body_mass_g")
///     .orientation(Orientation::Vertical)
///     .group("gender")
///     .box_points(true)
///     .point_offset(-1.5)
///     .jitter(0.01)
///     .opacity(0.15)
///     .colors(vec![Rgb(0, 191, 255), Rgb(57, 255, 20), Rgb(255, 105, 180)])
///     .plot_title(
///         Text::from("Body Mass Distribution")
///             .font("Arial Bold")
///             .size(16),
///     )
///     .x_title(Text::from("species").size(13))
///     .y_title(Text::from("body mass (g)").size(13).x(-0.12))
///     .legend_title(Text::from("Gender").size(12))
///     .y_axis(&Axis::new().value_thousands(true))
///     .legend(&Legend::new().x(0.85).y(0.9))
///     .build();
///
/// let dimensions = Dimensions::new().width(1400).height(850).auto_size(false);
///
/// SubplotGrid::regular()
///     .plots(vec![&plot1, &plot2, &plot3, &plot4])
///     .rows(2)
///     .cols(2)
///     .v_gap(0.3)
///     .h_gap(0.2)
///     .dimensions(&dimensions)
///     .title(
///         Text::from("Scientific Data Visualization Dashboard")
///             .size(26)
///             .font("Arial Bold"),
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/hxwInxB.png)
#[derive(Clone, Default)]
pub struct Dimensions {
    pub(crate) width: Option<usize>,
    pub(crate) height: Option<usize>,
    pub(crate) auto_size: Option<bool>,
}

impl Dimensions {
    /// Creates a new `Dimensions` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the width of the plot in pixels.
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the height of the plot in pixels.
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets whether the plot should automatically resize.
    pub fn auto_size(mut self, auto_size: bool) -> Self {
        self.auto_size = Some(auto_size);
        self
    }
}
