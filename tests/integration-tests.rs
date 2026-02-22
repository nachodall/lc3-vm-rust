use assert_cmd::cargo::cargo_bin_cmd;
use lc3_vm_rust::vm::Vm;
use predicates::prelude::*;

#[test]
fn test_hello_world_memory_load() {
    let mut lc3 = Vm::new();
    let result = lc3.read_image_file("assets/hello.obj");
    assert!(result.is_ok(), "Fail at loading assets/hello.obj.");

    let first_instr = lc3.read_memory(0x3000);
    assert_ne!(
        first_instr, 0,
        "Memory at 0x3000 should have an instruction, but it's empty."
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
