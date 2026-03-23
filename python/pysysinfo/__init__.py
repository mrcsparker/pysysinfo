"""Python-first bindings for the Rust `sysinfo` crate.

The public API centers on :class:`Sysinfo`, which keeps a modern, Pythonic
facade while still covering the broader `sysinfo` surface:

- immutable snapshots for CPUs, disks, networks, users, groups, processes, and sensors
- explicit refresh controls for memory, CPU, disks, and processes
- live process control helpers such as ``kill()``, ``wait()``, and ``kill_with()``
- system metadata helpers like product, motherboard, cgroup limits, and open-file limits
"""

from __future__ import annotations

from importlib.metadata import PackageNotFoundError, version

try:
    from ._core import (
        IS_SUPPORTED_SYSTEM,
        MINIMUM_CPU_UPDATE_INTERVAL,
        SUPPORTED_SIGNALS,
        CGroupLimits,
        Component,
        Cpu,
        Disk,
        DiskUsage,
        ExitStatus,
        Group,
        IpNetwork,
        LoadAverage,
        MacAddress,
        Motherboard,
        Network,
        Process,
        Product,
        Sysinfo,
        User,
        get_current_pid,
        set_open_files_limit,
    )
except ImportError as exc:  # pragma: no cover - exercised during local setup failures
    raise ImportError(
        "pysysinfo could not import its native extension. Run `uv run maturin develop` "
        "from the repository root or install a built wheel."
    ) from exc

try:
    __version__ = version("pysysinfo")
except PackageNotFoundError:  # pragma: no cover - editable dev installs without metadata are rare
    __version__ = "0.0.0"

System = Sysinfo

__all__ = [
    "CGroupLimits",
    "Component",
    "Cpu",
    "Disk",
    "DiskUsage",
    "ExitStatus",
    "Group",
    "IS_SUPPORTED_SYSTEM",
    "IpNetwork",
    "LoadAverage",
    "MINIMUM_CPU_UPDATE_INTERVAL",
    "MacAddress",
    "Motherboard",
    "Network",
    "Process",
    "Product",
    "SUPPORTED_SIGNALS",
    "Sysinfo",
    "System",
    "User",
    "__version__",
    "get_current_pid",
    "set_open_files_limit",
]
