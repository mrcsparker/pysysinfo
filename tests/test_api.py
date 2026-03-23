from __future__ import annotations

import json
import subprocess
import sys
import time

import pysysinfo
import pytest


def test_import_surface() -> None:
    assert pysysinfo.System.__module__ == "pysysinfo"
    assert pysysinfo.Process.__module__ == "pysysinfo"
    assert pysysinfo.Group.__module__ == "pysysinfo"
    assert pysysinfo.DiskUsage.__module__ == "pysysinfo"
    assert pysysinfo.MacAddress.__module__ == "pysysinfo"
    assert pysysinfo.IpNetwork.__module__ == "pysysinfo"
    assert pysysinfo.Motherboard.__module__ == "pysysinfo"
    assert pysysinfo.Product.__module__ == "pysysinfo"
    assert isinstance(pysysinfo.SUPPORTED_SIGNALS, tuple)


def test_system_properties_are_immutable_tuples() -> None:
    system = pysysinfo.System()

    assert isinstance(system.cpus, tuple)
    assert isinstance(system.disks, tuple)
    assert isinstance(system.networks, tuple)
    assert isinstance(system.components, tuple)
    assert isinstance(system.users, tuple)
    assert isinstance(system.groups, tuple)
    assert isinstance(system.processes, tuple)
    assert isinstance(system.distribution_id_like, tuple)
    assert isinstance(system.supported_signals, tuple)


def test_refresh_methods_keep_the_system_usable() -> None:
    system = pysysinfo.System.new_empty()

    system.refresh_memory()
    system.refresh_memory_specifics(ram=True, swap=False)
    system.refresh_disks()
    system.refresh_disks_specifics(kind=True, storage=True, io_usage=True)
    system.refresh_networks()
    system.refresh_components()
    system.refresh_users()
    system.refresh_groups()
    system.refresh_cpu_list(cpu_usage=True, frequency=True)
    system.refresh_cpu_specifics(cpu_usage=True, frequency=False)
    system.refresh_cpu_usage()
    time.sleep(pysysinfo.MINIMUM_CPU_UPDATE_INTERVAL)
    system.refresh_cpu_frequency()
    system.refresh_cpu_all()
    system.refresh_processes(remove_dead_processes=True)
    system.refresh_processes_specifics(
        cpu=True,
        disk_usage=True,
        memory=True,
        user="always",
        cwd="only_if_not_set",
        root="only_if_not_set",
        environ="never",
        cmd="always",
        exe="always",
        tasks=True,
    )
    updated = system.refresh_specifics(
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
    system.refresh_all()

    assert updated >= 0
    assert isinstance(system.global_cpu_usage, float)
    assert isinstance(system.total_memory, int)
    assert isinstance(system.open_files_limit, int | type(None))


def test_to_dict_and_to_json_round_trip() -> None:
    system = pysysinfo.System()

    as_dict = system.to_dict()
    as_json = json.loads(system.to_json())

    assert as_dict["distribution_id"] == as_json["distribution_id"]
    assert isinstance(as_dict["cpus"], list)
    assert isinstance(as_dict["networks"], list)
    assert isinstance(as_dict["groups"], list)
    assert isinstance(as_dict["processes"], list)
    assert isinstance(as_dict["load_average"], dict)
    assert "product" in as_dict


def test_nested_snapshot_objects_are_readable_and_frozen() -> None:
    system = pysysinfo.System()

    assert repr(system).startswith("System(")

    if system.cpus:
        cpu = system.cpus[0]
        assert cpu.to_dict()["name"] == cpu.name
        with pytest.raises(AttributeError):
            cpu.name = "renamed"  # type: ignore[misc]

    if system.disks:
        disk = system.disks[0]
        usage = disk.usage
        assert usage.to_dict()["written_bytes"] >= 0

    if system.networks:
        network = system.networks[0]
        assert isinstance(network.mac_address.value, str)
        assert isinstance(network.ip_networks, tuple)

    if system.users:
        user = system.users[0]
        assert isinstance(user.groups, tuple)
        if user.groups:
            assert isinstance(user.groups[0].name, str)

    if system.components:
        component = system.components[0]
        assert component.temperature is None or isinstance(component.temperature, float)
        assert component.max is None or isinstance(component.max, float)

    if system.processes:
        process = system.processes[0]
        assert process.disk_usage.to_dict()["total_read_bytes"] >= 0
        assert process.tasks is None or isinstance(process.tasks, tuple)


def test_module_constants_and_helpers() -> None:
    assert isinstance(pysysinfo.IS_SUPPORTED_SYSTEM, bool)
    assert pysysinfo.MINIMUM_CPU_UPDATE_INTERVAL >= 0.0
    assert pysysinfo.get_current_pid() > 0
    assert isinstance(pysysinfo.set_open_files_limit(32), bool)


def test_process_lookup_and_control() -> None:
    child = subprocess.Popen(
        [sys.executable, "-c", "import time; time.sleep(30)"],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )

    try:
        system = pysysinfo.System.new_empty()
        updated = system.refresh_processes_specifics(
            [child.pid],
            True,
            cpu=True,
            disk_usage=True,
            memory=True,
            user="always",
            cwd="always",
            root="always",
            environ="always",
            cmd="always",
            exe="always",
            tasks=True,
        )

        assert updated >= 0

        process = system.process(child.pid)
        assert process is not None
        assert process.pid == child.pid
        assert isinstance(process.cmd, list)
        assert process.disk_usage.total_written_bytes >= 0

        status = process.kill_and_wait()
        assert status is not None
        assert child.wait(timeout=5) is not None
    finally:
        if child.poll() is None:
            child.kill()
            child.wait(timeout=5)


def test_user_lookup_and_public_docstrings_are_present() -> None:
    system = pysysinfo.System()

    if system.users:
        resolved = system.get_user_by_id(system.users[0].id)
        assert resolved is not None
        assert resolved.id == system.users[0].id

    assert pysysinfo.System.__doc__
    assert pysysinfo.System.refresh_processes_specifics.__doc__
    assert pysysinfo.Process.kill_and_wait.__doc__
    assert pysysinfo.Product.to_dict.__doc__
