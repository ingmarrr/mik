# Macosx supervisor calls

[Supervisor Calls](https://opensource.apple.com/source/xnu/xnu-1504.3.12/bsd/kern/syscalls.master)

# As and Ld

```bash
Maos sdk: xcrun -sdk macosx --show-sdk-path
as -o <name>.o <name>.s
ld -e _start -l System -syslibroot `xcrun -sdk macosx --show-sdk-path` -arch arm64 -o <name> <name>.o
```
