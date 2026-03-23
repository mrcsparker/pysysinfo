from __future__ import annotations

from pysysinfo import Sysinfo


def preview_strings(values: list[str], limit: int = 5) -> str:
    preview = values[:limit]
    suffix = "" if len(values) <= limit else ", ..."
    return ", ".join(preview) + suffix


def main() -> None:
    system = Sysinfo()

    print("== disks ==")
    for disk in system.disks:
        print(
            f"{disk.name} ({disk.kind}) mount={disk.mount_point} "
            f"available={disk.available_space} total={disk.total_space} "
            f"read_only={disk.is_read_only} removable={disk.is_removable}"
        )
        print(f"  usage={disk.usage.to_dict()}")

    print("\n== networks ==")
    for network in system.networks:
        print(
            f"{network.interface}: rx={network.total_received} tx={network.total_transmitted} "
            f"mtu={network.mtu} mac={network.mac_address.value}"
        )
        if network.ip_networks:
            print("  ip networks:")
            for ip_network in network.ip_networks:
                print(f"    {ip_network.addr}/{ip_network.prefix}")

    print("\n== components ==")
    for component in system.components:
        print(
            f"{component.label}: temperature={component.temperature} "
            f"max={component.max} critical={component.critical} id={component.id}"
        )

    print("\n== users ==")
    for user in system.users:
        print(f"{user.name}: id={user.id} group_id={user.group_id}")
        if user.groups:
            rendered = [f"{group.name} ({group.id})" for group in user.groups]
            print(f"  groups: {preview_strings(rendered)}")

    print("\n== groups ==")
    for group in system.groups[:20]:
        print(f"{group.name} ({group.id})")
    if len(system.groups) > 20:
        print(f"... and {len(system.groups) - 20} more groups")

    print("\n== processes ==")
    for process in system.processes[:10]:
        print(
            f"pid={process.pid} name={process.name} status={process.status} "
            f"memory={process.memory} open_files={process.open_files}"
        )
        print(f"  disk_usage={process.disk_usage.to_dict()}")


if __name__ == "__main__":
    main()
