use pyo3::prelude::*;

#[pyclass(name = "Networks")]
pub struct PyNetworks {}

#[pymethods]
impl PyNetworks {
    #[new]
    pub fn new() -> Self {
        Self {}
    }

    pub fn refresh_list(&mut self) {}
}
