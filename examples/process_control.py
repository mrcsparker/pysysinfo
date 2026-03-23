from __future__ import annotations

import subprocess
import sys
from contextlib import suppress

from pysysinfo import System


def spawn_sleeping_child(seconds: float) -> subprocess.Popen[str]:
    return subprocess.Popen(
        [sys.executable, "-c", f"import time; time.sleep({seconds})"],
        text=True,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )


def refresh_process(system: System, pid: int):
    system.refresh_processes_specifics(
        [pid],
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
    return system.process(pid)


def wait_and_cleanup(child: subprocess.Popen[str]) -> None:
    with suppress(Exception):
        if child.poll() is None:
            child.kill()
    with suppress(Exception):
        child.wait(timeout=5)


def main() -> None:
    system = System.new_empty()

    print("== wait() on a naturally exiting child ==")
    child = spawn_sleeping_child(0.5)
    try:
        process = refresh_process(system, child.pid)
        if process is not None:
            status = process.wait()
            print(f"wait() -> {status}")
    finally:
        wait_and_cleanup(child)

    print("\n== kill() ==")
    child = spawn_sleeping_child(30)
    try:
        process = refresh_process(system, child.pid)
        if process is not None:
            print(f"kill() -> {process.kill()}")
    finally:
        wait_and_cleanup(child)

    print("\n== kill_with() + wait() ==")
    child = spawn_sleeping_child(30)
    try:
        process = refresh_process(system, child.pid)
        if process is not None:
            print(f"kill_with('term') -> {process.kill_with('term')}")
            print(f"wait() -> {process.wait()}")
    finally:
        wait_and_cleanup(child)

    print("\n== kill_and_wait() ==")
    child = spawn_sleeping_child(30)
    try:
        process = refresh_process(system, child.pid)
        if process is not None:
            print(f"kill_and_wait() -> {process.kill_and_wait()}")
    finally:
        wait_and_cleanup(child)

    print("\n== kill_with_and_wait() ==")
    child = spawn_sleeping_child(30)
    try:
        process = refresh_process(system, child.pid)
        if process is not None:
            print(f"kill_with_and_wait('kill') -> {process.kill_with_and_wait('kill')}")
    finally:
        wait_and_cleanup(child)


if __name__ == "__main__":
    main()
