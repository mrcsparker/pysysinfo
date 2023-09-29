use pyo3::prelude::*;
use serde::Serialize;
use sysinfo::UserExt;

/// Getting information for a user.
#[derive(Debug, Serialize)]
#[pyclass(name = "User")]
pub struct PyUser {
    /// Return the user id of the user.
    #[pyo3(get)]
    pub id: String,

    /// Return the group id of the user.
    #[pyo3(get)]
    pub group_id: String,

    /// Returns the name of the user.
    #[pyo3(get)]
    pub name: String,

    /// Returns the groups of the user.
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

common_methods!(PyUser);
