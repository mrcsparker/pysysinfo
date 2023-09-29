import pysysinfo

def main() -> None:
    print("Getting system information...")
    system = pysysinfo.Sysinfo()
    print("Done.")

    print("To get the commands' list, enter 'help'.")


if __name__ == "__main__":
    main()
