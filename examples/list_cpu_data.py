from __future__ import annotations

import time

from pysysinfo import MINIMUM_CPU_UPDATE_INTERVAL, System


def main() -> None:
    system = System.new_empty()

    # Build the CPU list, fetch frequency data, then prime the diff-based usage counters.
    system.refresh_cpu_list(cpu_usage=False, frequency=True)
    system.refresh_cpu_frequency()
    system.refresh_cpu_usage()
    time.sleep(MINIMUM_CPU_UPDATE_INTERVAL)
    system.refresh_cpu_usage()

    print("== initial cpu snapshot ==")
    for cpu in system.cpus:
        print(
            f"{cpu.name}: usage={cpu.cpu_usage:5.1f}% "
            f"frequency={cpu.frequency:5d} MHz "
            f"brand={cpu.brand}"
        )

    print("\n== repeated cpu refresh ==")
    for sample in range(1, 4):
        time.sleep(MINIMUM_CPU_UPDATE_INTERVAL)
        system.refresh_cpu_specifics(cpu_usage=True, frequency=False)
        print(f"sample {sample}: global={system.global_cpu_usage:.1f}%")
        for cpu in system.cpus[:4]:
            print(f"  {cpu.name}: {cpu.cpu_usage:.1f}%")

    print("\n== full cpu refresh ==")
    system.refresh_cpu_all()
    system.refresh_cpu()
    print(f"cpu count: {len(system.cpus)}")


if __name__ == "__main__":
    main()
