use std::env;

use plotly::{Layout, Plot as Plotly, Trace};

/// A trait representing a generic plot that can be displayed or rendered.
pub trait Plot {
    fn get_layout(&self) -> &Layout;

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>>;

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

    fn write_html(self, path: impl Into<String>)
    where
        Self: Sized,
    {
        let mut plot = Plotly::new();

        plot.set_layout(self.get_layout().clone());
        plot.add_traces(self.get_traces().clone());

        plot.write_html(path.into());
    }
}
