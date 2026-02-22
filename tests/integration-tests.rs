use assert_cmd::cargo::cargo_bin_cmd;
use lc3_vm_rust::vm::Vm;
use predicates::prelude::*;

#[test]
fn test_hello_world_memory_load() {
    let mut lc3 = Vm::new();
    let result = lc3.read_image_file("assets/hello.obj");
    assert!(result.is_ok(), "Failed to load assets/hello.obj.");

    let first_instr = lc3.read_memory(0x3000);
    assert_ne!(
        first_instr, 0,
        "Memory at 0x3000 should contain an instruction, but it is empty."
    );
}

#[test]
fn test_hello_world_terminal_output() {
    let mut cmd = cargo_bin_cmd!("lc3-vm-rust");

    cmd.arg("assets/hello.obj")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello World!"))
        .stdout(predicate::str::contains("HALT"));
}

#[test]
fn test_rogue_memory_load() {
    let mut lc3 = Vm::new();
    let result = lc3.read_image_file("assets/rogue.obj");
    assert!(result.is_ok(), "Failed to load assets/rogue.obj.");

    let first_instr = lc3.read_memory(0x3000);
    assert_ne!(
        first_instr, 0,
        "Memory at 0x3000 for Rogue should contain an instruction, but it is empty."
    );
}

#[test]
fn test_2048_memory_load() {
    let mut lc3 = Vm::new();
    let result = lc3.read_image_file("assets/2048.obj");
    assert!(result.is_ok(), "Failed to load assets/2048.obj.");

    let first_instr = lc3.read_memory(0x3000);
    assert_ne!(
        first_instr, 0,
        "Memory at 0x3000 for 2048 should contain an instruction, but it is empty."
    );
}
