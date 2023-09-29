use pyo3::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[pyclass(name = "LoadAvg")]
pub struct PyLoadAvg {
    /// Average load within one minute.
    #[pyo3(get)]
    pub one: f64,

    /// Average load within five minutes.
    #[pyo3(get)]
    pub five: f64,

    /// Average load within fifteen minutes.
    #[pyo3(get)]
    pub fifteen: f64,
}

common_methods!(PyLoadAvg);
