//! Shared state ownership and snapshot conversion for the Python-facing API.

use std::ffi::OsStr;
use std::sync::{Arc, Mutex, MutexGuard};

use pyo3::PyResult;
use pyo3::exceptions::PyRuntimeError;
use serde::Serialize;

use crate::data::{
    PyCGroupLimits, PyComponent, PyCpu, PyDisk, PyGroup, PyLoadAverage, PyMotherboard, PyNetwork,
    PyProduct, PyUser,
};
use crate::process::PyProcess;

/// Shared owner for the live collectors backing `Sysinfo` and any live `Process` objects.
pub(crate) type SharedState = Arc<Mutex<SystemState>>;

/// Parsed options for `Sysinfo.refresh_processes_specifics`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ProcessRefreshOptions {
    pub(crate) cpu: bool,
    pub(crate) disk_usage: bool,
    pub(crate) memory: bool,
    pub(crate) user: sysinfo::UpdateKind,
    pub(crate) cwd: sysinfo::UpdateKind,
    pub(crate) root: sysinfo::UpdateKind,
    pub(crate) environ: sysinfo::UpdateKind,
    pub(crate) cmd: sysinfo::UpdateKind,
    pub(crate) exe: sysinfo::UpdateKind,
    pub(crate) tasks: bool,
}

impl Default for ProcessRefreshOptions {
    fn default() -> Self {
        Self {
            cpu: false,
            disk_usage: false,
            memory: false,
            user: sysinfo::UpdateKind::Never,
            cwd: sysinfo::UpdateKind::Never,
            root: sysinfo::UpdateKind::Never,
            environ: sysinfo::UpdateKind::Never,
            cmd: sysinfo::UpdateKind::Never,
            exe: sysinfo::UpdateKind::Never,
            tasks: true,
        }
    }
}

/// Wrap a concrete `SystemState` in the shared mutex used across Python objects.
pub(crate) fn new_shared_state(state: SystemState) -> SharedState {
    Arc::new(Mutex::new(state))
}

/// Lock the shared state and convert poison errors into Python exceptions.
pub(crate) fn lock_state(state: &SharedState) -> PyResult<MutexGuard<'_, SystemState>> {
    state
        .lock()
        .map_err(|_| PyRuntimeError::new_err("system state lock was poisoned"))
}

/// Owns the live `sysinfo` collectors behind the Python-facing API.
pub(crate) struct SystemState {
    pub(crate) system: sysinfo::System,
    disks: sysinfo::Disks,
    networks: sysinfo::Networks,
    components: sysinfo::Components,
    users: sysinfo::Users,
    groups: sysinfo::Groups,
}

impl std::fmt::Debug for SystemState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SystemState")
            .field("cpus", &self.system.cpus().len())
            .field("disks", &self.disks.list().len())
            .field("networks", &self.networks.len())
            .field("components", &self.components.list().len())
            .field("users", &self.users.list().len())
            .field("groups", &self.groups.list().len())
            .field("processes", &self.system.processes().len())
            .finish()
    }
}

/// Serializable whole-system snapshot used by `Sysinfo.to_dict()` and `Sysinfo.to_json()`.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct SystemSnapshot {
    pub(crate) total_memory: u64,
    pub(crate) free_memory: u64,
    pub(crate) available_memory: u64,
    pub(crate) used_memory: u64,
    pub(crate) total_swap: u64,
    pub(crate) free_swap: u64,
    pub(crate) used_swap: u64,
    pub(crate) cgroup_limits: Option<PyCGroupLimits>,
    pub(crate) uptime: u64,
    pub(crate) boot_time: u64,
    pub(crate) load_average: PyLoadAverage,
    pub(crate) name: Option<String>,
    pub(crate) kernel_version: Option<String>,
    pub(crate) kernel_long_version: String,
    pub(crate) os_version: Option<String>,
    pub(crate) long_os_version: Option<String>,
    pub(crate) distribution_id: String,
    pub(crate) distribution_id_like: Vec<String>,
    pub(crate) host_name: Option<String>,
    pub(crate) cpu_arch: String,
    pub(crate) physical_core_count: Option<usize>,
    pub(crate) global_cpu_usage: f32,
    pub(crate) open_files_limit: Option<usize>,
    pub(crate) motherboard: Option<PyMotherboard>,
    pub(crate) product: PyProduct,
    pub(crate) cpus: Vec<PyCpu>,
    pub(crate) disks: Vec<PyDisk>,
    pub(crate) networks: Vec<PyNetwork>,
    pub(crate) components: Vec<PyComponent>,
    pub(crate) users: Vec<PyUser>,
    pub(crate) groups: Vec<PyGroup>,
    pub(crate) processes: Vec<PyProcess>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CollectionCounts {
    pub(crate) cpus: usize,
    pub(crate) disks: usize,
    pub(crate) networks: usize,
    pub(crate) components: usize,
    pub(crate) users: usize,
    pub(crate) groups: usize,
    pub(crate) processes: usize,
}

impl Default for SystemState {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemState {
    /// Create a state with broad, eager refreshes so the default `Sysinfo()` is immediately useful.
    pub(crate) fn new() -> Self {
        Self {
            system: sysinfo::System::new_all(),
            disks: sysinfo::Disks::new_with_refreshed_list(),
            networks: sysinfo::Networks::new_with_refreshed_list(),
            components: sysinfo::Components::new_with_refreshed_list(),
            users: sysinfo::Users::new_with_refreshed_list(),
            groups: sysinfo::Groups::new_with_refreshed_list(),
        }
    }

    /// Create a state with empty collectors so callers can control the first refresh themselves.
    pub(crate) fn new_empty() -> Self {
        Self {
            system: sysinfo::System::new(),
            disks: sysinfo::Disks::new(),
            networks: sysinfo::Networks::new(),
            components: sysinfo::Components::new(),
            users: sysinfo::Users::new(),
            groups: sysinfo::Groups::new(),
        }
    }

    pub(crate) fn refresh_all(&mut self) {
        self.system.refresh_all();
        self.disks.refresh(true);
        self.networks.refresh(true);
        self.components.refresh(true);
        self.users.refresh();
        self.groups.refresh();
    }

    pub(crate) fn refresh_memory(&mut self) {
        self.system.refresh_memory();
    }

    pub(crate) fn refresh_memory_specifics(&mut self, ram: bool, swap: bool) {
        let mut refresh_kind = sysinfo::MemoryRefreshKind::nothing();
        if ram {
            refresh_kind = refresh_kind.with_ram();
        }
        if swap {
            refresh_kind = refresh_kind.with_swap();
        }
        self.system.refresh_memory_specifics(refresh_kind);
    }

    pub(crate) fn refresh_cpu(&mut self) {
        self.system.refresh_cpu_all();
    }

    pub(crate) fn refresh_cpu_usage(&mut self) {
        self.system.refresh_cpu_usage();
    }

    pub(crate) fn refresh_cpu_frequency(&mut self) {
        self.system.refresh_cpu_frequency();
    }

    pub(crate) fn refresh_cpu_all(&mut self) {
        self.system.refresh_cpu_all();
    }

    pub(crate) fn refresh_cpu_specifics(&mut self, cpu_usage: bool, frequency: bool) {
        let mut refresh_kind = sysinfo::CpuRefreshKind::nothing();
        if cpu_usage {
            refresh_kind = refresh_kind.with_cpu_usage();
        }
        if frequency {
            refresh_kind = refresh_kind.with_frequency();
        }
        self.system.refresh_cpu_specifics(refresh_kind);
    }

    pub(crate) fn refresh_cpu_list(&mut self, cpu_usage: bool, frequency: bool) {
        let mut refresh_kind = sysinfo::CpuRefreshKind::nothing();
        if cpu_usage {
            refresh_kind = refresh_kind.with_cpu_usage();
        }
        if frequency {
            refresh_kind = refresh_kind.with_frequency();
        }
        self.system.refresh_cpu_list(refresh_kind);
    }

    pub(crate) fn refresh_processes(
        &mut self,
        pids: Option<&[u32]>,
        remove_dead_processes: bool,
    ) -> usize {
        match pids {
            Some(pids) => {
                let pids = pids
                    .iter()
                    .copied()
                    .map(sysinfo::Pid::from_u32)
                    .collect::<Vec<_>>();
                self.system.refresh_processes(
                    sysinfo::ProcessesToUpdate::Some(&pids),
                    remove_dead_processes,
                )
            }
            None => self
                .system
                .refresh_processes(sysinfo::ProcessesToUpdate::All, remove_dead_processes),
        }
    }

    pub(crate) fn refresh_processes_specifics(
        &mut self,
        pids: Option<&[u32]>,
        remove_dead_processes: bool,
        options: ProcessRefreshOptions,
    ) -> usize {
        let mut refresh_kind = sysinfo::ProcessRefreshKind::nothing();
        if options.cpu {
            refresh_kind = refresh_kind.with_cpu();
        }
        if options.disk_usage {
            refresh_kind = refresh_kind.with_disk_usage();
        }
        if options.memory {
            refresh_kind = refresh_kind.with_memory();
        }
        if options.user != sysinfo::UpdateKind::Never {
            refresh_kind = refresh_kind.with_user(options.user);
        }
        if options.cwd != sysinfo::UpdateKind::Never {
            refresh_kind = refresh_kind.with_cwd(options.cwd);
        }
        if options.root != sysinfo::UpdateKind::Never {
            refresh_kind = refresh_kind.with_root(options.root);
        }
        if options.environ != sysinfo::UpdateKind::Never {
            refresh_kind = refresh_kind.with_environ(options.environ);
        }
        if options.cmd != sysinfo::UpdateKind::Never {
            refresh_kind = refresh_kind.with_cmd(options.cmd);
        }
        if options.exe != sysinfo::UpdateKind::Never {
            refresh_kind = refresh_kind.with_exe(options.exe);
        }
        if !options.tasks {
            refresh_kind = refresh_kind.without_tasks();
        }

        match pids {
            Some(pids) => {
                let pids = pids
                    .iter()
                    .copied()
                    .map(sysinfo::Pid::from_u32)
                    .collect::<Vec<_>>();
                self.system.refresh_processes_specifics(
                    sysinfo::ProcessesToUpdate::Some(&pids),
                    remove_dead_processes,
                    refresh_kind,
                )
            }
            None => self.system.refresh_processes_specifics(
                sysinfo::ProcessesToUpdate::All,
                remove_dead_processes,
                refresh_kind,
            ),
        }
    }

    pub(crate) fn refresh_disks(&mut self) {
        self.disks.refresh(true);
    }

    pub(crate) fn refresh_disks_specifics(
        &mut self,
        remove_not_listed_disks: bool,
        kind: bool,
        storage: bool,
        io_usage: bool,
    ) {
        let mut refresh_kind = sysinfo::DiskRefreshKind::nothing();
        if kind {
            refresh_kind = refresh_kind.with_kind();
        }
        if storage {
            refresh_kind = refresh_kind.with_storage();
        }
        if io_usage {
            refresh_kind = refresh_kind.with_io_usage();
        }
        self.disks
            .refresh_specifics(remove_not_listed_disks, refresh_kind);
    }

    pub(crate) fn refresh_networks(&mut self) {
        self.networks.refresh(true);
    }

    pub(crate) fn refresh_components(&mut self) {
        self.components.refresh(true);
    }

    pub(crate) fn refresh_users(&mut self) {
        self.users.refresh();
    }

    pub(crate) fn refresh_groups(&mut self) {
        self.groups.refresh();
    }

    pub(crate) fn cpus(&self) -> Vec<PyCpu> {
        self.system.cpus().iter().map(PyCpu::from).collect()
    }

    pub(crate) fn disks(&self) -> Vec<PyDisk> {
        let mut disks = self
            .disks
            .list()
            .iter()
            .map(PyDisk::from)
            .collect::<Vec<_>>();
        disks.sort_by(|left, right| {
            left.mount_point
                .cmp(&right.mount_point)
                .then_with(|| left.name.cmp(&right.name))
        });
        disks
    }

    pub(crate) fn networks(&self) -> Vec<PyNetwork> {
        let mut networks = self
            .networks
            .iter()
            .map(|(interface, network)| PyNetwork::from_network(interface, network))
            .collect::<Vec<_>>();
        networks.sort_by(|left, right| left.interface.cmp(&right.interface));
        networks
    }

    pub(crate) fn components(&self) -> Vec<PyComponent> {
        let mut components = self
            .components
            .list()
            .iter()
            .map(PyComponent::from)
            .collect::<Vec<_>>();
        components.sort_by(|left, right| left.label.cmp(&right.label));
        components
    }

    pub(crate) fn users(&self) -> Vec<PyUser> {
        let mut users = self
            .users
            .list()
            .iter()
            .map(PyUser::from)
            .collect::<Vec<_>>();
        users.sort_by(|left, right| left.name.cmp(&right.name));
        users
    }

    pub(crate) fn groups(&self) -> Vec<PyGroup> {
        let mut groups = self
            .groups
            .list()
            .iter()
            .map(PyGroup::from)
            .collect::<Vec<_>>();
        groups.sort_by(|left, right| {
            left.name
                .cmp(&right.name)
                .then_with(|| left.id.cmp(&right.id))
        });
        groups
    }

    pub(crate) fn get_user_by_id(&self, user_id: &str) -> Option<PyUser> {
        self.users
            .list()
            .iter()
            .find(|user| user.id().to_string() == user_id)
            .map(PyUser::from)
    }

    pub(crate) fn processes(&self, shared: SharedState) -> Vec<PyProcess> {
        let mut processes = self
            .system
            .processes()
            .values()
            .map(|process| PyProcess::from_process(process, shared.clone()))
            .collect::<Vec<_>>();
        processes.sort_by_key(|process| process.pid);
        processes
    }

    pub(crate) fn process(&self, pid: u32, shared: SharedState) -> Option<PyProcess> {
        self.system
            .process(sysinfo::Pid::from_u32(pid))
            .map(|process| PyProcess::from_process(process, shared))
    }

    pub(crate) fn processes_by_name(&self, name: &str, shared: SharedState) -> Vec<PyProcess> {
        let mut processes = self
            .system
            .processes_by_name(OsStr::new(name))
            .map(|process| PyProcess::from_process(process, shared.clone()))
            .collect::<Vec<_>>();
        processes.sort_by_key(|process| process.pid);
        processes
    }

    pub(crate) fn processes_by_exact_name(
        &self,
        name: &str,
        shared: SharedState,
    ) -> Vec<PyProcess> {
        let mut processes = self
            .system
            .processes_by_exact_name(OsStr::new(name))
            .map(|process| PyProcess::from_process(process, shared.clone()))
            .collect::<Vec<_>>();
        processes.sort_by_key(|process| process.pid);
        processes
    }

    pub(crate) fn load_average(&self) -> PyLoadAverage {
        let load_average = sysinfo::System::load_average();
        PyLoadAverage {
            one: load_average.one,
            five: load_average.five,
            fifteen: load_average.fifteen,
        }
    }

    /// Build the deterministic, serialization-friendly snapshot used by `Sysinfo.to_dict()`.
    pub(crate) fn snapshot(&self, shared: SharedState) -> SystemSnapshot {
        SystemSnapshot {
            total_memory: self.system.total_memory(),
            free_memory: self.system.free_memory(),
            available_memory: self.system.available_memory(),
            used_memory: self.system.used_memory(),
            total_swap: self.system.total_swap(),
            free_swap: self.system.free_swap(),
            used_swap: self.system.used_swap(),
            cgroup_limits: self.system.cgroup_limits().map(PyCGroupLimits::from),
            uptime: sysinfo::System::uptime(),
            boot_time: sysinfo::System::boot_time(),
            load_average: self.load_average(),
            name: sysinfo::System::name(),
            kernel_version: sysinfo::System::kernel_version(),
            kernel_long_version: sysinfo::System::kernel_long_version(),
            os_version: sysinfo::System::os_version(),
            long_os_version: sysinfo::System::long_os_version(),
            distribution_id: sysinfo::System::distribution_id(),
            distribution_id_like: sysinfo::System::distribution_id_like(),
            host_name: sysinfo::System::host_name(),
            cpu_arch: sysinfo::System::cpu_arch(),
            physical_core_count: sysinfo::System::physical_core_count(),
            global_cpu_usage: self.system.global_cpu_usage(),
            open_files_limit: sysinfo::System::open_files_limit(),
            motherboard: PyMotherboard::collect(),
            product: PyProduct::collect(),
            cpus: self.cpus(),
            disks: self.disks(),
            networks: self.networks(),
            components: self.components(),
            users: self.users(),
            groups: self.groups(),
            processes: self.processes(shared),
        }
    }

    /// Return just the collection counts used by `Sysinfo.__repr__`.
    pub(crate) fn collection_counts(&self) -> CollectionCounts {
        CollectionCounts {
            cpus: self.system.cpus().len(),
            disks: self.disks.list().len(),
            networks: self.networks.len(),
            components: self.components.list().len(),
            users: self.users.list().len(),
            groups: self.groups.list().len(),
            processes: self.system.processes().len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ProcessRefreshOptions, SystemState};

    #[test]
    fn constructor_invariants_hold() {
        let state = SystemState::new();
        let snapshot = state.snapshot(super::new_shared_state(SystemState::new()));

        assert!(snapshot.used_swap <= snapshot.total_swap);
        assert!(snapshot.available_memory <= snapshot.total_memory);
        assert_eq!(snapshot.cpus.len(), state.system.cpus().len());
    }

    #[test]
    fn used_swap_snapshot_matches_sysinfo() {
        let state = SystemState::new();
        let snapshot = state.snapshot(super::new_shared_state(SystemState::new()));

        assert_eq!(snapshot.used_swap, state.system.used_swap());
    }

    #[test]
    fn process_refresh_defaults_match_sysinfo() {
        let options = ProcessRefreshOptions::default();

        assert!(!options.cpu);
        assert!(!options.memory);
        assert!(options.tasks);
        assert_eq!(options.user, sysinfo::UpdateKind::Never);
    }

    #[test]
    fn collections_have_deterministic_ordering() {
        let state = SystemState::new();
        let snapshot = state.snapshot(super::new_shared_state(SystemState::new()));

        assert!(
            snapshot
                .networks
                .windows(2)
                .all(|window| window[0].interface <= window[1].interface)
        );
        assert!(
            snapshot
                .users
                .windows(2)
                .all(|window| window[0].name <= window[1].name)
        );
        assert!(
            snapshot
                .groups
                .windows(2)
                .all(|window| window[0].name <= window[1].name)
        );
        assert!(
            snapshot
                .components
                .windows(2)
                .all(|window| window[0].label <= window[1].label)
        );
        assert!(
            snapshot
                .processes
                .windows(2)
                .all(|window| window[0].pid <= window[1].pid)
        );
        assert!(snapshot.disks.windows(2).all(|window| {
            window[0].mount_point < window[1].mount_point
                || (window[0].mount_point == window[1].mount_point
                    && window[0].name <= window[1].name)
        }));
    }
}
