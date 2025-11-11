use bon::bon;
use plotly::{layout::Layout as LayoutPlotly, Trace};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_json::Value;

use crate::common::{Layout, PlotHelper, Polar};
use crate::components::Text;

mod irregular;
mod regular;
mod shared;

#[derive(Clone)]
pub struct Subplot2 {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
    layout_json: Option<Value>,
}

impl Serialize for Subplot2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Subplot2", 2)?;
        state.serialize_field("data", &self.traces)?;

        if let Some(ref layout_json) = self.layout_json {
            state.serialize_field("layout", layout_json)?;
        } else {
            state.serialize_field("layout", &self.layout)?;
        }

        state.end()
    }
}

#[bon]
impl Subplot2 {
    #[builder(on(String, into), on(Text, into))]
    pub fn regular(
        plots: Vec<&dyn PlotHelper>,
        rows: Option<usize>,
        cols: Option<usize>,
        title: Option<Text>,
        h_gap: Option<f64>,
        v_gap: Option<f64>,
    ) -> Self {
        regular::build_regular(plots, rows, cols, title, h_gap, v_gap)
    }

    #[builder(on(String, into), on(Text, into))]
    pub fn irregular(
        plots: Vec<(&dyn PlotHelper, usize, usize, usize, usize)>,
        rows: Option<usize>,
        cols: Option<usize>,
        title: Option<Text>,
    ) -> Self {
        irregular::build_irregular(plots, rows, cols, title)
    }
}

impl Layout for Subplot2 {}
impl Polar for Subplot2 {}

impl PlotHelper for Subplot2 {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
