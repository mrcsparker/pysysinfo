# pysysinfo Docs

`pysysinfo` is a Python-first wrapper around the Rust
[`sysinfo`](https://github.com/GuillaumeGomez/sysinfo) crate.

It combines a modern native-extension toolchain with a friendly Python API:

- `uv` manages the Python environment and lockfile
- `maturin` builds the extension module
- `PyO3` exposes a clean Python-facing API
- `sysinfo` powers the system data collectors and process controls

## What You Get

- a single `Sysinfo` entry point with immediately useful data
- immutable snapshot objects for system resources
- explicit refresh methods for CPU, memory, disks, networks, users, groups, and processes
- live process control helpers such as `kill()`, `kill_with()`, and `wait()`
- `to_dict()` and `to_json()` helpers across the public data types
- typed package metadata with stub files and `py.typed`

## Quick Start

```python
from __future__ import annotations

import time

from pysysinfo import MINIMUM_CPU_UPDATE_INTERVAL, Sysinfo

system = Sysinfo()
system.refresh_cpu_usage()
time.sleep(MINIMUM_CPU_UPDATE_INTERVAL)
system.refresh_cpu_usage()

print(system.host_name)
print(system.global_cpu_usage)
print(system.load_average.to_dict())
```

## Navigate the Docs

- [API Reference](api.md)
- [Changelog](changelog.md)
- [Contributing](contributing.md)
- [Security](security.md)

## Development Commands

```bash
uv sync --group dev
pre-commit install --hook-type pre-commit --hook-type pre-push
uv run maturin develop
uv run cargo test --all-targets
uv run pytest
uv run mypy tests examples
uv run mkdocs serve
```
