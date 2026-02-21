#![allow(dead_code)]
use std::io::Read;

pub const MEMORY_MAX: usize = 1 << 16;
const REG_COUNT: usize = Register::Count as usize;

#[repr(u16)]
pub enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    Cond,
    Count,
}

// Memory-Mapped I/O Registers
const MR_KBSR: usize = 0xFE00; // Keyboard Status Register
const MR_KBDR: usize = 0xFE02; // Keyboard Data Register

#[repr(u16)]
pub enum ConditionalFlag {
    Pos = 1 << 0, // P (1)
    Zro = 1 << 1, // Z (2)
    Neg = 1 << 2, // N (4)
}

#[repr(u16)]
pub enum Opcode {
    Br = 0, /* branch */
    Add,    /* add  */
    Ld,     /* load */
    St,     /* store */
    Jsr,    /* jump register */
    And,    /* bitwise and */
    Ldr,    /* load register */
    Str,    /* store register */
    Rti,    /* unused */
    Not,    /* bitwise complement */
    Ldi,    /* load indirect */
    Sti,    /* store indirect */
    Jmp,    /* jump */
    Res,    /* reserved (unused) */
    Lea,    /* load effective address */
    Trap,   /* execute trap */
}

pub struct Vm {
    memory: [u16; MEMORY_MAX],
    registers: [u16; REG_COUNT],
}

impl Vm {
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_MAX],
            registers: [0; REG_COUNT],
        }
    }

    pub fn read_memory(&mut self, addr: u16) -> u16 {
        if addr == MR_KBSR as u16 {
            if crate::hardware::check_key() {
                self.memory[MR_KBSR] = 1 << 15;
                let mut buffer = [0; 1];
                std::io::stdin().read_exact(&mut buffer).unwrap();
                self.memory[MR_KBDR] = buffer[0] as u16;
            } else {
                self.memory[MR_KBSR] = 0;
            }
        }
        self.memory[addr as usize]
    }

    pub fn write_memory(&mut self, addr: u16, val: u16) {
        self.memory[addr as usize] = val;
    }
}
