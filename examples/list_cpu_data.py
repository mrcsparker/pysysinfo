import pysysinfo
import time

sys = pysysinfo.Sysinfo()

while 1:
    sys.refresh_cpu(); # Refreshing CPU information.
    for cpu in sys.cpus():
        print(f"{cpu.name}: {cpu.cpu_usage}%")

    # Sleeping to let time for the system to run for long
    # enough to have useful information.
    time.sleep(1)
