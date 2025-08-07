use plotly::surface::Lighting as LightingPlotly;

/// A structure describing the lighting model.
///
/// # Example
///
/// ```rust
/// use ndarray::Array;
/// use plotlars::{ColorBar, Lighting, Palette, Plot, SurfacePlot, Text};
/// use polars::prelude::*;
/// use std::iter;
///
/// let n: usize = 100;
/// let x_base: Vec<f64> = Array::linspace(-10.0, 10.0, n).into_raw_vec();
/// let y_base: Vec<f64> = Array::linspace(-10.0, 10.0, n).into_raw_vec();
///
/// let x = x_base
///     .iter()
///     .flat_map(|&xi| iter::repeat(xi).take(n))
///     .collect::<Vec<_>>();
///
/// let y = y_base
///     .iter()
///     .cycle()
///     .take(n * n)
///     .cloned()
///     .collect::<Vec<_>>();
///
/// let z = x_base
///     .iter()
///     .map(|i| {
///         y_base
///             .iter()
///             .map(|j| 1.0 / (j * j + 5.0) * j.sin() + 1.0 / (i * i + 5.0) * i.cos())
///             .collect::<Vec<_>>()
///     })
///     .flatten()
///     .collect::<Vec<_>>();
///
/// let dataset = df![
///         "x" => &x,
///         "y" => &y,
///         "z" => &z,
///     ]
///     .unwrap();
///
/// SurfacePlot::builder()
///     .data(&dataset)
///     .x("x")
///     .y("y")
///     .z("z")
///     .plot_title(
///         Text::from("Surface Plot")
///             .font("Arial")
///             .size(18),
///     )
///     .color_bar(
///         &ColorBar::new()
///             .border_width(1),
///     )
///     .color_scale(Palette::Cividis)
///     .reverse_scale(true)
///     .lighting(
///         &Lighting::new()
///             .position(1, 0, 0)
///             .ambient(1.0)
///             .diffuse(1.0)
///             .fresnel(1.0)
///             .roughness(1.0)
///             .specular(1.0),
///     )
///     .opacity(0.5)
///     .build()
///     .plot();
/// ```
///
/// ![example](https://imgur.com/LEjedUE.png)
#[derive(Default, Clone)]
pub struct Lighting {
    pub(crate) position: Option<[i32; 3]>,
    pub(crate) ambient: Option<f64>,
    pub(crate) diffuse: Option<f64>,
    pub(crate) fresnel: Option<f64>,
    pub(crate) roughness: Option<f64>,
    pub(crate) specular: Option<f64>,
}

impl Lighting {
    /// Creates a new `Lighting` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the position of the virtual light source.
    ///
    /// # Arguments
    ///
    /// * `x` – An `i32` value representing the *x*‑coordinate of the light.
    /// * `y` – An `i32` value representing the *y*‑coordinate of the light.
    /// * `z` – An `i32` value representing the *z*‑coordinate of the light (positive z points toward the viewer).
    pub fn position(mut self, x: i32, y: i32, z: i32) -> Self {
        self.position = Some([x, y, z]);
        self
    }

    /// Sets the ambient light component.
    ///
    /// # Arguments
    ///
    /// * `value` – A `f64` value in the range 0.0 – 1.0 specifying the uniform tint strength.
    pub fn ambient(mut self, value: f64) -> Self {
        self.ambient = Some(value);
        self
    }

    /// Sets the diffuse light component.
    ///
    /// # Arguments
    ///
    /// * `value` – A `f64` value in the range 0.0 – 1.0 specifying the Lambertian reflection strength.
    pub fn diffuse(mut self, value: f64) -> Self {
        self.diffuse = Some(value);
        self
    }

    /// Sets the Fresnel (edge brightness) component.
    ///
    /// # Arguments
    ///
    /// * `value` – A `f64` value in the range 0.0 – 1.0 specifying the rim‑light intensity.
    pub fn fresnel(mut self, value: f64) -> Self {
        self.fresnel = Some(value);
        self
    }

    /// Sets the roughness of the material.
    ///
    /// # Arguments
    ///
    /// * `value` – A `f64` value in the range 0.0 – 1.0 that controls highlight width (0.0 = glossy, 1.0 = matte).
    pub fn roughness(mut self, value: f64) -> Self {
        self.roughness = Some(value);
        self
    }

    /// Sets the specular highlight intensity.
    ///
    /// # Arguments
    ///
    /// * `value` – A `f64` value in the range 0.0 – 1.0 specifying the mirror‑like highlight strength.
    pub fn specular(mut self, value: f64) -> Self {
        self.specular = Some(value);
        self
    }

    pub(crate) fn set_lighting(lighting: Option<&Self>) -> LightingPlotly {
        let mut lighting_plotly = LightingPlotly::new();

        if let Some(light) = lighting {
            if let Some(ambient) = light.ambient {
                lighting_plotly = lighting_plotly.ambient(ambient);
            }

            if let Some(diffuse) = light.diffuse {
                lighting_plotly = lighting_plotly.diffuse(diffuse);
            }

            if let Some(fresnel) = light.fresnel {
                lighting_plotly = lighting_plotly.fresnel(fresnel);
            }

            if let Some(roughness) = light.roughness {
                lighting_plotly = lighting_plotly.roughness(roughness);
            }

            if let Some(specular) = light.specular {
                lighting_plotly = lighting_plotly.specular(specular);
            }
        }

        lighting_plotly
    }
}
