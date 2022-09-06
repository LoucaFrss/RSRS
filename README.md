# Rust Simple Reverse Shell (RSRS)



## Build

### Windows
Uncomment this line (to hide the child process window):
https://github.com/LoucaFrss/RSRS/blob/82d111a867c890ee41dfe25aa3ca1e88099c823d/src/main.rs#L3
and this line :

https://github.com/LoucaFrss/RSRS/blob/82d111a867c890ee41dfe25aa3ca1e88099c823d/src/main.rs#L11

Build with this cargo command:

`cargo rustc --release -- -Clink-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup"`

if you got an "link.exe" error, install [Visual Studio buildtools](https://aka.ms/vs/17/release/vs_buildtools.exe), open "Devlopper Powershell for VS 2022" and retry.


### Other
`cargo build --release`
