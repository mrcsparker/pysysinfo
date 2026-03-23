//! Python-facing `System` facade and the bulk of the public binding surface.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple, PyType};

use crate::data::{PyCGroupLimits, PyLoadAverage, PyMotherboard, PyProduct, PyUser};
use crate::process::PyProcess;
use crate::pythonize::{
    serialize_to_json, serialize_to_py_dict, tuple_from_mapped_vec, tuple_from_vec,
};
use crate::state::{ProcessRefreshOptions, SharedState, SystemState, lock_state, new_shared_state};
use crate::sysconv::{parse_update_kind, supported_signals};

/// Internal normalized request used to implement `new_with_specifics` and `refresh_specifics`
/// without duplicating the parsing and validation logic.
#[derive(Debug, Clone)]
struct RefreshSpecificsRequest {
    memory: bool,
    ram: bool,
    swap: bool,
    cpu: bool,
    cpu_usage: bool,
    cpu_frequency: bool,
    processes: bool,
    process_ids: Option<Vec<u32>>,
    remove_dead_processes: bool,
    process_cpu: bool,
    process_disk_usage: bool,
    process_memory: bool,
    process_user: String,
    process_cwd: String,
    process_root: String,
    process_environ: String,
    process_cmd: String,
    process_exe: String,
    process_tasks: bool,
}

/// Python-first facade over the Rust `sysinfo` collectors.
#[derive(Debug)]
#[pyclass(name = "System", module = "pysysinfo")]
pub struct PySystem {
    state: SharedState,
}

impl Default for PySystem {
    fn default() -> Self {
        Self::new()
    }
}

impl PySystem {
    /// Wrap a freshly constructed `SystemState` inside the shared lock used by Python objects.
    fn from_state(state: SystemState) -> Self {
        Self {
            state: new_shared_state(state),
        }
    }

    /// Build a system with no initial refresh work performed.
    fn new_empty_internal() -> Self {
        Self::from_state(SystemState::new_empty())
    }

    /// Read from the shared state with poison handling mapped into a Python exception.
    fn with_state<T>(&self, f: impl FnOnce(&SystemState) -> T) -> PyResult<T> {
        let state = lock_state(&self.state)?;
        Ok(f(&state))
    }

    /// Mutate the shared state with poison handling mapped into a Python exception.
    fn with_state_mut<T>(&self, f: impl FnOnce(&mut SystemState) -> T) -> PyResult<T> {
        let mut state = lock_state(&self.state)?;
        Ok(f(&mut state))
    }

    /// Apply the selective refresh request used by the high-level constructor and refresh API.
    fn apply_refresh_specifics(&self, request: RefreshSpecificsRequest) -> PyResult<usize> {
        let process_options = ProcessRefreshOptions {
            cpu: request.process_cpu,
            disk_usage: request.process_disk_usage,
            memory: request.process_memory,
            user: parse_update_kind(&request.process_user)?,
            cwd: parse_update_kind(&request.process_cwd)?,
            root: parse_update_kind(&request.process_root)?,
            environ: parse_update_kind(&request.process_environ)?,
            cmd: parse_update_kind(&request.process_cmd)?,
            exe: parse_update_kind(&request.process_exe)?,
            tasks: request.process_tasks,
        };

        self.with_state_mut(|state| {
            if request.memory {
                state.refresh_memory_specifics(request.ram, request.swap);
            }
            if request.cpu {
                state.refresh_cpu_specifics(request.cpu_usage, request.cpu_frequency);
            }
            if request.processes {
                state.refresh_processes_specifics(
                    request.process_ids.as_deref(),
                    request.remove_dead_processes,
                    process_options,
                )
            } else {
                0
            }
        })
    }
}

#[pymethods]
impl PySystem {
    /// Create a new `System` with an initial full snapshot already loaded.
    #[new]
    pub fn new() -> Self {
        Self::from_state(SystemState::new())
    }

    /// Create a `System` with nothing preloaded.
    ///
    /// This is useful when callers want explicit control over which collectors are
    /// refreshed first and when that work happens.
    #[classmethod]
    pub fn new_empty(_cls: &Bound<'_, PyType>) -> Self {
        Self::new_empty_internal()
    }

    /// Create a `System` with explicit refresh controls similar to
    /// `sysinfo::System::new_with_specifics`.
    ///
    /// The `process_*` keyword arguments use the string values `"never"`,
    /// `"always"`, and `"only_if_not_set"` to mirror `sysinfo::UpdateKind`
    /// without exposing Rust-specific builder types in Python.
    #[classmethod]
    #[pyo3(signature = (
        *,
        memory = false,
        ram = true,
        swap = true,
        cpu = false,
        cpu_usage = true,
        cpu_frequency = true,
        processes = false,
        process_ids = None,
        remove_dead_processes = true,
        process_cpu = false,
        process_disk_usage = false,
        process_memory = false,
        process_user = "never",
        process_cwd = "never",
        process_root = "never",
        process_environ = "never",
        process_cmd = "never",
        process_exe = "never",
        process_tasks = true
    ))]
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_specifics(
        _cls: &Bound<'_, PyType>,
        memory: bool,
        ram: bool,
        swap: bool,
        cpu: bool,
        cpu_usage: bool,
        cpu_frequency: bool,
        processes: bool,
        process_ids: Option<Vec<u32>>,
        remove_dead_processes: bool,
        process_cpu: bool,
        process_disk_usage: bool,
        process_memory: bool,
        process_user: &str,
        process_cwd: &str,
        process_root: &str,
        process_environ: &str,
        process_cmd: &str,
        process_exe: &str,
        process_tasks: bool,
    ) -> PyResult<Self> {
        let system = Self::new_empty_internal();
        system.apply_refresh_specifics(RefreshSpecificsRequest {
            memory,
            ram,
            swap,
            cpu,
            cpu_usage,
            cpu_frequency,
            processes,
            process_ids,
            remove_dead_processes,
            process_cpu,
            process_disk_usage,
            process_memory,
            process_user: process_user.to_string(),
            process_cwd: process_cwd.to_string(),
            process_root: process_root.to_string(),
            process_environ: process_environ.to_string(),
            process_cmd: process_cmd.to_string(),
            process_exe: process_exe.to_string(),
            process_tasks,
        })?;
        Ok(system)
    }

    /// Refresh every collector owned by this `System`.
    pub fn refresh_all(&self) -> PyResult<()> {
        self.with_state_mut(|state| state.refresh_all())
    }

    /// Refresh only RAM and swap metrics.
    pub fn refresh_memory(&self) -> PyResult<()> {
        self.with_state_mut(|state| state.refresh_memory())
    }

    /// Refresh only the selected memory categories.
    #[pyo3(signature = (*, ram = true, swap = true))]
    pub fn refresh_memory_specifics(&self, ram: bool, swap: bool) -> PyResult<()> {
        self.with_state_mut(|state| state.refresh_memory_specifics(ram, swap))
    }

    /// Refresh CPU metrics.
    ///
    /// CPU usage is based on a diff, so callers should refresh twice with at least
    /// `MINIMUM_CPU_UPDATE_INTERVAL` between calls for accurate percentages.
    pub fn refresh_cpu(&self) -> PyResult<()> {
        self.with_state_mut(|state| state.refresh_cpu())
    }

    /// Refresh only CPU usage counters.
    pub fn refresh_cpu_usage(&self) -> PyResult<()> {
        self.with_state_mut(|state| state.refresh_cpu_usage())
    }

    /// Refresh only CPU frequency information.
    pub fn refresh_cpu_frequency(&self) -> PyResult<()> {
        self.with_state_mut(|state| state.refresh_cpu_frequency())
    }

    /// Refresh both CPU usage and frequency information.
    pub fn refresh_cpu_all(&self) -> PyResult<()> {
        self.with_state_mut(|state| state.refresh_cpu_all())
    }

    /// Refresh selected CPU fields without rebuilding the CPU list.
    #[pyo3(signature = (*, cpu_usage = true, frequency = true))]
    pub fn refresh_cpu_specifics(&self, cpu_usage: bool, frequency: bool) -> PyResult<()> {
        self.with_state_mut(|state| state.refresh_cpu_specifics(cpu_usage, frequency))
    }

    /// Rebuild the logical CPU list and refresh selected CPU fields.
    #[pyo3(signature = (*, cpu_usage = true, frequency = true))]
    pub fn refresh_cpu_list(&self, cpu_usage: bool, frequency: bool) -> PyResult<()> {
        self.with_state_mut(|state| state.refresh_cpu_list(cpu_usage, frequency))
    }

    /// Refresh process data using the default `sysinfo` process refresh behavior.
    ///
    /// When `pids` is omitted, all visible processes are refreshed.
    /// When `pids` is provided, only those process IDs are targeted.
    #[pyo3(signature = (pids = None, remove_dead_processes = true))]
    pub fn refresh_processes(
        &self,
        pids: Option<Vec<u32>>,
        remove_dead_processes: bool,
    ) -> PyResult<usize> {
        self.with_state_mut(|state| state.refresh_processes(pids.as_deref(), remove_dead_processes))
    }

    /// Refresh process data with precise control over which fields are updated.
    ///
    /// The string-valued refresh switches accept `"never"`, `"always"`, and
    /// `"only_if_not_set"`.
    #[pyo3(signature = (
        pids = None,
        remove_dead_processes = true,
        *,
        cpu = false,
        disk_usage = false,
        memory = false,
        user = "never",
        cwd = "never",
        root = "never",
        environ = "never",
        cmd = "never",
        exe = "never",
        tasks = true
    ))]
    #[allow(clippy::too_many_arguments)]
    pub fn refresh_processes_specifics(
        &self,
        pids: Option<Vec<u32>>,
        remove_dead_processes: bool,
        cpu: bool,
        disk_usage: bool,
        memory: bool,
        user: &str,
        cwd: &str,
        root: &str,
        environ: &str,
        cmd: &str,
        exe: &str,
        tasks: bool,
    ) -> PyResult<usize> {
        let options = ProcessRefreshOptions {
            cpu,
            disk_usage,
            memory,
            user: parse_update_kind(user)?,
            cwd: parse_update_kind(cwd)?,
            root: parse_update_kind(root)?,
            environ: parse_update_kind(environ)?,
            cmd: parse_update_kind(cmd)?,
            exe: parse_update_kind(exe)?,
            tasks,
        };

        self.with_state_mut(|state| {
            state.refresh_processes_specifics(pids.as_deref(), remove_dead_processes, options)
        })
    }

    /// Refresh the combination of memory, CPU, and process details specified by
    /// keyword flags.
    ///
    /// This is the broadest refresh entry point in the Python API and is most
    /// useful when you want one place to describe the exact refresh policy for a
    /// monitoring loop.
    #[pyo3(signature = (
        *,
        memory = false,
        ram = true,
        swap = true,
        cpu = false,
        cpu_usage = true,
        cpu_frequency = true,
        processes = false,
        process_ids = None,
        remove_dead_processes = true,
        process_cpu = false,
        process_disk_usage = false,
        process_memory = false,
        process_user = "never",
        process_cwd = "never",
        process_root = "never",
        process_environ = "never",
        process_cmd = "never",
        process_exe = "never",
        process_tasks = true
    ))]
    #[allow(clippy::too_many_arguments)]
    pub fn refresh_specifics(
        &self,
        memory: bool,
        ram: bool,
        swap: bool,
        cpu: bool,
        cpu_usage: bool,
        cpu_frequency: bool,
        processes: bool,
        process_ids: Option<Vec<u32>>,
        remove_dead_processes: bool,
        process_cpu: bool,
        process_disk_usage: bool,
        process_memory: bool,
        process_user: &str,
        process_cwd: &str,
        process_root: &str,
        process_environ: &str,
        process_cmd: &str,
        process_exe: &str,
        process_tasks: bool,
    ) -> PyResult<usize> {
        self.apply_refresh_specifics(RefreshSpecificsRequest {
            memory,
            ram,
            swap,
            cpu,
            cpu_usage,
            cpu_frequency,
            processes,
            process_ids,
            remove_dead_processes,
            process_cpu,
            process_disk_usage,
            process_memory,
            process_user: process_user.to_string(),
            process_cwd: process_cwd.to_string(),
            process_root: process_root.to_string(),
            process_environ: process_environ.to_string(),
            process_cmd: process_cmd.to_string(),
            process_exe: process_exe.to_string(),
            process_tasks,
        })
    }

    /// Refresh disk metrics and reconcile the disk list.
    pub fn refresh_disks(&self) -> PyResult<()> {
        self.with_state_mut(|state| state.refresh_disks())
    }

    /// Refresh selected disk details.
    #[pyo3(signature = (remove_not_listed_disks = true, *, kind = true, storage = true, io_usage = true))]
    pub fn refresh_disks_specifics(
        &self,
        remove_not_listed_disks: bool,
        kind: bool,
        storage: bool,
        io_usage: bool,
    ) -> PyResult<()> {
        self.with_state_mut(|state| {
            state.refresh_disks_specifics(remove_not_listed_disks, kind, storage, io_usage)
        })
    }

    /// Refresh network metrics and reconcile the interface list.
    pub fn refresh_networks(&self) -> PyResult<()> {
        self.with_state_mut(|state| state.refresh_networks())
    }

    /// Refresh component metrics and reconcile the sensor list.
    pub fn refresh_components(&self) -> PyResult<()> {
        self.with_state_mut(|state| state.refresh_components())
    }

    /// Refresh the user list.
    pub fn refresh_users(&self) -> PyResult<()> {
        self.with_state_mut(|state| state.refresh_users())
    }

    /// Refresh the system group list.
    pub fn refresh_groups(&self) -> PyResult<()> {
        self.with_state_mut(|state| state.refresh_groups())
    }

    #[getter]
    /// Immutable tuple of logical CPU snapshots in native logical order.
    fn cpus<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let values = self.with_state(|state| state.cpus())?;
        tuple_from_mapped_vec(py, values, |py, value| Ok(Py::new(py, value)?.into_any()))
    }

    #[getter]
    /// Immutable tuple of disk snapshots, sorted by mount point then name.
    fn disks<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let values = self.with_state(|state| state.disks())?;
        tuple_from_mapped_vec(py, values, |py, value| Ok(Py::new(py, value)?.into_any()))
    }

    #[getter]
    /// Immutable tuple of network snapshots, sorted by interface name.
    fn networks<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let values = self.with_state(|state| state.networks())?;
        tuple_from_mapped_vec(py, values, |py, value| Ok(Py::new(py, value)?.into_any()))
    }

    #[getter]
    /// Immutable tuple of component snapshots, sorted by label.
    fn components<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let values = self.with_state(|state| state.components())?;
        tuple_from_mapped_vec(py, values, |py, value| Ok(Py::new(py, value)?.into_any()))
    }

    #[getter]
    /// Immutable tuple of user snapshots, sorted by user name.
    fn users<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let values = self.with_state(|state| state.users())?;
        tuple_from_mapped_vec(py, values, |py, value| Ok(Py::new(py, value)?.into_any()))
    }

    #[getter]
    /// Immutable tuple of system groups, sorted by group name.
    fn groups<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let values = self.with_state(|state| state.groups())?;
        tuple_from_mapped_vec(py, values, |py, value| Ok(Py::new(py, value)?.into_any()))
    }

    #[getter]
    /// Immutable tuple of process snapshots, sorted by PID.
    fn processes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let values = self.with_state(|state| state.processes(self.state.clone()))?;
        tuple_from_mapped_vec(py, values, |py, value| Ok(Py::new(py, value)?.into_any()))
    }

    #[getter]
    /// Total physical memory in bytes.
    fn total_memory(&self) -> PyResult<u64> {
        self.with_state(|state| state.system.total_memory())
    }

    #[getter]
    /// Free memory in bytes.
    fn free_memory(&self) -> PyResult<u64> {
        self.with_state(|state| state.system.free_memory())
    }

    #[getter]
    /// Memory currently available for reuse in bytes.
    fn available_memory(&self) -> PyResult<u64> {
        self.with_state(|state| state.system.available_memory())
    }

    #[getter]
    /// Used memory in bytes.
    fn used_memory(&self) -> PyResult<u64> {
        self.with_state(|state| state.system.used_memory())
    }

    #[getter]
    /// Total swap capacity in bytes.
    fn total_swap(&self) -> PyResult<u64> {
        self.with_state(|state| state.system.total_swap())
    }

    #[getter]
    /// Free swap in bytes.
    fn free_swap(&self) -> PyResult<u64> {
        self.with_state(|state| state.system.free_swap())
    }

    #[getter]
    /// Used swap in bytes.
    fn used_swap(&self) -> PyResult<u64> {
        self.with_state(|state| state.system.used_swap())
    }

    #[getter]
    /// Cgroup limits for the current process, when available.
    fn cgroup_limits<'py>(&self, py: Python<'py>) -> PyResult<Option<Py<PyCGroupLimits>>> {
        let limits =
            self.with_state(|state| state.system.cgroup_limits().map(PyCGroupLimits::from))?;
        limits.map(|limits| Py::new(py, limits)).transpose()
    }

    #[getter]
    /// System uptime in seconds.
    fn uptime(&self) -> u64 {
        sysinfo::System::uptime()
    }

    #[getter]
    /// Boot time as seconds since the Unix epoch.
    fn boot_time(&self) -> u64 {
        sysinfo::System::boot_time()
    }

    #[getter]
    /// The current load-average snapshot.
    fn load_average(&self, py: Python<'_>) -> PyResult<Py<PyLoadAverage>> {
        Py::new(py, self.with_state(|state| state.load_average())?)
    }

    #[getter]
    /// Operating system name, if available.
    fn name(&self) -> Option<String> {
        sysinfo::System::name()
    }

    #[getter]
    /// Kernel version string, if available.
    fn kernel_version(&self) -> Option<String> {
        sysinfo::System::kernel_version()
    }

    #[getter]
    /// Human-friendly kernel description.
    fn kernel_long_version(&self) -> String {
        sysinfo::System::kernel_long_version()
    }

    #[getter]
    /// Operating system version, if available.
    fn os_version(&self) -> Option<String> {
        sysinfo::System::os_version()
    }

    #[getter]
    /// Longer operating system version string, if available.
    fn long_os_version(&self) -> Option<String> {
        sysinfo::System::long_os_version()
    }

    #[getter]
    /// Distribution identifier such as `ubuntu`, `macos`, or `windows`.
    fn distribution_id(&self) -> String {
        sysinfo::System::distribution_id()
    }

    #[getter]
    /// Related distribution identifiers reported by the operating system.
    fn distribution_id_like<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        tuple_from_vec(py, sysinfo::System::distribution_id_like())
    }

    #[getter]
    /// Host name, if available.
    fn host_name(&self) -> Option<String> {
        sysinfo::System::host_name()
    }

    #[getter]
    /// CPU architecture such as `x86_64` or `aarch64`.
    fn cpu_arch(&self) -> String {
        sysinfo::System::cpu_arch()
    }

    #[getter]
    /// Number of physical CPU cores, if the platform can report it.
    fn physical_core_count(&self) -> Option<usize> {
        sysinfo::System::physical_core_count()
    }

    #[getter]
    /// Aggregate CPU usage percentage across all logical CPUs.
    fn global_cpu_usage(&self) -> PyResult<f32> {
        self.with_state(|state| state.system.global_cpu_usage())
    }

    #[getter]
    /// System-wide open-files limit, if available.
    fn open_files_limit(&self) -> Option<usize> {
        sysinfo::System::open_files_limit()
    }

    #[getter]
    /// Motherboard metadata when the current platform supports it.
    fn motherboard<'py>(&self, py: Python<'py>) -> PyResult<Option<Py<PyMotherboard>>> {
        PyMotherboard::collect()
            .map(|value| Py::new(py, value))
            .transpose()
    }

    #[getter]
    /// Product metadata for the current machine.
    fn product<'py>(&self, py: Python<'py>) -> PyResult<Py<PyProduct>> {
        Py::new(py, PyProduct::collect())
    }

    #[getter]
    /// Signals supported by the current platform, exposed as lowercase names.
    fn supported_signals<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        tuple_from_vec(py, supported_signals())
    }

    #[getter]
    /// Whether the current platform is supported by `sysinfo`.
    fn is_supported_system(&self) -> bool {
        sysinfo::IS_SUPPORTED_SYSTEM
    }

    #[getter]
    /// Minimum interval between CPU usage refreshes, in seconds.
    fn minimum_cpu_update_interval(&self) -> f64 {
        sysinfo::MINIMUM_CPU_UPDATE_INTERVAL.as_secs_f64()
    }

    /// Return the process with the given PID, if present.
    ///
    /// The returned `Process` is an immutable snapshot with live control methods
    /// that still talk to this owning `System`.
    fn process<'py>(&self, py: Python<'py>, pid: u32) -> PyResult<Option<Py<PyProcess>>> {
        let process = self.with_state(|state| state.process(pid, self.state.clone()))?;
        process.map(|process| Py::new(py, process)).transpose()
    }

    /// Find processes whose names contain the given string.
    ///
    /// This mirrors `sysinfo`'s substring search semantics.
    fn processes_by_name<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyTuple>> {
        let processes =
            self.with_state(|state| state.processes_by_name(name, self.state.clone()))?;
        tuple_from_mapped_vec(
            py,
            processes,
            |py, value| Ok(Py::new(py, value)?.into_any()),
        )
    }

    /// Find processes whose names exactly match the given string.
    fn processes_by_exact_name<'py>(
        &self,
        py: Python<'py>,
        name: &str,
    ) -> PyResult<Bound<'py, PyTuple>> {
        let processes =
            self.with_state(|state| state.processes_by_exact_name(name, self.state.clone()))?;
        tuple_from_mapped_vec(
            py,
            processes,
            |py, value| Ok(Py::new(py, value)?.into_any()),
        )
    }

    /// Look up a user by its operating-system user ID.
    ///
    /// The `user_id` string should use the same textual form returned by
    /// `Process.user_id` or `User.id`.
    fn get_user_by_id<'py>(&self, py: Python<'py>, user_id: &str) -> PyResult<Option<Py<PyUser>>> {
        let user = self.with_state(|state| state.get_user_by_id(user_id))?;
        user.map(|user| Py::new(py, user)).transpose()
    }

    /// Serialize the complete system snapshot to JSON.
    fn to_json(&self) -> PyResult<String> {
        let snapshot = self.with_state(|state| state.snapshot(self.state.clone()))?;
        serialize_to_json(&snapshot)
    }

    /// Convert the complete system snapshot into a plain Python dictionary.
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let snapshot = self.with_state(|state| state.snapshot(self.state.clone()))?;
        serialize_to_py_dict(py, &snapshot)
    }

    fn __repr__(&self) -> PyResult<String> {
        let counts = self.with_state(|state| state.collection_counts())?;
        Ok(format!(
            "System(name={}, cpus={}, disks={}, networks={}, components={}, users={}, groups={}, processes={})",
            repr_optional_string(&sysinfo::System::name()),
            counts.cpus,
            counts.disks,
            counts.networks,
            counts.components,
            counts.users,
            counts.groups,
            counts.processes,
        ))
    }
}

fn repr_optional_string(value: &Option<String>) -> String {
    match value {
        Some(value) => format!("{value:?}"),
        None => "None".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::PySystem;

    #[test]
    fn repr_is_readable() {
        let system = PySystem::new();
        let repr = system.__repr__().unwrap();

        assert!(repr.starts_with("System("));
        assert!(repr.contains("groups="));
        assert!(repr.contains("processes="));
    }
}
