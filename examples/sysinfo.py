from __future__ import annotations

import json
from pprint import pprint

from pysysinfo import System, get_current_pid, set_open_files_limit


def main() -> None:
    print("== construction ==")
    full = System()
    empty = System.new_empty()
    selective = System.new_with_specifics(
        memory=True,
        cpu=True,
        cpu_usage=True,
        cpu_frequency=True,
        processes=True,
        process_cpu=True,
        process_disk_usage=True,
        process_memory=True,
        process_user="always",
        process_cwd="always",
        process_root="always",
        process_environ="never",
        process_cmd="always",
        process_exe="always",
    )

    print(full)
    print(empty)
    print(selective)

    print("\n== module helper ==")
    print(f"set_open_files_limit result: {set_open_files_limit(256)}")

    print("\n== metadata ==")
    print(f"uptime: {full.uptime}")
    print(f"boot_time: {full.boot_time}")
    print(f"name: {full.name}")
    print(f"kernel_version: {full.kernel_version}")
    print(f"kernel_long_version: {full.kernel_long_version}")
    print(f"os_version: {full.os_version}")
    print(f"long_os_version: {full.long_os_version}")
    print(f"distribution_id: {full.distribution_id}")
    print(f"distribution_id_like: {full.distribution_id_like}")
    print(f"host_name: {full.host_name}")
    print(f"cpu_arch: {full.cpu_arch}")
    print(f"physical_core_count: {full.physical_core_count}")
    print(f"open_files_limit: {full.open_files_limit}")
    print(f"motherboard: {full.motherboard}")
    print(f"product: {full.product}")
    print(f"cgroup_limits: {full.cgroup_limits}")

    print("\n== targeted refreshes ==")
    full.refresh_memory()
    full.refresh_memory_specifics(ram=True, swap=False)
    full.refresh_disks()
    full.refresh_disks_specifics(kind=True, storage=True, io_usage=True)
    full.refresh_networks()
    full.refresh_components()
    full.refresh_users()
    full.refresh_groups()
    updated = full.refresh_processes(remove_dead_processes=True)
    print(f"default process refresh updated {updated} processes")

    updated = full.refresh_processes_specifics(
        [get_current_pid()],
        True,
        cpu=True,
        disk_usage=True,
        memory=True,
        user="always",
        cwd="always",
        root="always",
        environ="never",
        cmd="always",
        exe="always",
        tasks=True,
    )
    print(f"targeted process refresh updated {updated} process")

    updated = full.refresh_specifics(
        memory=True,
        ram=True,
        swap=True,
        cpu=True,
        cpu_usage=True,
        cpu_frequency=True,
        processes=True,
        process_ids=[get_current_pid()],
        remove_dead_processes=True,
        process_cpu=True,
        process_disk_usage=True,
        process_memory=True,
        process_user="always",
        process_cwd="always",
        process_root="always",
        process_environ="never",
        process_cmd="always",
        process_exe="always",
        process_tasks=True,
    )
    print(f"combined refresh updated {updated} process")

    print("\n== process lookups ==")
    current = full.process(get_current_pid())
    if current is not None:
        print(f"pid: {current.pid}")
        print(f"name: {current.name}")
        print(f"status: {current.status}")
        print(f"cmd: {current.cmd}")
        print(f"exe: {current.exe}")
        print(f"cwd: {current.cwd}")
        print(f"root: {current.root}")
        print(f"user_id: {current.user_id}")
        print(f"group_id: {current.group_id}")
        print(f"effective_user_id: {current.effective_user_id}")
        print(f"effective_group_id: {current.effective_group_id}")
        print(f"session_id: {current.session_id}")
        print(f"thread_kind: {current.thread_kind}")
        print(f"exists: {current.exists}")
        print(f"open_files: {current.open_files}")
        print(f"open_files_limit: {current.open_files_limit}")
        print(f"tasks: {current.tasks}")
        print(f"process disk usage: {current.disk_usage}")
        print(f"process json size: {len(current.to_json())} bytes")

    matches = full.processes_by_name("python")
    print(f"processes_by_name('python'): {len(matches)}")
    exact_matches = full.processes_by_exact_name("python")
    print(f"processes_by_exact_name('python'): {len(exact_matches)}")

    print("\n== user lookup ==")
    if current is not None and current.user_id is not None:
        user = full.get_user_by_id(current.user_id)
        if user is not None:
            print(f"user: {user.name} ({user.id})")
            print(f"group count: {len(user.groups)}")

    print("\n== serialization ==")
    pprint(full.load_average.to_dict())
    pprint(full.product.to_dict())
    payload = full.to_dict()
    print(f"top-level keys: {sorted(payload)}")
    print(f"json payload length: {len(json.dumps(payload))}")


if __name__ == "__main__":
    main()
