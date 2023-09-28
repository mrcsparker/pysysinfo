use pyo3::prelude::*;
use sysinfo::CpuExt;

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

impl From<&sysinfo::Cpu> for PyCpu {
    fn from(cpu: &sysinfo::Cpu) -> Self {
        Self {
            cpu_usage: cpu.cpu_usage(),
            name: cpu.name().to_string(),
            vendor_id: cpu.vendor_id().to_string(),
            brand: cpu.brand().to_string(),
            frequency: cpu.frequency(),
        }
    }
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
