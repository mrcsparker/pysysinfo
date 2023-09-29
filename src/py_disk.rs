use pyo3::prelude::*;
use serde::Serialize;
use sysinfo::{DiskExt, DiskKind};

#[derive(Debug, Serialize)]
#[pyclass(name = "Disk")]
pub struct PyDisk {
    /// Returns the kind of disk.
    #[pyo3(get)]
    pub kind: String,

    /// Returns the disk name.
    #[pyo3(get)]
    pub name: String,

    /// Returns the file system used on this disk (so for example: `EXT4`, `NTFS`, etc...).
    #[pyo3(get)]
    pub file_system: String,

    /// Returns the mount point of the disk (`/` for example).
    #[pyo3(get)]
    pub mount_point: String,

    /// Returns the total disk size, in bytes.
    #[pyo3(get)]
    pub total_space: u64,

    /// Returns the available disk size, in bytes.
    #[pyo3(get)]
    pub available_space: u64,

    /// Returns `true` if the disk is removable.
    #[pyo3(get)]
    pub is_removable: bool,
}

impl From<&sysinfo::Disk> for PyDisk {
    fn from(disk: &sysinfo::Disk) -> Self {
        let kind = match disk.kind() {
            DiskKind::HDD => "HDD".to_string(),
            DiskKind::SSD => "SSD".to_string(),
            DiskKind::Unknown(_) => "Unknown".to_string(),
        };

        Self {
            kind,
            name: disk.name().to_str().unwrap_or("").to_string(),
            file_system: std::str::from_utf8(disk.file_system())
                .unwrap_or("")
                .to_string(),
            mount_point: disk.mount_point().to_str().unwrap_or("").to_string(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            is_removable: disk.is_removable(),
        }
    }
}

common_methods!(PyDisk);
