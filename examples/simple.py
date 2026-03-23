from __future__ import annotations

import time

from pysysinfo import MINIMUM_CPU_UPDATE_INTERVAL, System, get_current_pid


def main() -> None:
    system = System()

    print("== package constants ==")
    print(f"minimum cpu update interval: {MINIMUM_CPU_UPDATE_INTERVAL:.3f}s")
    print(f"supported system: {system.is_supported_system}")
    print(f"supported signals: {', '.join(system.supported_signals) or '(none)'}")

    print("\n== basic system metadata ==")
    print(system)
    print(f"current pid: {get_current_pid()}")
    print(f"host: {system.host_name}")
    print(f"os: {system.long_os_version}")
    print(f"kernel: {system.kernel_long_version}")
    print(f"distribution id: {system.distribution_id}")
    print(f"distribution id like: {system.distribution_id_like}")
    print(f"cpu architecture: {system.cpu_arch}")
    print(f"physical core count: {system.physical_core_count}")
    print(f"open files limit: {system.open_files_limit}")

    print("\n== memory ==")
    print(f"total memory: {system.total_memory} bytes")
    print(f"available memory: {system.available_memory} bytes")
    print(f"used swap: {system.used_swap} bytes")
    print(f"cgroup limits: {system.cgroup_limits}")

    current = system.process(get_current_pid())
    if current is not None:
        print("\n== current process ==")
        print(f"name: {current.name}")
        print(f"exe: {current.exe}")
        print(f"cwd: {current.cwd}")
        print(f"memory: {current.memory} bytes")

    print("\n== cpu refresh example ==")
    system.refresh_cpu_usage()
    time.sleep(MINIMUM_CPU_UPDATE_INTERVAL)
    system.refresh_cpu_usage()
    print(f"global cpu usage: {system.global_cpu_usage:.1f}%")
    for cpu in system.cpus[:4]:
        print(f"{cpu.name}: {cpu.cpu_usage:.1f}% @ {cpu.frequency} MHz")

    print("\n== serialization ==")
    print(system.load_average.to_dict())
    print(f"system json size: {len(system.to_json())} bytes")


if __name__ == "__main__":
    main()
