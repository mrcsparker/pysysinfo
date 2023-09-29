use pyo3::prelude::*;
use serde::Serialize;

/// Getting volume of received and transmitted data.
#[derive(Debug, Serialize)]
#[pyclass(name = "Network")]
pub struct PyNetwork {
    #[pyo3(get)]
    pub interface: String,

    /// Returns the number of received bytes since the last refresh.
    #[pyo3(get)]
    pub received: u64,

    /// Returns the total number of received bytes.
    #[pyo3(get)]
    pub total_received: u64,

    /// Returns the number of transmitted bytes since the last refresh.
    #[pyo3(get)]
    pub transmitted: u64,

    /// Returns the total number of transmitted bytes.
    #[pyo3(get)]
    pub total_transmitted: u64,

    /// Returns the number of incoming packets since the last refresh.
    #[pyo3(get)]
    pub packets_received: u64,

    /// Returns the total number of incoming packets.
    #[pyo3(get)]
    pub total_packets_received: u64,

    /// Returns the number of outcoming packets since the last refresh.
    #[pyo3(get)]
    pub packets_transmitted: u64,

    /// Returns the total number of outcoming packets.
    #[pyo3(get)]
    pub total_packets_transmitted: u64,

    /// Returns the number of incoming errors since the last refresh.
    #[pyo3(get)]
    pub errors_on_received: u64,

    /// Returns the total number of incoming errors.
    #[pyo3(get)]
    pub total_errors_on_received: u64,

    /// Returns the number of outcoming errors since the last refresh.
    #[pyo3(get)]
    pub errors_on_transmitted: u64,

    /// Returns the total number of outcoming errors.
    #[pyo3(get)]
    pub total_errors_on_transmitted: u64,

    /// Returns the MAC address associated to current interface.
    #[pyo3(get)]
    pub mac_address: String,
}

common_methods!(PyNetwork);
