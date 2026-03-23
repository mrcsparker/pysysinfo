//! Python-facing process snapshots and live process-control helpers.

use std::process::ExitStatus;

use pyo3::exceptions::PyLookupError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use serde::Serialize;

use crate::data::PyDiskUsage;
use crate::pythonize::{
    repr_from_serialize, serialize_to_json, serialize_to_py_dict, tuple_from_vec,
};
use crate::state::{SharedState, lock_state};
use crate::sysconv::{
    optional_path_to_string, os_str_to_string, os_string_slice_to_strings, parse_signal,
    process_status_to_string, thread_kind_to_string,
};

/// Immutable snapshot of a process, with live control methods backed by the owning `Sysinfo`.
#[derive(Clone, Debug, Serialize)]
#[pyclass(name = "Process", module = "pysysinfo", frozen, skip_from_py_object)]
pub struct PyProcess {
    #[serde(skip)]
    state: SharedState,
    /// Process identifier.
    #[pyo3(get)]
    pub(crate) pid: u32,
    /// Process name as reported by the operating system.
    #[pyo3(get)]
    pub(crate) name: String,
    /// Command-line arguments.
    #[pyo3(get)]
    pub(crate) cmd: Vec<String>,
    /// Executable path, if available.
    #[pyo3(get)]
    pub(crate) exe: Option<String>,
    /// Environment variables.
    #[pyo3(get)]
    pub(crate) environ: Vec<String>,
    /// Current working directory, if available.
    #[pyo3(get)]
    pub(crate) cwd: Option<String>,
    /// Process root directory, if available.
    #[pyo3(get)]
    pub(crate) root: Option<String>,
    /// Resident memory in bytes.
    #[pyo3(get)]
    pub(crate) memory: u64,
    /// Virtual memory in bytes.
    #[pyo3(get)]
    pub(crate) virtual_memory: u64,
    /// Parent process ID, if available.
    #[pyo3(get)]
    pub(crate) parent: Option<u32>,
    /// Normalized process status string.
    #[pyo3(get)]
    pub(crate) status: String,
    /// Start time as seconds since the Unix epoch.
    #[pyo3(get)]
    pub(crate) start_time: u64,
    /// Runtime in seconds.
    #[pyo3(get)]
    pub(crate) run_time: u64,
    /// CPU usage percentage.
    #[pyo3(get)]
    pub(crate) cpu_usage: f32,
    /// Accumulated CPU time in CPU-milliseconds.
    #[pyo3(get)]
    pub(crate) accumulated_cpu_time: u64,
    disk_usage: PyDiskUsage,
    /// User ID of the process owner, if available.
    #[pyo3(get)]
    pub(crate) user_id: Option<String>,
    /// Effective user ID, if available.
    #[pyo3(get)]
    pub(crate) effective_user_id: Option<String>,
    /// Process group ID, if available.
    #[pyo3(get)]
    pub(crate) group_id: Option<String>,
    /// Effective group ID, if available.
    #[pyo3(get)]
    pub(crate) effective_group_id: Option<String>,
    /// Session ID, if available.
    #[pyo3(get)]
    pub(crate) session_id: Option<u32>,
    tasks: Option<Vec<u32>>,
    /// Thread kind on Linux when the process is actually a task/thread.
    #[pyo3(get)]
    pub(crate) thread_kind: Option<String>,
    /// Whether the process still exists in the most recent refresh.
    #[pyo3(get)]
    pub(crate) exists: bool,
    /// Number of open files, if available.
    #[pyo3(get)]
    pub(crate) open_files: Option<usize>,
    /// Open-files limit for this process, if available.
    #[pyo3(get)]
    pub(crate) open_files_limit: Option<usize>,
}

impl PyProcess {
    /// Capture an immutable Python snapshot from the current live `sysinfo::Process`.
    pub(crate) fn from_process(process: &sysinfo::Process, state: SharedState) -> Self {
        Self {
            state,
            pid: process.pid().as_u32(),
            name: os_str_to_string(process.name()),
            cmd: os_string_slice_to_strings(process.cmd()),
            exe: optional_path_to_string(process.exe()),
            environ: os_string_slice_to_strings(process.environ()),
            cwd: optional_path_to_string(process.cwd()),
            root: optional_path_to_string(process.root()),
            memory: process.memory(),
            virtual_memory: process.virtual_memory(),
            parent: process.parent().map(sysinfo::Pid::as_u32),
            status: process_status_to_string(process.status()),
            start_time: process.start_time(),
            run_time: process.run_time(),
            cpu_usage: process.cpu_usage(),
            accumulated_cpu_time: process.accumulated_cpu_time(),
            disk_usage: PyDiskUsage::from(process.disk_usage()),
            user_id: process.user_id().map(|value| value.to_string()),
            effective_user_id: process.effective_user_id().map(|value| value.to_string()),
            group_id: process.group_id().map(|value| value.to_string()),
            effective_group_id: process.effective_group_id().map(|value| value.to_string()),
            session_id: process.session_id().map(sysinfo::Pid::as_u32),
            tasks: process
                .tasks()
                .map(|tasks| tasks.iter().map(|pid| pid.as_u32()).collect()),
            thread_kind: process.thread_kind().map(thread_kind_to_string),
            exists: process.exists(),
            open_files: process.open_files(),
            open_files_limit: process.open_files_limit(),
        }
    }

    /// Resolve this snapshot back to a live `sysinfo::Process` owned by the parent `Sysinfo`.
    ///
    /// The extra `start_time` check guards against PID reuse so live control
    /// operations cannot silently act on a different process than the one this
    /// snapshot was created from.
    fn with_live_process<T>(
        &self,
        action: impl FnOnce(&sysinfo::Process) -> PyResult<T>,
    ) -> PyResult<T> {
        let state = lock_state(&self.state)?;
        let pid = sysinfo::Pid::from_u32(self.pid);
        let process = state.system.process(pid).ok_or_else(|| {
            PyLookupError::new_err(format!(
                "process {} is no longer available in the owning Sysinfo snapshot",
                self.pid
            ))
        })?;

        if process.start_time() != self.start_time {
            return Err(PyLookupError::new_err(format!(
                "process {} now refers to a different process than this snapshot",
                self.pid
            )));
        }

        action(process)
    }
}

#[pymethods]
impl PyProcess {
    #[getter]
    /// Disk I/O counters for the process.
    fn disk_usage<'py>(&self, py: Python<'py>) -> PyResult<Py<PyDiskUsage>> {
        Py::new(py, self.disk_usage.clone())
    }

    #[getter]
    /// Task or thread PIDs for the process when the platform exposes them.
    fn tasks<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyTuple>>> {
        self.tasks
            .as_ref()
            .map(|tasks| tuple_from_vec(py, tasks.clone()))
            .transpose()
    }

    /// Serialize this process snapshot to JSON.
    fn to_json(&self) -> PyResult<String> {
        serialize_to_json(self)
    }

    /// Convert this process snapshot into a plain Python dictionary.
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        serialize_to_py_dict(py, self)
    }

    /// Send `SIGKILL` (or the platform equivalent) to the process.
    fn kill(&self) -> PyResult<bool> {
        self.with_live_process(|process| Ok(process.kill()))
    }

    /// Send an explicit signal to the process.
    ///
    /// The `signal` argument is case-insensitive and accepts names like
    /// `"kill"`, `"term"`, `"sigterm"`, or `"floating_point_exception"`.
    fn kill_with(&self, signal: &str) -> PyResult<Option<bool>> {
        let signal = parse_signal(signal)?;
        self.with_live_process(|process| Ok(process.kill_with(signal)))
    }

    /// Kill the process and wait for it to exit.
    fn kill_and_wait<'py>(&self, py: Python<'py>) -> PyResult<Option<Py<PyExitStatus>>> {
        self.with_live_process(|process| {
            let status = process.kill_and_wait().map_err(|error| {
                pyo3::exceptions::PyRuntimeError::new_err(format!(
                    "failed to kill process {}: {error}",
                    self.pid
                ))
            })?;
            status
                .map(|status| Py::new(py, PyExitStatus::from(status)))
                .transpose()
        })
    }

    /// Send an explicit signal and wait for the process to exit.
    fn kill_with_and_wait<'py>(
        &self,
        py: Python<'py>,
        signal: &str,
    ) -> PyResult<Option<Py<PyExitStatus>>> {
        let signal = parse_signal(signal)?;
        self.with_live_process(|process| {
            let status = process.kill_with_and_wait(signal).map_err(|error| {
                pyo3::exceptions::PyRuntimeError::new_err(format!(
                    "failed to signal process {}: {error}",
                    self.pid
                ))
            })?;
            status
                .map(|status| Py::new(py, PyExitStatus::from(status)))
                .transpose()
        })
    }

    /// Wait for the process to exit.
    ///
    /// This method blocks until the process finishes or until `sysinfo` can no
    /// longer wait on that PID.
    fn wait<'py>(&self, py: Python<'py>) -> PyResult<Option<Py<PyExitStatus>>> {
        self.with_live_process(|process| {
            process
                .wait()
                .map(|status| Py::new(py, PyExitStatus::from(status)))
                .transpose()
        })
    }

    fn __repr__(&self) -> PyResult<String> {
        repr_from_serialize("Process", self)
    }
}

/// Structured view of a process exit status.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[pyclass(name = "ExitStatus", module = "pysysinfo", frozen, skip_from_py_object)]
pub struct PyExitStatus {
    /// Exit code if the process exited normally.
    #[pyo3(get)]
    pub(crate) code: Option<i32>,
    /// Whether the exit status represents success.
    #[pyo3(get)]
    pub(crate) success: bool,
    /// Signal number on Unix when the process exited because of a signal.
    #[pyo3(get)]
    pub(crate) unix_signal: Option<i32>,
}

impl From<ExitStatus> for PyExitStatus {
    fn from(status: ExitStatus) -> Self {
        Self {
            code: status.code(),
            success: status.success(),
            unix_signal: unix_signal(&status),
        }
    }
}

#[pymethods]
impl PyExitStatus {
    /// Serialize this exit status to JSON.
    fn to_json(&self) -> PyResult<String> {
        serialize_to_json(self)
    }

    /// Convert this exit status into a plain Python dictionary.
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        serialize_to_py_dict(py, self)
    }

    fn __repr__(&self) -> PyResult<String> {
        repr_from_serialize("ExitStatus", self)
    }
}

#[cfg(unix)]
fn unix_signal(status: &ExitStatus) -> Option<i32> {
    use std::os::unix::process::ExitStatusExt;

    status.signal()
}

#[cfg(not(unix))]
fn unix_signal(_status: &ExitStatus) -> Option<i32> {
    None
}
