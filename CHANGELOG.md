# Changelog

## v0.4.0

This is a major change from v0.3. You can fix most compilation errors by simply
doing `s/SyscallNo::SYS_/Sysno::/g`.

 - Created this changelog.
 - Renamed `SyscallNo::SYS_*` to `Sysno::*`.
 - Added `Errno` for more Rustic error handling.
 - Changed the `syscalls-gen` script to grab Linux headers from GitHub.
 - Added more architecture support for the syscall table. Issuing syscalls is
   still limited to x86-64, however.