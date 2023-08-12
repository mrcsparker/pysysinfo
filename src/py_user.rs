use pyo3::prelude::*;

#[pyclass(name = "User")]
pub struct PyUser {
    #[pyo3(get)]
    pub id: String,

    #[pyo3(get)]
    pub group_id: String,

    #[pyo3(get)]
    pub name: String,

    #[pyo3(get)]
    pub groups: Vec<String>,
}

#[pymethods]
impl PyUser {
    fn __repr__(&self) -> String {
        format!(
            "User(id={}, group_id={}, name={}, groups=[{}])",
            self.id,
            self.group_id,
            self.name,
            self.groups.join(", ")
        )
    }
}
