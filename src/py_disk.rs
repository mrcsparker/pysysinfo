use pyo3::prelude::*;

#[pyclass(name = "Disk")]
pub struct PyDisk {
    #[pyo3(get)]
    pub name: String,

    #[pyo3(get)]
    pub mount_point: String,

    #[pyo3(get)]
    pub total_space: u64,

    #[pyo3(get)]
    pub available_space: u64,

    #[pyo3(get)]
    pub is_removable: bool,
}

#[pymethods]
impl PyDisk {
    fn __repr__(&self) -> String {
        format!(
            "Disk(name={}, mount_point={}, total_space={}, available_space={}, is_removable={})",
            self.name, self.mount_point, self.total_space, self.available_space, self.is_removable
        )
    }
}
