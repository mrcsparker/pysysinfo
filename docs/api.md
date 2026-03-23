# API Reference

This document covers the public Python API exported by `pysysinfo`.
For runnable examples, see the scripts in [`examples/`](../examples).

## Top-Level Exports

### Classes

- `System`
- `Cpu`
- `Disk`
- `DiskUsage`
- `Network`
- `MacAddress`
- `IpNetwork`
- `Component`
- `User`
- `Group`
- `Process`
- `ExitStatus`
- `LoadAverage`
- `CGroupLimits`
- `Motherboard`
- `Product`

### Functions

- `get_current_pid() -> int`
- `set_open_files_limit(limit: int) -> bool`

### Constants

- `IS_SUPPORTED_SYSTEM: bool`
- `SUPPORTED_SIGNALS: tuple[str, ...]`
- `MINIMUM_CPU_UPDATE_INTERVAL: float`
- `__version__: str`

## System

`System` is the main entry point.
It owns the live `sysinfo` collectors and produces immutable Python snapshot
objects from that state.

### Construction

- `System()`
  Creates a fully initialized system snapshot using the same broad defaults as
  `sysinfo::System::new_all()`.

- `System.new_empty()`
  Creates a `System` with no initial data loaded.
  This is useful when you want tight control over the initial refresh cost.

- `System.new_with_specifics(...)`
  Creates an empty system and immediately applies a selective refresh.
  The refresh parameters mirror the Python `refresh_specifics(...)` method.

### Snapshot Properties

- `system.cpus -> tuple[Cpu, ...]`
- `system.disks -> tuple[Disk, ...]`
- `system.networks -> tuple[Network, ...]`
- `system.components -> tuple[Component, ...]`
- `system.users -> tuple[User, ...]`
- `system.groups -> tuple[Group, ...]`
- `system.processes -> tuple[Process, ...]`

These are immutable tuples, and each element is an immutable snapshot object.

### Memory Properties

- `system.total_memory -> int`
- `system.free_memory -> int`
- `system.available_memory -> int`
- `system.used_memory -> int`
- `system.total_swap -> int`
- `system.free_swap -> int`
- `system.used_swap -> int`
- `system.cgroup_limits -> CGroupLimits | None`

`cgroup_limits` is only meaningful on platforms where `sysinfo` can report it,
and may be `None`.

### System Metadata

- `system.uptime -> int`
- `system.boot_time -> int`
- `system.load_average -> LoadAverage`
- `system.name -> str | None`
- `system.kernel_version -> str | None`
- `system.kernel_long_version -> str`
- `system.os_version -> str | None`
- `system.long_os_version -> str | None`
- `system.distribution_id -> str`
- `system.distribution_id_like -> tuple[str, ...]`
- `system.host_name -> str | None`
- `system.cpu_arch -> str`
- `system.physical_core_count -> int | None`
- `system.global_cpu_usage -> float`
- `system.open_files_limit -> int | None`
- `system.motherboard -> Motherboard | None`
- `system.product -> Product`
- `system.supported_signals -> tuple[str, ...]`
- `system.is_supported_system -> bool`
- `system.minimum_cpu_update_interval -> float`

### Refresh Methods

#### Broad Refresh

- `system.refresh_all()`
- `system.refresh_specifics(...) -> int`

`refresh_specifics(...)` combines memory, CPU, and process refresh controls in a
single call. It returns the number of updated processes when process refreshing
is enabled, or `0` otherwise.

#### Memory Refresh

- `system.refresh_memory()`
- `system.refresh_memory_specifics(*, ram: bool = True, swap: bool = True)`

#### CPU Refresh

- `system.refresh_cpu()`
- `system.refresh_cpu_usage()`
- `system.refresh_cpu_frequency()`
- `system.refresh_cpu_all()`
- `system.refresh_cpu_specifics(*, cpu_usage: bool = True, frequency: bool = True)`
- `system.refresh_cpu_list(*, cpu_usage: bool = True, frequency: bool = True)`

Important:

- CPU usage is diff-based.
- To get meaningful `cpu_usage` and `global_cpu_usage` values, refresh twice.
- Wait at least `MINIMUM_CPU_UPDATE_INTERVAL` seconds between those refreshes.

#### Process Refresh

- `system.refresh_processes(pids: list[int] | None = None, remove_dead_processes: bool = True) -> int`
- `system.refresh_processes_specifics(...) -> int`

`refresh_processes_specifics(...)` supports:

- `cpu`
- `disk_usage`
- `memory`
- `user`
- `cwd`
- `root`
- `environ`
- `cmd`
- `exe`
- `tasks`

The string-valued options accept:

- `"never"`
- `"always"`
- `"only_if_not_set"`

#### Collector Refresh

- `system.refresh_disks()`
- `system.refresh_disks_specifics(remove_not_listed_disks: bool = True, *, kind: bool = True, storage: bool = True, io_usage: bool = True)`
- `system.refresh_networks()`
- `system.refresh_components()`
- `system.refresh_users()`
- `system.refresh_groups()`

### Lookup Methods

- `system.process(pid: int) -> Process | None`
- `system.processes_by_name(name: str) -> tuple[Process, ...]`
- `system.processes_by_exact_name(name: str) -> tuple[Process, ...]`
- `system.get_user_by_id(user_id: str) -> User | None`

### Serialization

- `system.to_dict() -> dict[str, Any]`
- `system.to_json() -> str`

These return a whole-system snapshot with nested plain Python data.

## Cpu

Properties:

- `cpu.cpu_usage`
- `cpu.name`
- `cpu.vendor_id`
- `cpu.brand`
- `cpu.frequency`

Methods:

- `cpu.to_dict()`
- `cpu.to_json()`

## Disk and DiskUsage

### Disk

Properties:

- `disk.kind`
- `disk.name`
- `disk.file_system`
- `disk.mount_point`
- `disk.total_space`
- `disk.available_space`
- `disk.is_removable`
- `disk.is_read_only`
- `disk.usage -> DiskUsage`

Methods:

- `disk.to_dict()`
- `disk.to_json()`

### DiskUsage

Properties:

- `usage.total_written_bytes`
- `usage.written_bytes`
- `usage.total_read_bytes`
- `usage.read_bytes`

Methods:

- `usage.to_dict()`
- `usage.to_json()`

## Network, MacAddress, and IpNetwork

### Network

Properties:

- `network.interface`
- `network.received`
- `network.total_received`
- `network.transmitted`
- `network.total_transmitted`
- `network.packets_received`
- `network.total_packets_received`
- `network.packets_transmitted`
- `network.total_packets_transmitted`
- `network.errors_on_received`
- `network.total_errors_on_received`
- `network.errors_on_transmitted`
- `network.total_errors_on_transmitted`
- `network.mac_address -> MacAddress`
- `network.ip_networks -> tuple[IpNetwork, ...]`
- `network.mtu`

Methods:

- `network.to_dict()`
- `network.to_json()`

### MacAddress

Properties:

- `mac.value`
- `mac.is_unspecified`

Methods:

- `mac.to_dict()`
- `mac.to_json()`

### IpNetwork

Properties:

- `ip_network.addr`
- `ip_network.prefix`

Methods:

- `ip_network.to_dict()`
- `ip_network.to_json()`

## Component

Properties:

- `component.temperature -> float | None`
- `component.max -> float | None`
- `component.critical -> float | None`
- `component.label -> str`
- `component.id -> str | None`

Methods:

- `component.to_dict()`
- `component.to_json()`

## User and Group

### User

Properties:

- `user.id`
- `user.group_id`
- `user.name`
- `user.groups -> tuple[Group, ...]`

Methods:

- `user.to_dict()`
- `user.to_json()`

### Group

Properties:

- `group.id`
- `group.name`

Methods:

- `group.to_dict()`
- `group.to_json()`

## Process and ExitStatus

`Process` snapshots are immutable, but their lifecycle methods remain live.
They operate against the owning `System` state.

If the process disappears or the PID has been reused for a different process,
control methods raise a lookup-style Python exception instead of silently acting
on the wrong process.

### Process Properties

- `process.pid`
- `process.name`
- `process.cmd`
- `process.exe`
- `process.environ`
- `process.cwd`
- `process.root`
- `process.memory`
- `process.virtual_memory`
- `process.parent`
- `process.status`
- `process.start_time`
- `process.run_time`
- `process.cpu_usage`
- `process.accumulated_cpu_time`
- `process.disk_usage -> DiskUsage`
- `process.user_id`
- `process.effective_user_id`
- `process.group_id`
- `process.effective_group_id`
- `process.session_id`
- `process.tasks -> tuple[int, ...] | None`
- `process.thread_kind -> str | None`
- `process.exists`
- `process.open_files`
- `process.open_files_limit`

### Process Methods

- `process.kill() -> bool`
- `process.kill_with(signal: str) -> bool | None`
- `process.kill_and_wait() -> ExitStatus | None`
- `process.kill_with_and_wait(signal: str) -> ExitStatus | None`
- `process.wait() -> ExitStatus | None`
- `process.to_dict()`
- `process.to_json()`

Accepted signal names are lowercase identifiers such as:

- `"kill"`
- `"term"`
- `"sigterm"`
- `"interrupt"`
- `"floating_point_exception"`

Use `SUPPORTED_SIGNALS` or `system.supported_signals` to discover what the
current platform supports.

### ExitStatus

Properties:

- `status.code`
- `status.success`
- `status.unix_signal`

Methods:

- `status.to_dict()`
- `status.to_json()`

## LoadAverage

Properties:

- `load.one`
- `load.five`
- `load.fifteen`

Methods:

- `load.to_dict()`
- `load.to_json()`

## CGroupLimits

Properties:

- `limits.total_memory`
- `limits.free_memory`
- `limits.free_swap`
- `limits.rss`

Methods:

- `limits.to_dict()`
- `limits.to_json()`

## Motherboard

Properties:

- `motherboard.name`
- `motherboard.vendor_name`
- `motherboard.version`
- `motherboard.serial_number`
- `motherboard.asset_tag`

Methods:

- `motherboard.to_dict()`
- `motherboard.to_json()`

## Product

Properties:

- `product.name`
- `product.family`
- `product.serial_number`
- `product.stock_keeping_unit`
- `product.uuid`
- `product.version`
- `product.vendor_name`

Methods:

- `product.to_dict()`
- `product.to_json()`

## Notes

- Snapshot collections are returned in deterministic order for stable tests and
  readable output.
- The module constants mirror the corresponding `sysinfo` values at import time.
- `set_open_files_limit(...)` only has effect on Linux where `sysinfo` exposes
  that tuning hook.
