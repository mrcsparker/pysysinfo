use pyo3::prelude::*;
use sysinfo::{Component, ComponentExt};

#[pyclass(name = "Component")]
pub struct PyComponent {
    #[pyo3(get)]
    pub temperature: f32,

    #[pyo3(get)]
    pub max: f32,

    #[pyo3(get)]
    pub critical: Option<f32>,

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

#[pymethods]
impl PyComponent {
    fn __repr__(&self) -> String {
        format!(
            "Component(temperature={}, max={}, critical={}, label='{}')",
            self.temperature,
            self.max,
            self.critical.unwrap_or(0.0),
            self.label
        )
    }
}
