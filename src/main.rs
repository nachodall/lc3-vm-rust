mod hardware;
mod vm;
use hardware::{disable_input_buffering, restore_input_buffering};
use std::io::{Read, Write};
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
        if let Some(opcode) = Opcode::from_u16(opcode_bits) {
            match opcode {
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

                Opcode::Ldr => {
                    let dst = lc3.reg((instr >> 9) & 0x7);
                    let src = lc3.reg((instr >> 6) & 0x7);
                    let offset = lc3.sign_ext(instr & 0x3F, 6);

                    let val_base = lc3.read_register(src);
                    let addr = val_base.wrapping_add(offset);

                    let val = lc3.read_memory(addr);
                    lc3.write_register(dst, val);
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
                    /* TRAP instructions in the LC-3 architecture are service calls.
                    According to the spec, the current PC must be saved in R7
                    to allow the service routine to return (via RET/JMP R7).
                    Even though our VM handles traps directly in Rust (keeping the control
                    flow within this loop), we update R7 to maintain architectural
                    fidelity and compatibility with programs that might inspect it. */
                    let current_pc = lc3.read_register(Register::PC);
                    lc3.write_register(Register::R7, current_pc);

                    let trap_vector = instr & 0xFF;

                    match trap_vector {
                        0x20 => {
                            // GETC
                            let mut buffer = [0u8; 1];
                            std::io::stdin().read_exact(&mut buffer).unwrap();
                            lc3.write_register(Register::R0, buffer[0] as u16);
                        }

                        0x21 => {
                            // OUT
                            let char = (lc3.read_register(Register::R0) & 0xFF) as u8 as char;
                            print!("{}", char);
                            std::io::stdout().flush().expect("Failed to flush");
                        }

                        0x22 => {
                            // PUTS
                            let mut addr = lc3.read_register(Register::R0);
                            loop {
                                let char = lc3.read_memory(addr);
                                if char == 0x0000 {
                                    break;
                                }
                                print!("{}", (char as u8) as char);
                                addr = addr.wrapping_add(1);
                            }
                            std::io::stdout().flush().expect("Failed to flush");
                        }

                        0x23 => {
                            // IN
                            print!("Enter a character: ");
                            std::io::stdout().flush().expect("Failed to flush");
                            let mut buffer = [0u8; 1];
                            std::io::stdin().read_exact(&mut buffer).unwrap();
                            let char = buffer[0] as char;
                            print!("{}", char);
                            std::io::stdout().flush().expect("Failed to flush");

                            lc3.write_register(Register::R0, buffer[0] as u16);
                        }

                        0x24 => {
                            // PUTSP
                            let mut addr = lc3.read_register(Register::R0);
                            loop {
                                let word = lc3.read_memory(addr);
                                if word == 0x0000 {
                                    break;
                                }

                                let char_l = (word & 0xFF) as u8;
                                print!("{}", char_l as char);

                                let char_h = (word >> 8) as u8;
                                if char_h != 0 {
                                    print!("{}", char_h as char);
                                }

                                addr = addr.wrapping_add(1);
                            }
                            std::io::stdout().flush().expect("Failed to flush");
                        }

                        0x25 => {
                            // HALT
                            println!("HALT");
                            std::io::stdout().flush().expect("Failed to flush");
                            running = false;
                        }

                        _ => {
                            println!("trap not implemented: 0x{:02X}", trap_vector);
                            running = false;
                        }
                    }
                }

                _ => {
                    println!("Opcode {:?} not yet implemented", opcode);
                    running = false;
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
