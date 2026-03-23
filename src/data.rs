//! Immutable Python snapshot objects built from the underlying `sysinfo` data.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use serde::Serialize;

use crate::pythonize::{
    repr_from_serialize, serialize_to_json, serialize_to_py_dict, tuple_from_mapped_vec,
};
use crate::sysconv::{disk_kind_to_string, os_str_to_string, path_to_string};

macro_rules! impl_snapshot_methods {
    ($name:ident, $label:literal) => {
        #[pymethods]
        impl $name {
            /// Serialize this snapshot to JSON.
            fn to_json(&self) -> PyResult<String> {
                serialize_to_json(self)
            }

            /// Convert this snapshot into a plain Python dictionary.
            fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
                serialize_to_py_dict(py, self)
            }

            /// Return a readable Python-style representation of the snapshot.
            fn __repr__(&self) -> PyResult<String> {
                repr_from_serialize($label, self)
            }
        }
    };
}

/// Immutable snapshot of a MAC address.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[pyclass(name = "MacAddress", module = "pysysinfo", frozen, skip_from_py_object)]
pub struct PyMacAddress {
    /// Human-readable MAC address.
    #[pyo3(get)]
    pub(crate) value: String,
    /// Whether the address is the all-zero unspecified address.
    #[pyo3(get)]
    pub(crate) is_unspecified: bool,
}

impl From<sysinfo::MacAddr> for PyMacAddress {
    fn from(value: sysinfo::MacAddr) -> Self {
        Self {
            value: value.to_string(),
            is_unspecified: value.is_unspecified(),
        }
    }
}

impl_snapshot_methods!(PyMacAddress, "MacAddress");

/// Immutable snapshot of an IP network attached to an interface.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[pyclass(name = "IpNetwork", module = "pysysinfo", frozen, skip_from_py_object)]
pub struct PyIpNetwork {
    /// IP address string.
    #[pyo3(get)]
    pub(crate) addr: String,
    /// CIDR prefix length.
    #[pyo3(get)]
    pub(crate) prefix: u8,
}

impl From<&sysinfo::IpNetwork> for PyIpNetwork {
    fn from(value: &sysinfo::IpNetwork) -> Self {
        Self {
            addr: value.addr.to_string(),
            prefix: value.prefix,
        }
    }
}

impl_snapshot_methods!(PyIpNetwork, "IpNetwork");

/// Read and write counters shared by process and disk snapshots.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[pyclass(name = "DiskUsage", module = "pysysinfo", frozen, skip_from_py_object)]
pub struct PyDiskUsage {
    /// Total number of written bytes.
    #[pyo3(get)]
    pub(crate) total_written_bytes: u64,
    /// Number of written bytes since the previous refresh.
    #[pyo3(get)]
    pub(crate) written_bytes: u64,
    /// Total number of read bytes.
    #[pyo3(get)]
    pub(crate) total_read_bytes: u64,
    /// Number of read bytes since the previous refresh.
    #[pyo3(get)]
    pub(crate) read_bytes: u64,
}

impl From<sysinfo::DiskUsage> for PyDiskUsage {
    fn from(value: sysinfo::DiskUsage) -> Self {
        Self {
            total_written_bytes: value.total_written_bytes,
            written_bytes: value.written_bytes,
            total_read_bytes: value.total_read_bytes,
            read_bytes: value.read_bytes,
        }
    }
}

impl_snapshot_methods!(PyDiskUsage, "DiskUsage");

/// Immutable snapshot of a logical CPU.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[pyclass(name = "Cpu", module = "pysysinfo", frozen, skip_from_py_object)]
pub struct PyCpu {
    /// Percentage of CPU activity since the previous refresh.
    #[pyo3(get)]
    pub(crate) cpu_usage: f32,
    /// Operating system label for this CPU.
    #[pyo3(get)]
    pub(crate) name: String,
    /// Vendor identifier reported by the CPU.
    #[pyo3(get)]
    pub(crate) vendor_id: String,
    /// Human-readable CPU brand string.
    #[pyo3(get)]
    pub(crate) brand: String,
    /// Current CPU frequency in MHz.
    #[pyo3(get)]
    pub(crate) frequency: u64,
}

impl From<&sysinfo::Cpu> for PyCpu {
    fn from(cpu: &sysinfo::Cpu) -> Self {
        Self {
            cpu_usage: cpu.cpu_usage(),
            name: cpu.name().to_string(),
            vendor_id: cpu.vendor_id().to_string(),
            brand: cpu.brand().to_string(),
            frequency: cpu.frequency(),
        }
    }
}

impl_snapshot_methods!(PyCpu, "Cpu");

/// Immutable snapshot of a mounted disk.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[pyclass(name = "Disk", module = "pysysinfo", frozen, skip_from_py_object)]
pub struct PyDisk {
    /// Disk kind reported by `sysinfo`.
    #[pyo3(get)]
    pub(crate) kind: String,
    /// Operating system disk name.
    #[pyo3(get)]
    pub(crate) name: String,
    /// Filesystem label, such as `apfs`, `ext4`, or `ntfs`.
    #[pyo3(get)]
    pub(crate) file_system: String,
    /// Mount point path.
    #[pyo3(get)]
    pub(crate) mount_point: String,
    /// Total disk capacity in bytes.
    #[pyo3(get)]
    pub(crate) total_space: u64,
    /// Currently available capacity in bytes.
    #[pyo3(get)]
    pub(crate) available_space: u64,
    /// Whether the disk is removable.
    #[pyo3(get)]
    pub(crate) is_removable: bool,
    /// Whether the disk is read-only.
    #[pyo3(get)]
    pub(crate) is_read_only: bool,
    usage: PyDiskUsage,
}

impl From<&sysinfo::Disk> for PyDisk {
    fn from(disk: &sysinfo::Disk) -> Self {
        Self {
            kind: disk_kind_to_string(disk.kind()),
            name: os_str_to_string(disk.name()),
            file_system: os_str_to_string(disk.file_system()),
            mount_point: path_to_string(disk.mount_point()),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            is_removable: disk.is_removable(),
            is_read_only: disk.is_read_only(),
            usage: PyDiskUsage::from(disk.usage()),
        }
    }
}

#[pymethods]
impl PyDisk {
    #[getter]
    /// Disk read and write counters.
    fn usage<'py>(&self, py: Python<'py>) -> PyResult<Py<PyDiskUsage>> {
        Py::new(py, self.usage.clone())
    }

    /// Serialize this snapshot to JSON.
    fn to_json(&self) -> PyResult<String> {
        serialize_to_json(self)
    }

    /// Convert this snapshot into a plain Python dictionary.
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        serialize_to_py_dict(py, self)
    }

    fn __repr__(&self) -> PyResult<String> {
        repr_from_serialize("Disk", self)
    }
}

/// Immutable snapshot of network interface counters.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[pyclass(name = "Network", module = "pysysinfo", frozen, skip_from_py_object)]
pub struct PyNetwork {
    /// Network interface name.
    #[pyo3(get)]
    pub(crate) interface: String,
    /// Bytes received since the previous refresh.
    #[pyo3(get)]
    pub(crate) received: u64,
    /// Total bytes received.
    #[pyo3(get)]
    pub(crate) total_received: u64,
    /// Bytes transmitted since the previous refresh.
    #[pyo3(get)]
    pub(crate) transmitted: u64,
    /// Total bytes transmitted.
    #[pyo3(get)]
    pub(crate) total_transmitted: u64,
    /// Packets received since the previous refresh.
    #[pyo3(get)]
    pub(crate) packets_received: u64,
    /// Total packets received.
    #[pyo3(get)]
    pub(crate) total_packets_received: u64,
    /// Packets transmitted since the previous refresh.
    #[pyo3(get)]
    pub(crate) packets_transmitted: u64,
    /// Total packets transmitted.
    #[pyo3(get)]
    pub(crate) total_packets_transmitted: u64,
    /// Receive errors since the previous refresh.
    #[pyo3(get)]
    pub(crate) errors_on_received: u64,
    /// Total receive errors.
    #[pyo3(get)]
    pub(crate) total_errors_on_received: u64,
    /// Transmit errors since the previous refresh.
    #[pyo3(get)]
    pub(crate) errors_on_transmitted: u64,
    /// Total transmit errors.
    #[pyo3(get)]
    pub(crate) total_errors_on_transmitted: u64,
    mac_address: PyMacAddress,
    ip_networks: Vec<PyIpNetwork>,
    /// Maximum transfer unit for the interface.
    #[pyo3(get)]
    pub(crate) mtu: u64,
}

impl PyNetwork {
    /// Build a stable Python snapshot from a live `sysinfo` network collector entry.
    pub(crate) fn from_network(interface: &str, network: &sysinfo::NetworkData) -> Self {
        Self {
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
            mac_address: PyMacAddress::from(network.mac_address()),
            ip_networks: network
                .ip_networks()
                .iter()
                .map(PyIpNetwork::from)
                .collect(),
            mtu: network.mtu(),
        }
    }
}

#[pymethods]
impl PyNetwork {
    #[getter]
    /// MAC address of the interface.
    fn mac_address<'py>(&self, py: Python<'py>) -> PyResult<Py<PyMacAddress>> {
        Py::new(py, self.mac_address.clone())
    }

    #[getter]
    /// IP networks attached to the interface.
    fn ip_networks<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        tuple_from_mapped_vec(py, self.ip_networks.clone(), |py, value| {
            Ok(Py::new(py, value)?.into_any())
        })
    }

    /// Serialize this snapshot to JSON.
    fn to_json(&self) -> PyResult<String> {
        serialize_to_json(self)
    }

    /// Convert this snapshot into a plain Python dictionary.
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        serialize_to_py_dict(py, self)
    }

    fn __repr__(&self) -> PyResult<String> {
        repr_from_serialize("Network", self)
    }
}

/// Immutable snapshot of a temperature sensor.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[pyclass(name = "Component", module = "pysysinfo", frozen, skip_from_py_object)]
pub struct PyComponent {
    /// Current temperature in Celsius, or `None` when unavailable.
    #[pyo3(get)]
    pub(crate) temperature: Option<f32>,
    /// Maximum observed temperature in Celsius, or `None` when unavailable.
    #[pyo3(get)]
    pub(crate) max: Option<f32>,
    /// Critical temperature in Celsius, or `None` when unavailable.
    #[pyo3(get)]
    pub(crate) critical: Option<f32>,
    /// Sensor label.
    #[pyo3(get)]
    pub(crate) label: String,
    /// Kernel-provided identifier, if available.
    #[pyo3(get)]
    pub(crate) id: Option<String>,
}

impl From<&sysinfo::Component> for PyComponent {
    fn from(component: &sysinfo::Component) -> Self {
        Self {
            temperature: component.temperature(),
            max: component.max(),
            critical: component.critical(),
            label: component.label().to_string(),
            id: component.id().map(str::to_string),
        }
    }
}

impl_snapshot_methods!(PyComponent, "Component");

/// Immutable snapshot of a system group.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[pyclass(name = "Group", module = "pysysinfo", frozen, skip_from_py_object)]
pub struct PyGroup {
    /// Operating system group identifier.
    #[pyo3(get)]
    pub(crate) id: String,
    /// Group name.
    #[pyo3(get)]
    pub(crate) name: String,
}

impl From<&sysinfo::Group> for PyGroup {
    fn from(group: &sysinfo::Group) -> Self {
        Self {
            id: group.id().to_string(),
            name: group.name().to_string(),
        }
    }
}

impl_snapshot_methods!(PyGroup, "Group");

/// Immutable snapshot of a system user.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[pyclass(name = "User", module = "pysysinfo", frozen, skip_from_py_object)]
pub struct PyUser {
    /// Operating system user identifier.
    #[pyo3(get)]
    pub(crate) id: String,
    /// Primary operating system group identifier.
    #[pyo3(get)]
    pub(crate) group_id: String,
    /// Username.
    #[pyo3(get)]
    pub(crate) name: String,
    groups: Vec<PyGroup>,
}

impl From<&sysinfo::User> for PyUser {
    fn from(user: &sysinfo::User) -> Self {
        let mut groups = user
            .groups()
            .into_iter()
            .map(|group| PyGroup::from(&group))
            .collect::<Vec<_>>();
        groups.sort_by(|left, right| {
            left.name
                .cmp(&right.name)
                .then_with(|| left.id.cmp(&right.id))
        });

        Self {
            id: user.id().to_string(),
            group_id: user.group_id().to_string(),
            name: user.name().to_string(),
            groups,
        }
    }
}

#[pymethods]
impl PyUser {
    #[getter]
    /// Groups the user belongs to, returned as an immutable tuple.
    fn groups<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        tuple_from_mapped_vec(py, self.groups.clone(), |py, value| {
            Ok(Py::new(py, value)?.into_any())
        })
    }

    /// Serialize this snapshot to JSON.
    fn to_json(&self) -> PyResult<String> {
        serialize_to_json(self)
    }

    /// Convert this snapshot into a plain Python dictionary.
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        serialize_to_py_dict(py, self)
    }

    fn __repr__(&self) -> PyResult<String> {
        repr_from_serialize("User", self)
    }
}

/// Memory and swap limits for the current cgroup, when available.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[pyclass(
    name = "CGroupLimits",
    module = "pysysinfo",
    frozen,
    skip_from_py_object
)]
pub struct PyCGroupLimits {
    /// Total cgroup memory in bytes.
    #[pyo3(get)]
    pub(crate) total_memory: u64,
    /// Free cgroup memory in bytes.
    #[pyo3(get)]
    pub(crate) free_memory: u64,
    /// Free cgroup swap in bytes.
    #[pyo3(get)]
    pub(crate) free_swap: u64,
    /// Resident set size in bytes.
    #[pyo3(get)]
    pub(crate) rss: u64,
}

impl From<sysinfo::CGroupLimits> for PyCGroupLimits {
    fn from(value: sysinfo::CGroupLimits) -> Self {
        Self {
            total_memory: value.total_memory,
            free_memory: value.free_memory,
            free_swap: value.free_swap,
            rss: value.rss,
        }
    }
}

impl_snapshot_methods!(PyCGroupLimits, "CGroupLimits");

/// Snapshot of motherboard metadata.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[pyclass(
    name = "Motherboard",
    module = "pysysinfo",
    frozen,
    skip_from_py_object
)]
pub struct PyMotherboard {
    /// Motherboard name or model identifier.
    #[pyo3(get)]
    pub(crate) name: Option<String>,
    /// Manufacturer name.
    #[pyo3(get)]
    pub(crate) vendor_name: Option<String>,
    /// Version or revision string.
    #[pyo3(get)]
    pub(crate) version: Option<String>,
    /// Serial number.
    #[pyo3(get)]
    pub(crate) serial_number: Option<String>,
    /// Asset tag, if reported by the platform.
    #[pyo3(get)]
    pub(crate) asset_tag: Option<String>,
}

impl PyMotherboard {
    /// Collect a one-shot motherboard snapshot when the current platform supports it.
    pub(crate) fn collect() -> Option<Self> {
        let board = sysinfo::Motherboard::new()?;
        Some(Self {
            name: board.name(),
            vendor_name: board.vendor_name(),
            version: board.version(),
            serial_number: board.serial_number(),
            asset_tag: board.asset_tag(),
        })
    }
}

impl_snapshot_methods!(PyMotherboard, "Motherboard");

/// Snapshot of product metadata.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[pyclass(name = "Product", module = "pysysinfo", frozen, skip_from_py_object)]
pub struct PyProduct {
    /// Product name.
    #[pyo3(get)]
    pub(crate) name: Option<String>,
    /// Product family identifier.
    #[pyo3(get)]
    pub(crate) family: Option<String>,
    /// Serial number.
    #[pyo3(get)]
    pub(crate) serial_number: Option<String>,
    /// Stock keeping unit.
    #[pyo3(get)]
    pub(crate) stock_keeping_unit: Option<String>,
    /// Product UUID.
    #[pyo3(get)]
    pub(crate) uuid: Option<String>,
    /// Product version or model name.
    #[pyo3(get)]
    pub(crate) version: Option<String>,
    /// Vendor name.
    #[pyo3(get)]
    pub(crate) vendor_name: Option<String>,
}

impl PyProduct {
    /// Collect a one-shot product snapshot from the current machine.
    pub(crate) fn collect() -> Self {
        Self {
            name: sysinfo::Product::name(),
            family: sysinfo::Product::family(),
            serial_number: sysinfo::Product::serial_number(),
            stock_keeping_unit: sysinfo::Product::stock_keeping_unit(),
            uuid: sysinfo::Product::uuid(),
            version: sysinfo::Product::version(),
            vendor_name: sysinfo::Product::vendor_name(),
        }
    }
}

impl_snapshot_methods!(PyProduct, "Product");

/// Immutable load-average snapshot.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[pyclass(
    name = "LoadAverage",
    module = "pysysinfo",
    frozen,
    skip_from_py_object
)]
pub struct PyLoadAverage {
    /// One-minute load average.
    #[pyo3(get)]
    pub(crate) one: f64,
    /// Five-minute load average.
    #[pyo3(get)]
    pub(crate) five: f64,
    /// Fifteen-minute load average.
    #[pyo3(get)]
    pub(crate) fifteen: f64,
}

impl_snapshot_methods!(PyLoadAverage, "LoadAverage");

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::{PyDiskUsage, PyMacAddress, PyProduct};
    use crate::pythonize::{repr_from_serialize, serialize_to_json};

    #[test]
    fn nested_snapshots_serialize_cleanly() {
        let usage = PyDiskUsage {
            total_written_bytes: 10,
            written_bytes: 2,
            total_read_bytes: 8,
            read_bytes: 1,
        };

        let value: Value = serde_json::from_str(&serialize_to_json(&usage).unwrap()).unwrap();
        assert_eq!(value["written_bytes"], 2);
        assert_eq!(value["total_read_bytes"], 8);
    }

    #[test]
    fn repr_handles_simple_value_objects() {
        let mac = PyMacAddress {
            value: "00:00:00:00:00:00".to_string(),
            is_unspecified: true,
        };

        let repr = repr_from_serialize("MacAddress", &mac).unwrap();
        assert!(repr.contains("is_unspecified=True"));
    }

    #[test]
    fn product_repr_includes_optional_fields() {
        let product = PyProduct {
            name: Some("ThinkPad".to_string()),
            family: None,
            serial_number: None,
            stock_keeping_unit: None,
            uuid: None,
            version: Some("T14".to_string()),
            vendor_name: Some("Lenovo".to_string()),
        };

        let repr = repr_from_serialize("Product", &product).unwrap();
        assert!(repr.contains("name=\"ThinkPad\""));
        assert!(repr.contains("family=None"));
    }
}
