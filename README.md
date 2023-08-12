# pysysinfo

Python wrapper for [Sysinfo](https://github.com/GuillaumeGomez/sysinfo)

## Usage

```python
import pysysinfo

sys = pysysinfo.Sysinfo()
sys.refresh_all()

for disk in sys.disks():
    print(disk)

for cpu in sys.cpus():
    print(cpu)
```
