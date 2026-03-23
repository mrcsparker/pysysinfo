# pysysinfo

`pysysinfo` is a Python-first wrapper around the Rust
[`sysinfo`](https://github.com/GuillaumeGomez/sysinfo) crate.
It focuses on a clean Python API, immutable snapshot objects, modern packaging
with `uv` + `maturin`, and full access to the useful parts of `sysinfo`
without feeling like a raw port of the Rust API.

## Highlights

- A single `System` entry point with immediate, usable data
- Immutable snapshot objects such as `Cpu`, `Disk`, `Network`, `User`, and `Process`
- Live process-control helpers backed by the owning `System`
- Friendly serialization helpers with `to_dict()` and `to_json()`
- Fine-grained refresh controls for memory, CPU, processes, disks, and more
- Typed Python package layout with `py.typed` and a full stub file

## Quick Start

```python
from __future__ import annotations

import time

from pysysinfo import MINIMUM_CPU_UPDATE_INTERVAL, System, get_current_pid

system = System()

print(system)
print(f"Host: {system.host_name}")
print(f"OS: {system.long_os_version}")
print(f"CPU architecture: {system.cpu_arch}")
print(f"Current PID: {get_current_pid()}")

current_process = system.process(get_current_pid())
if current_process is not None:
    print(f"Current executable: {current_process.exe}")
    print(f"Current process memory: {current_process.memory} bytes")

# CPU usage is diff-based, so refresh twice with a pause.
system.refresh_cpu_usage()
time.sleep(MINIMUM_CPU_UPDATE_INTERVAL)
system.refresh_cpu_usage()

print(f"Global CPU usage: {system.global_cpu_usage:.1f}%")
print(f"Load average: {system.load_average.to_dict()}")
print(f"Serialized system snapshot size: {len(system.to_json())} bytes")
```

## Documentation

- API reference: [docs/api.md](docs/api.md)
- Examples: [examples/](examples)

The examples directory is organized so the scripts collectively exercise the
full public API surface, including refresh controls, snapshot traversal,
serialization, and process lifecycle helpers.

## Public API

```python
from pysysinfo import (
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
    System,
    User,
    get_current_pid,
    set_open_files_limit,
)
```

`System` exposes:

- Snapshot properties: `cpus`, `disks`, `networks`, `components`, `users`, `groups`, `processes`
- Memory properties: `total_memory`, `free_memory`, `available_memory`, `used_memory`, `total_swap`, `free_swap`, `used_swap`, `cgroup_limits`
- System metadata: `uptime`, `boot_time`, `load_average`, `name`, `kernel_version`, `kernel_long_version`, `os_version`, `long_os_version`, `distribution_id`, `distribution_id_like`, `host_name`, `cpu_arch`, `physical_core_count`, `global_cpu_usage`, `open_files_limit`, `motherboard`, `product`
- Refresh methods: `refresh_all`, `refresh_memory`, `refresh_memory_specifics`, `refresh_cpu`, `refresh_cpu_usage`, `refresh_cpu_frequency`, `refresh_cpu_all`, `refresh_cpu_specifics`, `refresh_cpu_list`, `refresh_processes`, `refresh_processes_specifics`, `refresh_specifics`, `refresh_disks`, `refresh_disks_specifics`, `refresh_networks`, `refresh_components`, `refresh_users`, `refresh_groups`
- Lookup helpers: `process(pid)`, `processes_by_name(name)`, `processes_by_exact_name(name)`, `get_user_by_id(user_id)`
- Serialization helpers: `to_dict()`, `to_json()`

The package also exports:

- Constants: `IS_SUPPORTED_SYSTEM`, `SUPPORTED_SIGNALS`, `MINIMUM_CPU_UPDATE_INTERVAL`
- Functions: `get_current_pid()`, `set_open_files_limit(limit)`

Accepted refresh-update strings for process-specific refresh controls are:

- `"never"`
- `"always"`
- `"only_if_not_set"`

## Examples

- [`examples/simple.py`](examples/simple.py): small overview of the package and module-level helpers
- [`examples/list_cpu_data.py`](examples/list_cpu_data.py): CPU refresh semantics and usage reporting
- [`examples/list_resources.py`](examples/list_resources.py): every snapshot collection and nested object type
- [`examples/sysinfo.py`](examples/sysinfo.py): advanced refresh controls, metadata, lookups, and serialization
- [`examples/process_control.py`](examples/process_control.py): safe demonstrations of `Process.kill*()` and `Process.wait()`

## Development

```bash
uv sync --group dev
uv run maturin develop
uv run cargo test --all-targets
uv run pytest
uv run ruff check .
cargo clippy --all-targets -- -D warnings
```

`uv` owns the Python environment and lockfile.
`maturin` builds and installs the native extension into the mixed Python
package under `python/pysysinfo`.

## Migration From `Sysinfo`

This release intentionally breaks the old API.

- `pysysinfo.Sysinfo` was replaced with `pysysinfo.System`
- `system.cpus()` became `system.cpus`
- `system.disks()` became `system.disks`
- `system.networks()` became `system.networks`
- `system.components()` became `system.components`
- `system.users()` became `system.users`
- There is no compatibility alias for `Sysinfo`
