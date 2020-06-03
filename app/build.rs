extern crate cc;

use std::process::Command;

fn main() {
    Command::new("nifpga-apigen").args(&["./src/fpga/NiFpga_fpga.h", "-g"])
        .status().unwrap();
    cc::Build::new()
        .compiler("arm-linux-gnueabi-gcc")
        .file("src/fpga/NiFpga.c")
        .compile("NiFpga");
}
