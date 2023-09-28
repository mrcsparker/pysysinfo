import pysysinfo

def main() -> None:
    print("Getting system information...")
    system = pysysinfo.Sysinfo()
    networks = pysysinfo.Networks()
    # let mut disks = Disks::new();
    # let mut components = Components::new();
    networks.refresh_list()
    # disks.refresh_list();
    # components.refresh_list();
    print("Done.")


if __name__ == "__main__":
    main()
