use pyo3::prelude::*;

#[pyclass(name = "Cpu")]
pub struct PyCpu {
    #[pyo3(get)]
    pub cpu_usage: f32,

    #[pyo3(get)]
    pub name: String,

    #[pyo3(get)]
    pub vendor_id: String,

    #[pyo3(get)]
    pub brand: String,

    #[pyo3(get)]
    pub frequency: u64,
}

#[pymethods]
impl PyCpu {
    fn __repr__(&self) -> String {
        format!(
            "Cpu(cpu_usage={}, name={}, vendor_id={}, brand={}, frequency={})",
            self.cpu_usage, self.name, self.vendor_id, self.brand, self.frequency
        )
    }
}
