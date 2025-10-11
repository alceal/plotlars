use crate::components::Text;
use std::cmp::Ordering;

#[derive(Clone, Debug, Default)]
pub enum FacetScales {
    #[default]
    Fixed,
    Free,
    FreeX,
    FreeY,
}

#[derive(Clone, Default)]
pub struct FacetConfig {
    pub(crate) ncol: Option<usize>,
    pub(crate) nrow: Option<usize>,
    pub(crate) scales: FacetScales,
    pub(crate) x_gap: Option<f64>,
    pub(crate) y_gap: Option<f64>,
    pub(crate) title_style: Option<Text>,
    pub(crate) sorter: Option<fn(&str, &str) -> Ordering>,
}

impl FacetConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ncol(mut self, ncol: usize) -> Self {
        self.ncol = Some(ncol);
        self
    }

    pub fn nrow(mut self, nrow: usize) -> Self {
        self.nrow = Some(nrow);
        self
    }

    pub fn scales(mut self, scales: FacetScales) -> Self {
        self.scales = scales;
        self
    }

    pub fn x_gap(mut self, gap: f64) -> Self {
        self.x_gap = Some(gap);
        self
    }

    pub fn y_gap(mut self, gap: f64) -> Self {
        self.y_gap = Some(gap);
        self
    }

    pub fn title_style(mut self, style: Text) -> Self {
        self.title_style = Some(style);
        self
    }

    pub fn sorter(mut self, f: fn(&str, &str) -> Ordering) -> Self {
        self.sorter = Some(f);
        self
    }
}
