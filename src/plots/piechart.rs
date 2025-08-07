use bon::bon;

use plotly::{Layout as LayoutPlotly, Pie, Trace};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper, Polar},
    components::Text,
};

/// A structure representing a pie chart.
///
/// The `PieChart` struct allows for the creation and customization of pie charts, supporting
/// features such as labels, hole size for donut-style charts, slice pulling, rotation, and customizable plot titles.
/// It is ideal for visualizing proportions and distributions in categorical data.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `labels` - A string slice specifying the column name to be used for slice labels.
/// * `hole` - An optional `f64` value specifying the size of the hole in the center of the pie chart.
///   A value of `0.0` creates a full pie chart, while a value closer to `1.0` creates a thinner ring.
/// * `pull` - An optional `f64` value specifying the fraction by which each slice should be pulled out from the center.
/// * `rotation` - An optional `f64` value specifying the starting angle (in degrees) of the first slice.
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
///
/// # Example
///
/// ## Basic Pie Chart with Customization
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{PieChart, Plot, Text};
///
/// let dataset = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
///     .finish()
///     .unwrap()
///     .select([
///         col("species"),
///     ])
///     .collect()
///     .unwrap();
///
/// PieChart::builder()
///     .data(&dataset)
///     .labels("species")
///     .hole(0.4)
///     .pull(0.01)
///     .rotation(20.0)
///     .plot_title(
///         Text::from("Pie Chart")
///             .font("Arial")
///             .size(18)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/jE70hYS.png)
#[derive(Clone, Serialize)]
pub struct PieChart {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl PieChart {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        labels: &str,
        hole: Option<f64>,
        pull: Option<f64>,
        rotation: Option<f64>,
        plot_title: Option<Text>,
    ) -> Self {
        let x_title = None;
        let y_title = None;
        let z_title = None;
        let legend_title = None;
        let x_axis = None;
        let y_axis = None;
        let z_axis = None;
        let legend = None;
        let y2_title = None;
        let y2_axis = None;

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

        let mut traces = vec![];

        let trace = Self::create_trace(data, labels, hole, pull, rotation);

        traces.push(trace);

        Self { traces, layout }
    }

    fn create_trace(
        data: &DataFrame,
        labels: &str,
        hole: Option<f64>,
        pull: Option<f64>,
        rotation: Option<f64>,
    ) -> Box<dyn Trace + 'static> {
        let labels = Self::get_string_column(data, labels)
            .iter()
            .filter_map(|s| {
                if s.is_some() {
                    Some(s.clone().unwrap().to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        let mut trace = Pie::<u32>::from_labels(&labels);

        if let Some(hole) = hole {
            trace = trace.hole(hole);
        }

        if let Some(pull) = pull {
            trace = trace.pull(pull);
        }

        if let Some(rotation) = rotation {
            trace = trace.rotation(rotation);
        }

        trace
    }
}

impl Layout for PieChart {}
impl Polar for PieChart {}

impl PlotHelper for PieChart {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
