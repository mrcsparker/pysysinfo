use pyo3::prelude::*;
use sysinfo::UserExt;

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

impl From<&sysinfo::User> for PyUser {
    fn from(user: &sysinfo::User) -> Self {
        Self {
            id: user.id().to_string(),
            group_id: user.group_id().to_string(),
            name: user.name().to_string(),
            groups: user.groups().to_vec(),
        }
    }
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
