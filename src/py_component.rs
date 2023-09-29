use pyo3::prelude::*;
use serde::Serialize;
use sysinfo::ComponentExt;

/// Getting a component temperature information.
#[derive(Debug, Serialize)]
#[pyclass(name = "Component")]
pub struct PyComponent {
    /// Returns the temperature of the component (in celsius degree).
    #[pyo3(get)]
    pub temperature: f32,

    /// Returns the maximum temperature of the component (in celsius degree).
    #[pyo3(get)]
    pub max: f32,

    /// Returns the highest temperature before the component halts (in celsius degree).
    #[pyo3(get)]
    pub critical: Option<f32>,

    /// Returns the label of the component.
    #[pyo3(get)]
    pub label: String,
}

impl From<&sysinfo::Component> for PyComponent {
    fn from(component: &sysinfo::Component) -> Self {
        Self {
            temperature: component.temperature(),
            max: component.max(),
            critical: component.critical(),
            label: component.label().to_string(),
        }
    }
}

common_methods!(PyComponent);
