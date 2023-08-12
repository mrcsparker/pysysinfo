import pysysinfo

sys = pysysinfo.Sysinfo()
sys.refresh_all()

print(sys)

for disk in sys.disks():
    print(disk)

for cpu in sys.cpus():
    print(cpu)

for user in sys.users():
    print(user)

for network in sys.networks():
    print(network)
