import pysysinfo

# Initialize the Sysinfo object
sys = pysysinfo.Sysinfo()

# First we update all information of our `System` struct.
sys.refresh_all()

# We display all disks' information:
print("=> disks:\n")
for disk in sys.disks():
    print(f"{disk}\n")

# Network interfaces name, data received and data transmitted:
print("=> networks:")
for network in sys.networks():
    print(f"{network.interface}: {network.received}/{network.transmitted} B")

# Components temperature:
print("=> components:")
for component in sys.components():
    print(f"{component}")

print("\n")

print("=> system:")
# RAM and swap information:
print(f"total memory: {sys.total_memory} bytes")
print(f"used memory : {sys.used_memory} bytes")
print(f"total swap  : {sys.total_swap} bytes")
print(f"used swap   : {sys.used_swap} bytes")

# Display system information:
print(f"System name:             {sys.name}")
print(f"System kernel version:   {sys.kernel_version}")
print(f"System OS version:       {sys.os_version}")
print(f"System host name:        {sys.host_name}")

# Number of CPUs:
print(f"NB CPUs: {len(sys.cpus())}")

# Display processes ID, name na disk usage:
#for (pid, process) in sys.processes() {
#    println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
#}