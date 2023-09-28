mod py_component;
mod py_cpu;
mod py_disk;
mod py_load_avg;
mod py_network;
mod py_networks;
mod py_sysinfo;
mod py_user;

use py_component::PyComponent;
use py_cpu::PyCpu;
use py_disk::PyDisk;
use py_load_avg::PyLoadAvg;
use py_network::PyNetwork;
use py_networks::PyNetworks;
use py_sysinfo::PySysinfo;
use py_user::PyUser;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn pysysinfo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyComponent>()?;
    m.add_class::<PyCpu>()?;
    m.add_class::<PyDisk>()?;
    m.add_class::<PyLoadAvg>()?;
    m.add_class::<PyNetwork>()?;
    m.add_class::<PyNetworks>()?;
    m.add_class::<PySysinfo>()?;
    m.add_class::<PyUser>()?;
    Ok(())
}
