/// An enumeration of various marker shapes used in plots.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Axis, Legend, Plot, Rgb, ScatterPlot, Shape, Text, TickDirection};
///
/// let dataset = LazyCsvReader::new(PlRefPath::new("data/penguins.csv"))
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
/// let axis = Axis::new()
///     .show_line(true)
///     .tick_direction(TickDirection::OutSide)
///     .value_thousands(true);
///
/// ScatterPlot::builder()
///     .data(&dataset)
///     .x("body_mass_g")
///     .y("flipper_length_mm")
///     .group("species")
///     .opacity(0.5)
///     .size(12)
///     .colors(vec![
///         Rgb(178, 34, 34),
///         Rgb(65, 105, 225),
///         Rgb(255, 140, 0),
///     ])
///     .shapes(vec![
///         Shape::Circle,
///         Shape::Square,
///         Shape::Diamond,
///     ])
///     .plot_title(
///         Text::from("Scatter Plot")
///             .font("Arial")
///             .size(20)
///             .x(0.065)
///     )
///     .x_title("body mass (g)")
///     .y_title("flipper length (mm)")
///     .legend_title("species")
///     .x_axis(
///         &axis.clone()
///             .value_range(2500.0, 6500.0)
///     )
///     .y_axis(
///         &axis.clone()
///             .value_range(170.0, 240.0)
///     )
///     .legend(
///         &Legend::new()
///             .x(0.85)
///             .y(0.15)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/9jfO8RU.png)
#[derive(Clone, Copy)]
pub enum Shape {
    Circle,
    CircleOpen,
    CircleDot,
    CircleOpenDot,
    Square,
    SquareOpen,
    SquareDot,
    SquareOpenDot,
    Diamond,
    DiamondOpen,
    DiamondDot,
    DiamondOpenDot,
    Cross,
    CrossOpen,
    CrossDot,
    CrossOpenDot,
    X,
    XOpen,
    XDot,
    XOpenDot,
    TriangleUp,
    TriangleUpOpen,
    TriangleUpDot,
    TriangleUpOpenDot,
    TriangleDown,
    TriangleDownOpen,
    TriangleDownDot,
    TriangleDownOpenDot,
    TriangleLeft,
    TriangleLeftOpen,
    TriangleLeftDot,
    TriangleLeftOpenDot,
    TriangleRight,
    TriangleRightOpen,
    TriangleRightDot,
    TriangleRightOpenDot,
    TriangleNE,
    TriangleNEOpen,
    TriangleNEDot,
    TriangleNEOpenDot,
    TriangleSE,
    TriangleSEOpen,
    TriangleSEDot,
    TriangleSEOpenDot,
    TriangleSW,
    TriangleSWOpen,
    TriangleSWDot,
    TriangleSWOpenDot,
    TriangleNW,
    TriangleNWOpen,
    TriangleNWDot,
    TriangleNWOpenDot,
    Pentagon,
    PentagonOpen,
    PentagonDot,
    PentagonOpenDot,
    Hexagon,
    HexagonOpen,
    HexagonDot,
    HexagonOpenDot,
    Hexagon2,
    Hexagon2Open,
    Hexagon2Dot,
    Hexagon2OpenDot,
    Octagon,
    OctagonOpen,
    OctagonDot,
    OctagonOpenDot,
    Star,
    StarOpen,
    StarDot,
    StarOpenDot,
    Hexagram,
    HexagramOpen,
    HexagramDot,
    HexagramOpenDot,
    StarTriangleUp,
    StarTriangleUpOpen,
    StarTriangleUpDot,
    StarTriangleUpOpenDot,
    StarTriangleDown,
    StarTriangleDownOpen,
    StarTriangleDownDot,
    StarTriangleDownOpenDot,
    StarSquare,
    StarSquareOpen,
    StarSquareDot,
    StarSquareOpenDot,
    StarDiamond,
    StarDiamondOpen,
    StarDiamondDot,
    StarDiamondOpenDot,
    DiamondTall,
    DiamondTallOpen,
    DiamondTallDot,
    DiamondTallOpenDot,
    DiamondWide,
    DiamondWideOpen,
    DiamondWideDot,
    DiamondWideOpenDot,
    Hourglass,
    HourglassOpen,
    BowTie,
    BowTieOpen,
    CircleCross,
    CircleCrossOpen,
    CircleX,
    CircleXOpen,
    SquareCross,
    SquareCrossOpen,
    SquareX,
    SquareXOpen,
    DiamondCross,
    DiamondCrossOpen,
    DiamondX,
    DiamondXOpen,
    CrossThin,
    CrossThinOpen,
    XThin,
    XThinOpen,
    Asterisk,
    AsteriskOpen,
    Hash,
    HashOpen,
    HashDot,
    HashOpenDot,
    YUp,
    YUpOpen,
    YDown,
    YDownOpen,
    YLeft,
    YLeftOpen,
    YRight,
    YRightOpen,
    LineEW,
    LineEWOpen,
    LineNS,
    LineNSOpen,
    LineNE,
    LineNEOpen,
    LineNW,
    LineNWOpen,
}
