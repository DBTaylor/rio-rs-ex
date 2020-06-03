Example NI RIO Rust Project
===

This is an example Rust application that runs on an NI 9147 and interfaces with the FPGA using [nifpga-rs](https://github.com/dbtaylor/nifpga-rs). It includes basic deployment and debugging tooling through Visual Studio Code configurations and tasks.

Requirements 
---
* [Rust](https://www.rust-lang.org/tools/install)
* Appropriate Rust target for the 9147 ```rustup toolchain add armv7-unknown-linux-gnueabi```
* Some GCC. On Windows, [tmd-gcc](https://jmeubank.github.io/tdm-gcc/) will work
* An appropriate cross compilation toolchain for your target. This example targets an NI 9147, so [arm-linux-gnueabi from Linaro](https://releases.linaro.org/components/toolchain/binaries/latest-7/arm-linux-gnueabi/) will work. arm-linux-gnueabi-gcc also needs to be in PATH
* [PuTTY](https://www.chiark.greenend.org.uk/~sgtatham/putty/latest.html), used for deployment and debug scripts
* cURL, used for deployment scripts
* Ability to run bash scripts. On Windows, [Git for Windows](https://git-scm.com/download/win) will work
* [nifpga-apigen](https://github.com/dbtaylor/nifpga-apigen) ```cargo install nifpga-apigen```
* LabVIEW >= 2016, LabVIEW Real-Time >= 2016, LabVIEW FPGA >= 2016, NI-RIO Drivers
* [Visual Studio Code](https://code.visualstudio.com/)

Structure
---
/fpga contains the LabVIEW project with the FPGA VI and build spec. You will need to change the build destination directory to the absolute path of /app/src/fpga on your machine.

/app contains The Visual Studio Code Rust project. It contains helpful tasks:
* fpga-apigen: Generates the FPGA API to /app/src/fpga/mod.rs from the C header file.
* deploy-release: builds app in release mode, kills app if it's running on the target, deploys it and the bitfile to the the target, sets up app to run on startup, and reboots the target

The debug configuration will:
* build app with debug symbols
* kill app if it's running on the target
* deploy the debug app
* connect an external terminal to the target and run the app through gdbserver
* start a gdb debugging instance connected to the target

Configuration
---
The project has the following configuration:
* The target IP Address is 192.168.0.12
* The target admin password is empty
* The binary name is app
* The binary and bitfile are deployed to /home/lvuser on the target
* The debug server port is 3245

To change the IP Address, change:
* app/scripts/config ```ip=192.168.0.12```
* app/.vscode/launch.json ```"miDebuggerServerAddress": "192.168.0.12:3245"```

To change the target admin password, change:
* app/scripts/password ```password=```

To change the binary name, change:
* app/Cargo.toml ```name = "app"```
* app/scripts/app.sh ```admin --exec /home/lvuser/app```
* app/scripts/config ```app=app```
* app/.vscode/launch.json ```"program": "${workspaceRoot}/target/armv7-unknown-linux-gnueabi/debug/app"```

To change the app and bitfile deployment location, change:
* app/scripts/app.sh ```admin --exec /home/lvuser/app```
* app/scripts/config ```remote_dir=/home/lvuser```
* build.rs ```Command::new("nifpga-apigen").args(&["./src/fpga/NiFpga_fpga.h", "-g"])``` with a -p flag with the correct path to the bitfile
* app/.vscode/tasks.json ```"command": "nifpga-apigen ./src/fpga/NiFpga_fpga.h -g"``` with -p flag with the correct path to the bitfile

To change the bitfile name, change:
* app/scripts/config ```bitfile_name=fpga.lvbitx```
* build.rs ```Command::new("nifpga-apigen").args(&["./src/fpga/NiFpga_fpga.h", "-g"])``` with a -p flag with the correct path to the bitfile
* app/.vscode/tasks.json ```"command": "nifpga-apigen ./src/fpga/NiFpga_fpga.h -g"``` with -p flag with the correct path to the bitfile

To change the debug server port, change:
* app/scripts/config ```debug_port=3245```
* app/.vscode/launch.json ```"miDebuggerServerAddress": "192.168.0.12:3245"```