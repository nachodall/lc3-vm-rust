mod hardware;
mod vm;
use hardware::{disable_input_buffering, restore_input_buffering};
use vm::*;

fn main() {
    disable_input_buffering();
    let _lc3 = Vm::new();
    restore_input_buffering();
}
