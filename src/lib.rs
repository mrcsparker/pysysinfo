//! Native extension module for the public `pysysinfo` Python package.

mod data;
mod process;
mod pythonize;
mod state;
mod sysconv;
mod system;

use data::{
    PyCGroupLimits, PyComponent, PyCpu, PyDisk, PyDiskUsage, PyGroup, PyIpNetwork, PyLoadAverage,
    PyMacAddress, PyMotherboard, PyNetwork, PyProduct, PyUser,
};
use process::{PyExitStatus, PyProcess};
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use system::PySystem;

use crate::sysconv::supported_signals;

/// Return the current process ID.
///
/// This is a small Python-friendly wrapper around `sysinfo::get_current_pid()`.
#[pyfunction]
fn get_current_pid() -> PyResult<u32> {
    sysinfo::get_current_pid()
        .map(sysinfo::Pid::as_u32)
        .map_err(pyo3::exceptions::PyRuntimeError::new_err)
}

/// Update the Linux open-files limit used internally by `sysinfo`.
///
/// On unsupported platforms this simply returns `False`.
#[pyfunction]
fn set_open_files_limit(limit: usize) -> bool {
    sysinfo::set_open_files_limit(limit)
}

/// Native extension for the public `pysysinfo` Python package.
#[pymodule]
#[pyo3(name = "_core")]
fn pysysinfo_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCGroupLimits>()?;
    m.add_class::<PyComponent>()?;
    m.add_class::<PyCpu>()?;
    m.add_class::<PyDisk>()?;
    m.add_class::<PyDiskUsage>()?;
    m.add_class::<PyExitStatus>()?;
    m.add_class::<PyGroup>()?;
    m.add_class::<PyIpNetwork>()?;
    m.add_class::<PyLoadAverage>()?;
    m.add_class::<PyMacAddress>()?;
    m.add_class::<PyMotherboard>()?;
    m.add_class::<PyNetwork>()?;
    m.add_class::<PyProcess>()?;
    m.add_class::<PyProduct>()?;
    m.add_class::<PySystem>()?;
    m.add_class::<PyUser>()?;
    m.add_function(wrap_pyfunction!(get_current_pid, m)?)?;
    m.add_function(wrap_pyfunction!(set_open_files_limit, m)?)?;
    m.add("IS_SUPPORTED_SYSTEM", sysinfo::IS_SUPPORTED_SYSTEM)?;
    m.add(
        "MINIMUM_CPU_UPDATE_INTERVAL",
        sysinfo::MINIMUM_CPU_UPDATE_INTERVAL.as_secs_f64(),
    )?;
    m.add(
        "SUPPORTED_SIGNALS",
        PyTuple::new(m.py(), supported_signals())?,
    )?;
    Ok(())
}
