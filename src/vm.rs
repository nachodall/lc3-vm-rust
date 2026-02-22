use std::fs::File;
use std::io::{BufReader, Read};

pub const MEMORY_MAX: usize = 1 << 16;
pub const PC_START: u16 = 0x3000;
const REG_COUNT: usize = Register::Count as usize;

#[derive(Clone, Copy, PartialEq, Debug)]
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

impl Register {
    pub fn from_u16(v: u16) -> Option<Register> {
        match v {
            0 => Some(Register::R0),
            1 => Some(Register::R1),
            2 => Some(Register::R2),
            3 => Some(Register::R3),
            4 => Some(Register::R4),
            5 => Some(Register::R5),
            6 => Some(Register::R6),
            7 => Some(Register::R7),
            8 => Some(Register::PC),
            9 => Some(Register::Cond),
            10 => Some(Register::Count),
            _ => None,
        }
    }
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

#[derive(Copy, Clone, Debug, PartialEq)] // Agregamos Debug acá
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

impl Opcode {
    pub fn from_u16(value: u16) -> Option<Opcode> {
        match value {
            0 => Some(Opcode::Br),
            1 => Some(Opcode::Add),
            2 => Some(Opcode::Ld),
            3 => Some(Opcode::St),
            4 => Some(Opcode::Jsr),
            5 => Some(Opcode::And),
            6 => Some(Opcode::Ldr),
            7 => Some(Opcode::Str),
            8 => Some(Opcode::Rti),
            9 => Some(Opcode::Not),
            10 => Some(Opcode::Ldi),
            11 => Some(Opcode::Sti),
            12 => Some(Opcode::Jmp),
            13 => Some(Opcode::Res),
            14 => Some(Opcode::Lea),
            15 => Some(Opcode::Trap),
            _ => None,
        }
    }
}

pub struct Vm {
    memory: [u16; MEMORY_MAX],
    registers: [u16; REG_COUNT],
}

impl Default for Vm {
    fn default() -> Self {
        let mut registers = [0; REG_COUNT];
        registers[Register::PC as usize] = PC_START;
        registers[Register::Cond as usize] = ConditionalFlag::Zro as u16;

        Self {
            memory: [0; MEMORY_MAX],
            registers,
        }
    }
}

impl Vm {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn read_memory(&mut self, addr: u16) -> u16 {
        if addr == MR_KBSR as u16 {
            if crate::hardware::check_key() {
                self.memory[MR_KBSR] = 1 << 15;
                let mut buffer = [0; 1];
                if std::io::stdin().read_exact(&mut buffer).is_ok() {
                    self.memory[MR_KBDR] = buffer[0] as u16;
                } else {
                    self.memory[MR_KBSR] = 0;
                }
            } else {
                self.memory[MR_KBSR] = 0;
            }
        }
        self.memory[addr as usize]
    }

    pub fn write_memory(&mut self, addr: u16, value: u16) {
        self.memory[addr as usize] = value;
    }

    pub fn read_register(&self, reg: Register) -> u16 {
        self.registers[reg as usize]
    }

    pub fn write_register(&mut self, reg: Register, value: u16) {
        self.registers[reg as usize] = value;
        if (reg as usize) < Register::PC as usize {
            self.update_flags(value);
        }
    }

    fn update_flags(&mut self, value: u16) {
        if value == 0 {
            self.registers[Register::Cond as usize] = ConditionalFlag::Zro as u16;
        } else if (value >> 15) == 1 {
            self.registers[Register::Cond as usize] = ConditionalFlag::Neg as u16;
        } else {
            self.registers[Register::Cond as usize] = ConditionalFlag::Pos as u16;
        }
    }

    pub fn reg(&self, bits: u16) -> Register {
        Register::from_u16(bits).expect("Invalid Register bits")
    }

    pub fn sign_ext(&self, x: u16, bit_count: usize) -> u16 {
        (((x as i16) << (16 - bit_count)) >> (16 - bit_count)) as u16
    }

    pub fn read_image_file(&mut self, path: &str) -> std::io::Result<()> {
        let mut file = File::open(path)?;
        let mut reader = BufReader::new(&mut file);
        let mut buffer_bytes = [0u8; 2];

        reader.read_exact(&mut buffer_bytes)?;
        let origin_addr = u16::from_be_bytes(buffer_bytes);
        self.write_register(Register::PC, origin_addr);

        let mut addr = origin_addr;
        loop {
            match reader.read_exact(&mut buffer_bytes) {
                Ok(_) => {
                    let instruction = u16::from_be_bytes(buffer_bytes);
                    self.write_memory(addr, instruction);
                    addr = addr.wrapping_add(1);
                }
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    break;
                }
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}
