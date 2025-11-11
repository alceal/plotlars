use plotly::Trace;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq)]
pub(super) enum PlotType {
    Cartesian2D,
    Cartesian3D,
    Polar,
}

pub(super) fn detect_plot_type(trace: &(dyn Trace + 'static)) -> PlotType {
    let json_str = trace.to_json();
    if let Ok(json) = serde_json::from_str::<Value>(&json_str) {
        if let Some(trace_type) = json.get("type").and_then(|v| v.as_str()) {
            match trace_type {
                "scatter3d" | "mesh3d" | "surface" | "isosurface" | "volume" | "streamtube"
                | "cone" => PlotType::Cartesian3D,
                "scatterpolar" | "scatterpolargl" | "barpolar" => PlotType::Polar,
                _ => PlotType::Cartesian2D,
            }
        } else {
            PlotType::Cartesian2D
        }
    } else {
        PlotType::Cartesian2D
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct JsonTrace {
    #[serde(flatten)]
    data: Value,
}

impl JsonTrace {
    pub(super) fn new(trace: Box<dyn Trace + 'static>) -> Self {
        let json_string = trace.to_json();
        let data = serde_json::from_str(&json_string).unwrap();
        Self { data }
    }

    pub(super) fn set_axis_references(&mut self, x_axis: &str, y_axis: &str) {
        if let Some(obj) = self.data.as_object_mut() {
            obj.insert("xaxis".to_string(), json!(x_axis));
            obj.insert("yaxis".to_string(), json!(y_axis));
        }
    }
}

impl Trace for JsonTrace {
    fn to_json(&self) -> String {
        serde_json::to_string(&self.data).unwrap()
    }
}
