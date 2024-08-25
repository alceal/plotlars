use plotly::common::{DashType, Line as LinePlotly};

/// An enum representing different styles of lines that can be used in plots.
///
/// The `LineType` enum defines various styles of lines, such as solid, dashed, or dotted,
/// that can be applied to plot traces, such as lines in a line plot or borders in a bar plot.
///
/// # Variants
///
/// - `Solid`: A continuous, solid line with no breaks.
/// - `Dot`: A line composed of dots, spaced evenly.
/// - `Dash`: A dashed line with evenly spaced short dashes.
/// - `LongDash`: A dashed line with longer dashes than `Dash`.
/// - `DashDot`: A line that alternates between a dash and a dot.
/// - `LongDashDot`: A line that alternates between a long dash and a dot.
///
/// # Example Usage
///
/// ```rust
/// use crate::LineType;
///
/// let solid_line = LineType::Solid;
/// let dashed_line = LineType::Dash;
/// let custom_line = LineType::LongDashDot;
/// ```
#[derive(Clone)]
pub enum LineType {
    Solid,
    Dot,
    Dash,
    LongDash,
    DashDot,
    LongDashDot,
}

pub(crate) trait Line {
    fn create_line() -> LinePlotly {
        LinePlotly::new()
    }

    fn set_line_type(
        line: &LinePlotly,
        line_types: &Option<Vec<LineType>>,
        index: usize,
    ) -> LinePlotly {
        let mut updated_line = line.clone();

        if let Some(line_type_list) = line_types {
            if let Some(line_type) = line_type_list.get(index) {
                let line_style = Self::convert_line_type(line_type);
                updated_line = updated_line.dash(line_style);
            }
        }

        updated_line
    }

    fn convert_line_type(line_type: &LineType) -> DashType {
        match line_type {
            LineType::Solid => DashType::Solid,
            LineType::Dot => DashType::Dot,
            LineType::Dash => DashType::Dash,
            LineType::LongDash => DashType::LongDash,
            LineType::DashDot => DashType::DashDot,
            LineType::LongDashDot => DashType::LongDashDot,
        }
    }
}
