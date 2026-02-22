# LC-3 Virtual Machine

A full implementation of the LC-3 (Little Computer 3) architecture written in Rust. This project was developed as part of the LambdaClass Engineering Residency.

## Features

* **Complete Instruction Set**: Implements all standard LC-3 opcodes (`ADD`, `AND`, `BR`, `JMP`, `JSR`, `JSRR`, `LD`, `LDI`, `LDR`, `LEA`, `NOT`, `ST`, `STI`, `STR`, `TRAP`).
* **Hardware Accurate**: Accurately replicates condition flags (`N`, `Z`, `P`) and memory-mapped I/O operations.
* **Interactive Software Support**: Fully capable of executing complex `.obj` files, including interactive games with subroutine linkages and indirect memory addressing.

## Prerequisites

* [Rust / Cargo](https://www.rust-lang.org/tools/install)

## Usage

You can run any compiled LC-3 object file by passing its path as an argument.

```bash
# Run a basic Hello World program
cargo run -- assets/hello.obj

# Run LC-3 Rogue
# Controls: W, A, S, D to move the '@' character to the 'D' door.
cargo run -- assets/rogue.obj

# Run 2048
# Controls: W, A, S, D to slide tiles.
cargo run -- assets/2048.obj
# lc3-vm-rust
