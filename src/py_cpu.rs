use pyo3::prelude::*;
use serde::Serialize;
use sysinfo::CpuExt;

#[derive(Debug, Serialize)]
#[pyclass(name = "Cpu")]
pub struct PyCpu {
    /// Returns this CPU's usage.
    #[pyo3(get)]
    pub cpu_usage: f32,

    /// Returns this CPU's name.
    #[pyo3(get)]
    pub name: String,

    /// Returns the CPU's vendor id.
    #[pyo3(get)]
    pub vendor_id: String,

    /// Returns the CPU's brand.
    #[pyo3(get)]
    pub brand: String,

    /// Returns the CPU's frequency.
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

common_methods!(PyCpu);
