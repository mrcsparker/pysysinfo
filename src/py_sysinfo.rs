use crate::py_component::PyComponent;
use crate::py_cpu::PyCpu;
use crate::py_disk::PyDisk;
use crate::py_load_avg::PyLoadAvg;
use crate::py_network::PyNetwork;
use crate::py_user::PyUser;
use pyo3::prelude::*;
use rayon::prelude::*;
use sysinfo::{NetworkExt, NetworksExt, System, SystemExt};

#[pyclass(name = "Sysinfo")]
pub struct PySysinfo {
    sys: System,
}

#[pymethods]
impl PySysinfo {
    /// Creates a new [`System`] instance with everything loaded.
    #[new]
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
        }
    }

    /// Refreshes all system, processes, disks and network interfaces information.
    pub fn refresh_all(&mut self) {
        self.sys.refresh_all()
    }

    /// Refreshes system information (RAM, swap, CPU usage and components' temperature).
    pub fn refresh_system(&mut self) {
        self.sys.refresh_system()
    }

    /// Refreshes RAM and SWAP usage.
    pub fn refresh_memory(&mut self) {
        self.sys.refresh_memory()
    }

    /// Refreshes CPUs information.
    pub fn refresh_cpu(&mut self) {
        self.sys.refresh_cpu()
    }

    /// Refreshes components' temperature.
    pub fn refresh_components(&mut self) {
        self.sys.refresh_components()
    }

    /// Refreshes components list.
    pub fn refresh_components_list(&mut self) {
        self.sys.refresh_components_list()
    }

    /// Gets all processes and updates their information.
    pub fn refresh_processes(&mut self) {
        self.sys.refresh_processes()
    }

    /// Refreshes the listed disks' information.
    pub fn refresh_disks(&mut self) {
        self.sys.refresh_disks()
    }

    /// The disk list will be emptied then completely recomputed.
    pub fn refresh_disks_list(&mut self) {
        self.sys.refresh_disks_list()
    }

    /// Refreshes users list.
    pub fn refresh_users_list(&mut self) {
        self.sys.refresh_users_list()
    }

    /// Refreshes networks data.
    pub fn refresh_networks(&mut self) {
        self.sys.refresh_networks()
    }

    /// The network list will be updated: removing not existing anymore interfaces and adding new
    /// ones.
    pub fn refresh_networks_list(&mut self) {
        self.sys.refresh_networks_list()
    }

    // Returns the process list.
    // fn processes(&self) -> &HashMap<Pid, Process>;

    // Returns the process corresponding to the given `pid` or `None` if no such process exists.
    // fn process(&self, pid: Pid) -> Option<&Process>;

    // Returns an iterator of process containing the given `name`.
    // fn processes_by_name<'a: 'b, 'b>(

    // Returns an iterator of processes with exactly the given `name`.
    // fn processes_by_exact_name<'a: 'b, 'b>(

    // Returns "global" CPUs information (aka the addition of all the CPUs).
    // fn global_cpu_info(&self) -> &Cpu;

    /// Returns the list of the CPUs.
    pub fn cpus(&mut self) -> Vec<PyCpu> {
        self.sys.cpus().par_iter().map(Into::into).collect()
    }

    // Returns the number of physical cores on the CPU or `None` if it couldn't get it.
    #[getter]
    pub fn physical_core_count(&self) -> Option<usize> {
        self.sys.physical_core_count()
    }

    /// Returns the RAM size in bytes.
    #[getter]
    pub fn total_memory(&mut self) -> u64 {
        self.sys.total_memory()
    }

    /// Returns the amount of free RAM in bytes.
    #[getter]
    pub fn free_memory(&mut self) -> u64 {
        self.sys.free_memory()
    }

    /// Returns the amount of available RAM in bytes.
    #[getter]
    pub fn available_memory(&mut self) -> u64 {
        self.sys.available_memory()
    }

    /// Returns the amount of used RAM in bytes.
    #[getter]
    pub fn used_memory(&mut self) -> u64 {
        self.sys.used_memory()
    }

    /// Returns the SWAP size in bytes.
    #[getter]
    pub fn total_swap(&mut self) -> u64 {
        self.sys.total_swap()
    }

    /// Returns the amount of free SWAP in bytes.
    #[getter]
    pub fn free_swap(&mut self) -> u64 {
        self.sys.free_swap()
    }

    /// Returns the amount of used SWAP in bytes.
    #[getter]
    pub fn used_swap(&mut self) -> u64 {
        self.sys.free_swap()
    }

    /// Returns the components list.
    pub fn components(&self) -> Vec<PyComponent> {
        self.sys.components().par_iter().map(Into::into).collect()
    }

    /// Returns the users list.
    pub fn users(&mut self) -> Vec<PyUser> {
        self.sys.users().par_iter().map(Into::into).collect()
    }

    /// Returns the disks list.
    pub fn disks(&mut self) -> Vec<PyDisk> {
        self.sys.disks().par_iter().map(Into::into).collect()
    }

    /// Returns the network interfaces object.
    pub fn networks(&mut self) -> Vec<PyNetwork> {
        self.sys
            .networks()
            .iter()
            .map(|(interface, network)| PyNetwork {
                interface: interface.to_string(),
                received: network.received(),
                total_received: network.total_received(),
                transmitted: network.transmitted(),
                total_transmitted: network.total_transmitted(),
                packets_received: network.packets_received(),
                total_packets_received: network.total_packets_received(),
                packets_transmitted: network.packets_transmitted(),
                total_packets_transmitted: network.total_packets_transmitted(),
                errors_on_received: network.errors_on_received(),
                total_errors_on_received: network.total_errors_on_received(),
                errors_on_transmitted: network.errors_on_transmitted(),
                total_errors_on_transmitted: network.total_errors_on_transmitted(),
                mac_address: network.mac_address().to_string(),
            })
            .collect()
    }

    /// Returns system uptime (in seconds).
    #[getter]
    pub fn uptime(&mut self) -> u64 {
        self.sys.uptime()
    }

    /// Returns the time (in seconds) when the system booted since UNIX epoch.
    #[getter]
    pub fn boot_time(&mut self) -> u64 {
        self.sys.boot_time()
    }

    // Returns the system load average value.
    fn load_average(&self) -> PyLoadAvg {
        let l = self.sys.load_average();
        PyLoadAvg {
            one: l.one,
            five: l.five,
            fifteen: l.fifteen,
        }
    }

    /// Returns the system name.
    #[getter]
    pub fn name(&mut self) -> Option<String> {
        self.sys.name()
    }

    /// Returns the system's kernel version.
    #[getter]
    pub fn kernel_version(&mut self) -> Option<String> {
        self.sys.kernel_version()
    }

    /// Returns the system version (e.g. for MacOS this will return 11.1 rather than the kernel version).
    #[getter]
    pub fn os_version(&mut self) -> Option<String> {
        self.sys.os_version()
    }

    /// Returns the system long os version (e.g "MacOS 11.2 BigSur").
    #[getter]
    pub fn long_os_version(&mut self) -> Option<String> {
        self.sys.long_os_version()
    }

    /// Returns the distribution id as defined by os-release,
    #[getter]
    pub fn distribution_id(&mut self) -> String {
        self.sys.distribution_id()
    }

    /// Returns the system hostname based off DNS
    #[getter]
    pub fn host_name(&mut self) -> Option<String> {
        self.sys.host_name()
    }

    fn __repr__(&self) -> String {
        format!(
            "Sysinfo(total_memory={}, free_memory={}, available_memory={}, used_memory={}, total_swap={}, free_swap={}, used_swap={})",
            self.sys.total_memory(),
            self.sys.free_memory(),
            self.sys.available_memory(),
            self.sys.used_memory(),
            self.sys.total_swap(),
            self.sys.free_swap(),
            self.sys.used_swap()
        )
    }
}

impl Default for PySysinfo {
    fn default() -> Self {
        Self::new()
    }
}
