use std::env;

use plotly::{Layout, Plot as Plotly, Trace};

/// A trait representing a generic plot that can be displayed or rendered.
pub trait Plot {
    #[allow(dead_code)]
    fn get_layout(&self) -> &Layout;
    #[allow(dead_code)]
    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>>;
    #[allow(dead_code)]
    fn plot(self)
    where
        Self: Sized,
    {
        let mut plot = Plotly::new();
        plot.set_layout(self.get_layout().clone());
        plot.add_traces(self.get_traces().clone());

        match env::var("EVCXR_IS_RUNTIME") {
            Ok(_) => plot.notebook_display(),
            _ => plot.show(),
        }
    }
}
