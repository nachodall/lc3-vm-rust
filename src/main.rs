mod hardware;
mod vm;
use hardware::{disable_input_buffering, restore_input_buffering};
use vm::*;

fn main() {
    disable_input_buffering();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please use: cargo run -- path/file_name.obj");
        return;
    }

    let mut lc3 = Vm::new();
    lc3.read_image_file(&args[1])
        .expect("Error while loading .obj file");

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
                Opcode::Br => {
                    let pc_offset = lc3.sign_ext(instr & 0x1FF, 9);
                    let instr_cond = (instr >> 9) & 0x7;
                    let current_cond = lc3.read_register(Register::Cond);

                    if (current_cond & instr_cond) != 0 {
                        let current_pc_value = lc3.read_register(Register::PC);
                        lc3.write_register(Register::PC, current_pc_value.wrapping_add(pc_offset));
                    }
                }

                Opcode::Add => {
                    let dst = lc3.reg((instr >> 9) & 0x7);
                    let src1 = lc3.reg((instr >> 6) & 0x7);
                    let imm_flag = (instr >> 5) & 0x1;

                    if imm_flag == 1 {
                        let imm5 = lc3.sign_ext(instr & 0x1F, 5);
                        let val1 = lc3.read_register(src1);
                        lc3.write_register(dst, val1.wrapping_add(imm5));
                    } else {
                        let src2 = lc3.reg(instr & 0x7);
                        let val1 = lc3.read_register(src1);
                        let val2 = lc3.read_register(src2);
                        lc3.write_register(dst, val1.wrapping_add(val2));
                    }
                }

                Opcode::Ld => {
                    let dst = lc3.reg((instr >> 9) & 0x7);
                    let pc_offset = lc3.sign_ext(instr & 0x1FF, 9);
                    let pc_val = lc3.read_register(Register::PC);
                    let addr = pc_val.wrapping_add(pc_offset);

                    let val = lc3.read_memory(addr);
                    lc3.write_register(dst, val);
                }

                Opcode::St => {
                    let src = lc3.reg((instr >> 9) & 0x7);
                    let pc_offset = lc3.sign_ext(instr & 0x1FF, 9);
                    let pc_val = lc3.read_register(Register::PC);
                    let addr = pc_val.wrapping_add(pc_offset);

                    let val = lc3.read_register(src);
                    lc3.write_memory(addr, val);
                }

                Opcode::And => {
                    let dst = lc3.reg((instr >> 9) & 0x7);
                    let src1 = lc3.reg((instr >> 6) & 0x7);
                    let imm_flag = (instr >> 5) & 0x1;

                    if imm_flag == 1 {
                        let imm5 = lc3.sign_ext(instr & 0x1F, 5);
                        let val1 = lc3.read_register(src1);
                        lc3.write_register(dst, val1 & imm5);
                    } else {
                        let src2 = lc3.reg(instr & 0x7);
                        let val1 = lc3.read_register(src1);
                        let val2 = lc3.read_register(src2);
                        lc3.write_register(dst, val1 & val2);
                    }
                }

                Opcode::Not => {
                    let dst = lc3.reg((instr >> 9) & 0x7);
                    let src = lc3.reg((instr >> 6) & 0x7);
                    let val = lc3.read_register(src);
                    lc3.write_register(dst, !val);
                }

                Opcode::Jmp => {
                    let src_reg = lc3.reg((instr >> 6) & 0x7);
                    let val = lc3.read_register(src_reg);
                    lc3.write_register(Register::PC, val);
                }

                Opcode::Lea => {
                    let dst = lc3.reg((instr >> 9) & 0x7);
                    let pc_offset = lc3.sign_ext(instr & 0x1FF, 9);
                    let val = lc3.read_register(Register::PC).wrapping_add(pc_offset);
                    lc3.write_register(dst, val);
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

    println!("Debug information");
    println!("R1: {} ", lc3.read_register(Register::R1));
    println!("PC: 0x{:04X} ", lc3.read_register(Register::PC));

    let cond = lc3.read_register(Register::Cond);
    let cond_name = match cond {
        1 => "P",
        2 => "Z",
        4 => "N",
        _ => "Unknown",
    };
    println!("Flag COND: {}", cond_name);

    restore_input_buffering();
}
