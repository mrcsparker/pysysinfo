use pyo3::prelude::*;

#[pyclass(name = "Network")]
pub struct PyNetwork {
    #[pyo3(get)]
    pub interface: String,

    #[pyo3(get)]
    pub received: u64,

    #[pyo3(get)]
    pub total_received: u64,

    #[pyo3(get)]
    pub transmitted: u64,

    #[pyo3(get)]
    pub total_transmitted: u64,

    #[pyo3(get)]
    pub packets_received: u64,

    #[pyo3(get)]
    pub total_packets_received: u64,

    #[pyo3(get)]
    pub packets_transmitted: u64,

    #[pyo3(get)]
    pub total_packets_transmitted: u64,

    #[pyo3(get)]
    pub errors_on_received: u64,

    #[pyo3(get)]
    pub total_errors_on_received: u64,

    #[pyo3(get)]
    pub errors_on_transmitted: u64,

    #[pyo3(get)]
    pub total_errors_on_transmitted: u64,
    //#[pyo3(get)]
    //mac_address: PyMacAddr;
}

#[pymethods]
impl PyNetwork {
    fn __repr__(&self) -> String {
        format!(
            "Network(interface={}, received={}, total_received={}, transmitted={}, total_transmitted={}, packets_received={}, total_packets_received={}, packets_transmitted={}, total_packets_transmitted={}, errors_on_received={}, total_errors_on_received={},)",
            self.interface, self.received, self.total_received, self.transmitted, self.total_transmitted, self.packets_received, self.total_packets_received, self.packets_transmitted, self.total_packets_transmitted, self.errors_on_received, self.total_errors_on_received
        )
    }
}
