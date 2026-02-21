mod hardware;
mod vm;
use hardware::{disable_input_buffering, restore_input_buffering};
use vm::*;

fn main() {
    let reg = |bits: u16| Register::from_u16(bits).expect("Invalid Register");
    let sign_ext = |x: u16, bc: usize| (((x as i16) << (16 - bc)) >> (16 - bc)) as u16;

    disable_input_buffering();
    let mut lc3 = Vm::new();
    let mut running = true;

    while running {
        //fetch
        let pc_idx = lc3.read_register(Register::PC);
        let instr = lc3.read_memory(pc_idx);

        lc3.write_register(Register::PC, pc_idx.wrapping_add(1));

        //decode
        let opcode_bits = instr >> 12;

        //execute
        if let Some(op) = Opcode::from_u16(opcode_bits) {
            match op {
                Opcode::Add => {
                    let dr = reg((instr >> 9) & 0x7);
                    let sr1 = reg((instr >> 6) & 0x7);
                    let imm_flag = (instr >> 5) & 0x1;

                    if imm_flag == 1 {
                        let imm5 = sign_ext(instr & 0x1F, 5);
                        let val1 = lc3.read_register(sr1);
                        lc3.write_register(dr, val1.wrapping_add(imm5));
                    } else {
                        let sr2 = reg(instr & 0x7);
                        let val1 = lc3.read_register(sr1);
                        let val2 = lc3.read_register(sr2);
                        lc3.write_register(dr, val1.wrapping_add(val2));
                    }
                }

                Opcode::Trap => {
                    running = false;
                }
                _ => {
                    // Opcode not yet supported
                }
            }
        }
    }

    restore_input_buffering();
}
