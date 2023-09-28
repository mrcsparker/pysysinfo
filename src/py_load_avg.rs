use pyo3::prelude::*;

#[derive(Clone, Debug)]
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

impl PyLoadAvg {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}
