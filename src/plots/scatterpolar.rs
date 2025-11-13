use bon::bon;

use plotly::{
    common::{Line as LinePlotly, Marker as MarkerPlotly},
    layout::Margin,
    Layout as LayoutPlotly, ScatterPolar as ScatterPolarPlotly, Trace,
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, Marker, PlotHelper, Polar},
    components::{
        FacetConfig, Fill, Legend, Line as LineStyle, Mode, Rgb, Shape, Text, DEFAULT_PLOTLY_COLORS,
    },
};

/// A structure representing a scatter polar plot.
///
/// The `ScatterPolar` struct facilitates the creation and customization of polar scatter plots with various options
/// for data selection, grouping, layout configuration, and aesthetic adjustments. It supports grouping of data,
/// customization of marker shapes, colors, sizes, line styles, and comprehensive layout customization
/// including titles and legends.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `theta` - A string slice specifying the column name to be used for the angular coordinates (in degrees).
/// * `r` - A string slice specifying the column name to be used for the radial coordinates.
/// * `group` - An optional string slice specifying the column name to be used for grouping data points.
/// * `sort_groups_by` - Optional comparator `fn(&str, &str) -> std::cmp::Ordering` to control group ordering. Groups are sorted lexically by default.
/// * `facet` - An optional string slice specifying the column name to be used for faceting (creating multiple subplots).
/// * `facet_config` - An optional reference to a `FacetConfig` struct for customizing facet behavior (grid dimensions, scales, gaps, etc.).
/// * `mode` - An optional `Mode` specifying the drawing mode (lines, markers, or both). Defaults to markers.
/// * `opacity` - An optional `f64` value specifying the opacity of the plot elements (range: 0.0 to 1.0).
/// * `fill` - An optional `Fill` type specifying how to fill the area under the trace.
/// * `size` - An optional `usize` specifying the size of the markers.
/// * `color` - An optional `Rgb` value specifying the color of the markers. This is used when `group` is not specified.
/// * `colors` - An optional vector of `Rgb` values specifying the colors for the markers. This is used when `group` is specified to differentiate between groups.
/// * `shape` - An optional `Shape` specifying the shape of the markers. This is used when `group` is not specified.
/// * `shapes` - An optional vector of `Shape` values specifying multiple shapes for the markers when plotting multiple groups.
/// * `width` - An optional `f64` specifying the width of the lines.
/// * `line` - An optional `LineStyle` specifying the style of the line (e.g., solid, dashed).
/// * `lines` - An optional vector of `LineStyle` enums specifying the styles of lines for multiple traces.
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
/// * `legend_title` - An optional `Text` struct specifying the title of the legend.
/// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot (e.g., positioning, font, etc.).
///
/// # Example
///
/// ```rust
/// use plotlars::{Legend, Line, Mode, Plot, Rgb, ScatterPolar, Shape, Text};
/// use polars::prelude::*;
///
/// let dataset = LazyCsvReader::new(PlPath::new("data/product_comparison_polar.csv"))
///     .finish()
///     .unwrap()
///     .collect()
///     .unwrap();
///
/// ScatterPolar::builder()
///     .data(&dataset)
///     .theta("angle")
///     .r("score")
///     .group("product")
///     .mode(Mode::LinesMarkers)
///     .colors(vec![
///         Rgb(255, 99, 71),
///         Rgb(60, 179, 113),
///     ])
///     .shapes(vec![
///         Shape::Circle,
///         Shape::Square,
///     ])
///     .lines(vec![
///         Line::Solid,
///         Line::Dash,
///     ])
///     .width(2.5)
///     .size(8)
///     .plot_title(
///         Text::from("Scatter Polar Plot")
///             .font("Arial")
///             .size(24)
///     )
///     .legend_title(
///         Text::from("Products")
///             .font("Arial")
///             .size(14)
///     )
///     .legend(
///         &Legend::new()
///             .x(0.85)
///             .y(0.95)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/kl1pY9c.png)
#[derive(Clone)]
pub struct ScatterPolar {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
    layout_json: Option<serde_json::Value>,
}

impl Serialize for ScatterPolar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("ScatterPolar", 2)?;
        state.serialize_field("traces", &self.traces)?;
        // Use modified layout JSON if available, otherwise serialize the layout
        if let Some(ref layout_json) = self.layout_json {
            state.serialize_field("layout", layout_json)?;
        } else {
            state.serialize_field("layout", &self.layout)?;
        }
        state.end()
    }
}

#[derive(Clone)]
struct FacetGrid {
    ncols: usize,
    nrows: usize,
    x_gap: f64,
    y_gap: f64,
}

const POLAR_FACET_TITLE_HEIGHT_RATIO: f64 = 0.12;
const POLAR_FACET_TOP_INSET_RATIO: f64 = 0.10;

#[bon]
impl ScatterPolar {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        theta: &str,
        r: &str,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
        mode: Option<Mode>,
        opacity: Option<f64>,
        fill: Option<Fill>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        line: Option<LineStyle>,
        lines: Option<Vec<LineStyle>>,
        plot_title: Option<Text>,
        legend_title: Option<Text>,
        legend: Option<&Legend>,
    ) -> Self {
        let x_title = None;
        let y_title = None;
        let y2_title = None;
        let z_title = None;
        let x_axis = None;
        let y_axis = None;
        let y2_axis = None;
        let z_axis = None;

        let (layout, traces, layout_json) = match facet {
            Some(facet_column) => {
                let config = facet_config.cloned().unwrap_or_default();

                let (layout, grid) = Self::create_faceted_layout(
                    data,
                    facet_column,
                    &config,
                    plot_title,
                    legend_title,
                    legend,
                );

                let traces = Self::create_faceted_traces(
                    data,
                    theta,
                    r,
                    group,
                    sort_groups_by,
                    facet_column,
                    &config,
                    mode,
                    opacity,
                    fill,
                    size,
                    color,
                    colors,
                    shape,
                    shapes,
                    width,
                    line,
                    lines,
                );

                // Inject polar subplot domains into layout JSON
                let mut layout_json = serde_json::to_value(&layout).unwrap();
                Self::inject_polar_domains_static(
                    &mut layout_json,
                    grid.ncols,
                    grid.nrows,
                    grid.x_gap,
                    grid.y_gap,
                );

                (layout, traces, Some(layout_json))
            }
            None => {
                let layout = Self::create_layout(
                    plot_title,
                    x_title,
                    y_title,
                    y2_title,
                    z_title,
                    legend_title,
                    x_axis,
                    y_axis,
                    y2_axis,
                    z_axis,
                    legend,
                );

                let traces = Self::create_traces(
                    data,
                    theta,
                    r,
                    group,
                    sort_groups_by,
                    mode,
                    opacity,
                    fill,
                    size,
                    color,
                    colors,
                    shape,
                    shapes,
                    width,
                    line,
                    lines,
                );

                (layout, traces, None)
            }
        };

        Self {
            traces,
            layout,
            layout_json,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
        data: &DataFrame,
        theta: &str,
        r: &str,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        mode: Option<Mode>,
        opacity: Option<f64>,
        fill: Option<Fill>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        line: Option<LineStyle>,
        lines: Option<Vec<LineStyle>>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();
        let mode = mode
            .map(|m| m.to_plotly())
            .unwrap_or(plotly::common::Mode::Markers);

        match group {
            Some(group_col) => {
                let groups = Self::get_unique_groups(data, group_col, sort_groups_by);
                let groups = groups.iter().map(|s| s.as_str());

                for (i, group) in groups.enumerate() {
                    let marker = Self::create_marker(
                        i,
                        opacity,
                        size,
                        color,
                        colors.clone(),
                        shape,
                        shapes.clone(),
                    );

                    let line_style = Self::create_line_with_color(
                        i,
                        width,
                        color,
                        colors.clone(),
                        line,
                        lines.clone(),
                    );

                    let subset = Self::filter_data_by_group(data, group_col, group);

                    let trace = Self::create_trace(
                        &subset,
                        theta,
                        r,
                        Some(group),
                        mode.clone(),
                        marker,
                        line_style,
                        fill,
                    );

                    traces.push(trace);
                }
            }
            None => {
                let group = None;

                let marker = Self::create_marker(
                    0,
                    opacity,
                    size,
                    color,
                    colors.clone(),
                    shape,
                    shapes.clone(),
                );

                let line_style = Self::create_line_with_color(
                    0,
                    width,
                    color,
                    colors.clone(),
                    line,
                    lines.clone(),
                );

                let trace =
                    Self::create_trace(data, theta, r, group, mode, marker, line_style, fill);

                traces.push(trace);
            }
        }

        traces
    }

    #[allow(clippy::too_many_arguments)]
    fn create_trace(
        data: &DataFrame,
        theta: &str,
        r: &str,
        group_name: Option<&str>,
        mode: plotly::common::Mode,
        marker: MarkerPlotly,
        line: LinePlotly,
        fill: Option<Fill>,
    ) -> Box<dyn Trace + 'static> {
        let theta_values = Self::get_numeric_column(data, theta);
        let r_values = Self::get_numeric_column(data, r);

        let mut trace = ScatterPolarPlotly::default()
            .theta(theta_values)
            .r(r_values)
            .mode(mode);

        trace = trace.marker(marker);
        trace = trace.line(line);

        if let Some(fill_type) = fill {
            trace = trace.fill(fill_type.to_plotly());
        }

        if let Some(name) = group_name {
            trace = trace.name(name);
        }

        trace
    }

    fn create_line_with_color(
        index: usize,
        width: Option<f64>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        style: Option<LineStyle>,
        styles: Option<Vec<LineStyle>>,
    ) -> LinePlotly {
        let mut line = LinePlotly::new();

        // Set width
        if let Some(width) = width {
            line = line.width(width);
        }

        // Set style
        if let Some(style) = style {
            line = line.dash(style.to_plotly());
        } else if let Some(styles) = styles {
            if let Some(style) = styles.get(index) {
                line = line.dash(style.to_plotly());
            }
        }

        // Set color
        if let Some(color) = color {
            line = line.color(color.to_plotly());
        } else if let Some(colors) = colors {
            if let Some(color) = colors.get(index) {
                line = line.color(color.to_plotly());
            }
        }

        line
    }

    fn get_polar_subplot_reference(index: usize) -> String {
        match index {
            0 => "polar".to_string(),
            1 => "polar2".to_string(),
            2 => "polar3".to_string(),
            3 => "polar4".to_string(),
            4 => "polar5".to_string(),
            5 => "polar6".to_string(),
            6 => "polar7".to_string(),
            7 => "polar8".to_string(),
            _ => "polar".to_string(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn build_scatter_polar_trace_with_subplot(
        data: &DataFrame,
        theta: &str,
        r: &str,
        group_name: Option<&str>,
        mode: plotly::common::Mode,
        marker: MarkerPlotly,
        line: LinePlotly,
        fill: Option<Fill>,
        subplot: Option<&str>,
        show_legend: bool,
        legend_group: Option<&str>,
    ) -> Box<dyn Trace + 'static> {
        let theta_values = Self::get_numeric_column(data, theta);
        let r_values = Self::get_numeric_column(data, r);

        let mut trace = ScatterPolarPlotly::default()
            .theta(theta_values)
            .r(r_values)
            .mode(mode);

        trace = trace.marker(marker);
        trace = trace.line(line);

        if let Some(fill_type) = fill {
            trace = trace.fill(fill_type.to_plotly());
        }

        if let Some(name) = group_name {
            trace = trace.name(name);
        }

        if let Some(subplot_ref) = subplot {
            trace = trace.subplot(subplot_ref);
        }

        let trace = if let Some(group) = legend_group {
            trace.legend_group(group)
        } else {
            trace
        };

        if !show_legend {
            trace.show_legend(false)
        } else {
            trace
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_faceted_traces(
        data: &DataFrame,
        theta: &str,
        r: &str,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet_column: &str,
        config: &FacetConfig,
        mode: Option<Mode>,
        opacity: Option<f64>,
        fill: Option<Fill>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        line: Option<LineStyle>,
        lines: Option<Vec<LineStyle>>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        const MAX_FACETS: usize = 8;

        let facet_categories = Self::get_unique_groups(data, facet_column, config.sorter);

        if facet_categories.len() > MAX_FACETS {
            panic!(
                "Facet column '{}' has {} unique values, but plotly.rs supports maximum {} polar subplots",
                facet_column,
                facet_categories.len(),
                MAX_FACETS
            );
        }

        if let Some(ref color_vec) = colors {
            if group.is_none() {
                let color_count = color_vec.len();
                let facet_count = facet_categories.len();

                if color_count != facet_count {
                    panic!(
                        "When using colors with facet (without group), colors.len() must equal number of facets. \
                         Expected {} colors for {} facets, but got {} colors. \
                         Each facet must be assigned exactly one color.",
                        facet_count, facet_count, color_count
                    );
                }
            } else if let Some(group_col) = group {
                let groups = Self::get_unique_groups(data, group_col, sort_groups_by);
                let color_count = color_vec.len();
                let group_count = groups.len();

                if color_count < group_count {
                    panic!(
                        "When using colors with group, colors.len() must be >= number of groups. \
                         Need at least {} colors for {} groups, but got {} colors",
                        group_count, group_count, color_count
                    );
                }
            }
        }

        let global_group_indices: std::collections::HashMap<String, usize> =
            if let Some(group_col) = group {
                let global_groups = Self::get_unique_groups(data, group_col, sort_groups_by);
                global_groups
                    .into_iter()
                    .enumerate()
                    .map(|(idx, group_name)| (group_name, idx))
                    .collect()
            } else {
                std::collections::HashMap::new()
            };

        let colors = if group.is_some() && colors.is_none() {
            Some(DEFAULT_PLOTLY_COLORS.to_vec())
        } else {
            colors
        };

        let mode = mode
            .map(|m| m.to_plotly())
            .unwrap_or(plotly::common::Mode::Markers);

        let mut all_traces = Vec::new();

        if config.highlight_facet {
            for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
                let subplot = Self::get_polar_subplot_reference(facet_idx);

                for other_facet_value in facet_categories.iter() {
                    if other_facet_value != facet_value {
                        let other_data =
                            Self::filter_data_by_group(data, facet_column, other_facet_value);

                        let grey_color = config.unhighlighted_color.unwrap_or(Rgb(200, 200, 200));
                        let grey_marker = Self::create_marker(
                            0,
                            opacity,
                            size,
                            Some(grey_color),
                            None,
                            shape,
                            None,
                        );

                        let grey_line = Self::create_line_with_color(
                            0,
                            width,
                            Some(grey_color),
                            None,
                            line,
                            None,
                        );

                        let trace = Self::build_scatter_polar_trace_with_subplot(
                            &other_data,
                            theta,
                            r,
                            None,
                            mode.clone(),
                            grey_marker,
                            grey_line,
                            fill,
                            Some(&subplot),
                            false,
                            None,
                        );

                        all_traces.push(trace);
                    }
                }

                let facet_data = Self::filter_data_by_group(data, facet_column, facet_value);

                match group {
                    Some(group_col) => {
                        let groups =
                            Self::get_unique_groups(&facet_data, group_col, sort_groups_by);

                        for group_val in groups.iter() {
                            let group_data =
                                Self::filter_data_by_group(&facet_data, group_col, group_val);

                            let global_idx =
                                global_group_indices.get(group_val).copied().unwrap_or(0);

                            let marker = Self::create_marker(
                                global_idx,
                                opacity,
                                size,
                                color,
                                colors.clone(),
                                shape,
                                shapes.clone(),
                            );

                            let line_style = Self::create_line_with_color(
                                global_idx,
                                width,
                                color,
                                colors.clone(),
                                line,
                                lines.clone(),
                            );

                            let show_legend = facet_idx == 0;

                            let trace = Self::build_scatter_polar_trace_with_subplot(
                                &group_data,
                                theta,
                                r,
                                Some(group_val),
                                mode.clone(),
                                marker,
                                line_style,
                                fill,
                                Some(&subplot),
                                show_legend,
                                Some(group_val),
                            );

                            all_traces.push(trace);
                        }
                    }
                    None => {
                        let marker = Self::create_marker(
                            facet_idx,
                            opacity,
                            size,
                            color,
                            colors.clone(),
                            shape,
                            shapes.clone(),
                        );

                        let line_style = Self::create_line_with_color(
                            facet_idx,
                            width,
                            color,
                            colors.clone(),
                            line,
                            lines.clone(),
                        );

                        let trace = Self::build_scatter_polar_trace_with_subplot(
                            &facet_data,
                            theta,
                            r,
                            None,
                            mode.clone(),
                            marker,
                            line_style,
                            fill,
                            Some(&subplot),
                            false,
                            None,
                        );

                        all_traces.push(trace);
                    }
                }
            }
        } else {
            for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
                let facet_data = Self::filter_data_by_group(data, facet_column, facet_value);

                let subplot = Self::get_polar_subplot_reference(facet_idx);

                match group {
                    Some(group_col) => {
                        let groups =
                            Self::get_unique_groups(&facet_data, group_col, sort_groups_by);

                        for group_val in groups.iter() {
                            let group_data =
                                Self::filter_data_by_group(&facet_data, group_col, group_val);

                            let global_idx =
                                global_group_indices.get(group_val).copied().unwrap_or(0);

                            let marker = Self::create_marker(
                                global_idx,
                                opacity,
                                size,
                                color,
                                colors.clone(),
                                shape,
                                shapes.clone(),
                            );

                            let line_style = Self::create_line_with_color(
                                global_idx,
                                width,
                                color,
                                colors.clone(),
                                line,
                                lines.clone(),
                            );

                            let show_legend = facet_idx == 0;

                            let trace = Self::build_scatter_polar_trace_with_subplot(
                                &group_data,
                                theta,
                                r,
                                Some(group_val),
                                mode.clone(),
                                marker,
                                line_style,
                                fill,
                                Some(&subplot),
                                show_legend,
                                Some(group_val),
                            );

                            all_traces.push(trace);
                        }
                    }
                    None => {
                        let marker = Self::create_marker(
                            facet_idx,
                            opacity,
                            size,
                            color,
                            colors.clone(),
                            shape,
                            shapes.clone(),
                        );

                        let line_style = Self::create_line_with_color(
                            facet_idx,
                            width,
                            color,
                            colors.clone(),
                            line,
                            lines.clone(),
                        );

                        let trace = Self::build_scatter_polar_trace_with_subplot(
                            &facet_data,
                            theta,
                            r,
                            None,
                            mode.clone(),
                            marker,
                            line_style,
                            fill,
                            Some(&subplot),
                            false,
                            None,
                        );

                        all_traces.push(trace);
                    }
                }
            }
        }

        all_traces
    }

    fn create_faceted_layout(
        data: &DataFrame,
        facet_column: &str,
        config: &FacetConfig,
        plot_title: Option<Text>,
        legend_title: Option<Text>,
        legend: Option<&Legend>,
    ) -> (LayoutPlotly, FacetGrid) {
        let facet_categories = Self::get_unique_groups(data, facet_column, config.sorter);
        let n_facets = facet_categories.len();

        let (ncols, nrows) = Self::calculate_grid_dimensions(n_facets, config.ncol, config.nrow);

        // Store grid dimensions for polar domain injection later
        let x_gap = config.x_gap.unwrap_or(0.08);
        let y_gap = config.y_gap.unwrap_or(0.12);

        let grid = FacetGrid {
            ncols,
            nrows,
            x_gap,
            y_gap,
        };

        // Note: We'll inject polar subplot domain configurations manually via Plot trait
        // since plotly.rs doesn't support LayoutPolar
        let mut layout = LayoutPlotly::new();

        if let Some(title) = plot_title {
            layout = layout.title(title.to_plotly());
        }

        let annotations = Self::create_facet_annotations_polar(
            &facet_categories,
            ncols,
            nrows,
            config.title_style.as_ref(),
            config.x_gap,
            config.y_gap,
        );
        layout = layout.annotations(annotations);

        layout = layout.legend(Legend::set_legend(legend_title, legend));

        // Add margins to provide adequate space for polar subplots
        // Top margin accounts for plot title and facet labels
        // Side margins prevent clipping of circular polar plots
        layout = layout.margin(Margin::new().top(140).bottom(80).left(80).right(80));

        (layout, grid)
    }

    /// Calculates the geometry for a polar facet cell, including subplot domain bounds and title baseline.
    ///
    /// Returning both the domain and annotation placement keeps titles aligned with their subplot
    /// while guaranteeing consistent padding above the polar chart.
    fn calculate_polar_facet_cell(
        subplot_index: usize,
        ncols: usize,
        nrows: usize,
        x_gap: Option<f64>,
        y_gap: Option<f64>,
    ) -> PolarFacetCell {
        let row = subplot_index / ncols;
        let col = subplot_index % ncols;

        let x_gap_val = x_gap.unwrap_or(0.08);
        let y_gap_val = y_gap.unwrap_or(0.12);

        let cell_width = (1.0 - x_gap_val * (ncols - 1) as f64) / ncols as f64;
        let cell_height = (1.0 - y_gap_val * (nrows - 1) as f64) / nrows as f64;

        let title_height = cell_height * POLAR_FACET_TITLE_HEIGHT_RATIO;
        let polar_padding = cell_height * POLAR_FACET_TOP_INSET_RATIO;

        let cell_x_start = col as f64 * (cell_width + x_gap_val);
        let cell_y_top = 1.0 - row as f64 * (cell_height + y_gap_val);
        let cell_y_bottom = cell_y_top - cell_height;

        let domain_y_top = cell_y_top - title_height - polar_padding;
        let domain_y_bottom = cell_y_bottom;

        let domain_x = [cell_x_start, cell_x_start + cell_width];
        let domain_y = [domain_y_bottom, domain_y_top];

        let annotation_x = cell_x_start + cell_width / 2.0;
        let annotation_y = cell_y_top - title_height / 2.0;

        PolarFacetCell {
            annotation_x,
            annotation_y,
            domain_x,
            domain_y,
        }
    }

    fn create_facet_annotations_polar(
        categories: &[String],
        ncols: usize,
        nrows: usize,
        title_style: Option<&Text>,
        x_gap: Option<f64>,
        y_gap: Option<f64>,
    ) -> Vec<plotly::layout::Annotation> {
        use plotly::common::Anchor;
        use plotly::layout::Annotation;

        categories
            .iter()
            .enumerate()
            .map(|(i, cat)| {
                let cell = Self::calculate_polar_facet_cell(i, ncols, nrows, x_gap, y_gap);

                let mut ann = Annotation::new()
                    .text(cat.as_str())
                    .x_ref("paper")
                    .y_ref("paper")
                    .x_anchor(Anchor::Center)
                    .y_anchor(Anchor::Bottom)
                    .x(cell.annotation_x)
                    .y(cell.annotation_y)
                    .show_arrow(false);

                if let Some(style) = title_style {
                    ann = ann.font(style.to_font());
                }

                ann
            })
            .collect()
    }
}

/// Helper struct containing calculated annotation positions for a polar facet cell
struct PolarFacetCell {
    annotation_x: f64,
    annotation_y: f64,
    domain_x: [f64; 2],
    domain_y: [f64; 2],
}

impl ScatterPolar {
    /// Injects polar subplot domain configurations into the layout JSON
    /// This is a workaround for plotly.rs not supporting LayoutPolar configuration
    fn inject_polar_domains_static(
        layout_json: &mut serde_json::Value,
        ncols: usize,
        nrows: usize,
        x_gap: f64,
        y_gap: f64,
    ) {
        // Configure all 8 possible polar subplots (polar, polar2, ..., polar8)
        // Traces reference these via their subplot parameter

        let total_cells = (ncols * nrows).clamp(1, 8);

        for i in 0..total_cells {
            let polar_key = if i == 0 {
                "polar".to_string()
            } else {
                format!("polar{}", i + 1)
            };

            let cell = Self::calculate_polar_facet_cell(i, ncols, nrows, Some(x_gap), Some(y_gap));

            let compression_factor = 0.9;
            let domain_height = cell.domain_y[1] - cell.domain_y[0];
            let height_reduction = domain_height * (1.0 - compression_factor);
            let compressed_domain_y = [
                cell.domain_y[0] + height_reduction / 2.0,
                cell.domain_y[1] - height_reduction / 2.0,
            ];

            let polar_config = serde_json::json!({
                "domain": {
                    "x": cell.domain_x,
                    "y": compressed_domain_y
                }
            });

            layout_json[polar_key] = polar_config;
        }
    }
}

impl Layout for ScatterPolar {}
impl Marker for ScatterPolar {}
impl Polar for ScatterPolar {}

impl PlotHelper for ScatterPolar {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }

    fn get_layout_override(&self) -> Option<&serde_json::Value> {
        self.layout_json.as_ref()
    }
}
