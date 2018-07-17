use register_pair::*;
use memory_manager::*;
use instructions::*;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Cpu {

    // Register pairs
    reg_af: RegisterPair,
    reg_bc: RegisterPair,
    reg_de: RegisterPair,
    reg_hl: RegisterPair,
    reg_sp: RegisterPair,

    // Program counter
    reg_pc: u16,

    // Memory manager
    memory_manager: Rc<RefCell<MemoryManager>>,

}

impl Cpu {

    /// Default constructor.
    pub fn new(memory_manager: Rc<RefCell<MemoryManager>>) -> Cpu {
        Cpu {
            reg_af: RegisterPair::new(0x01B0),
            reg_bc: RegisterPair::new(0x0013),
            reg_de: RegisterPair::new(0x00D8),
            reg_hl: RegisterPair::new(0x014D),
            reg_sp: RegisterPair::new(0xFFFE),
            reg_pc: 0x0100,
            memory_manager: memory_manager
        }
    }

    /// Returns the next byte in memory.
    pub fn get_byte(&mut self) -> u8 {
        let byte = self.memory_manager.borrow_mut().read_memory(self.reg_pc);
        self.reg_pc += 1;
        byte
    }

    /// Returns the next word in memory.
    pub fn get_word(&mut self) -> u16 {
        let byte_lo = self.memory_manager.borrow_mut().read_memory(self.reg_pc);
        let byte_hi = self.memory_manager.borrow_mut().read_memory(self.reg_pc + 1);
        let word = ((byte_hi as u16) << 8) | (byte_lo as u16);
        self.reg_pc += 2;
        word
    }

    /// Moves the PC and reads the next opcode,
    /// then returns the number of cycles it 
    /// took.
    pub fn interpret_opcode(&mut self) {
        let opcode = self.memory_manager.borrow_mut().read_memory(self.reg_pc);
        self.reg_pc += 1;
        match opcode {
            0x00 => { /* NOP */ },
            0x01 => { ld_u16_reg_pair(self.get_word(), &mut self.reg_bc) },
            0x02 => {},
            0x03 => {},
            0x04 => {},
            0x05 => {},
            0x06 => {},
            0x07 => {},
            0x08 => {},
            0x09 => {},
            0x0A => {},
            0x0B => {},
            0x0C => {},
            0x0D => {},
            0x0E => { ld_u8_reg(self.get_byte(), &mut self.reg_bc.get_lo()) },
            0x0F => {},
            0x10 => {},
            0x11 => {},
            0x12 => {},
            0x13 => {},
            0x14 => {},
            0x15 => {},
            0x16 => {},
            0x17 => {},
            0x18 => {},
            0x19 => {},
            0x1A => {},
            0x1B => {},
            0x1C => {},
            0x1D => {},
            0x1E => {},
            0x1F => {},
            0x20 => {},
            0x21 => {},
            0x22 => {},
            0x23 => {},
            0x24 => {},
            0x25 => {},
            0x26 => {},
            0x27 => {},
            0x28 => {},
            0x29 => {},
            0x2A => {},
            0x2B => {},
            0x2C => {},
            0x2D => {},
            0x2E => {},
            0x2F => {},
            0x30 => {},
            0x31 => {},
            0x32 => {},
            0x33 => {},
            0x34 => {},
            0x35 => {},
            0x36 => {},
            0x37 => {},
            0x38 => {},
            0x39 => {},
            0x3A => {},
            0x3B => {},
            0x3C => {},
            0x3D => {},
            0x3E => {},
            0x3F => {},
            0x40 => { /* LD B, B */ },
            0x41 => {},
            0x42 => {},
            0x43 => {},
            0x44 => {},
            0x45 => {},
            0x46 => {},
            0x47 => {},
            0x48 => {},
            0x49 => { /* LD C, C */ },
            0x4A => {},
            0x4B => {},
            0x4C => {},
            0x4D => {},
            0x4E => {},
            0x4F => {},
            0x50 => {},
            0x51 => {},
            0x52 => {},
            0x53 => { /* LD D, D */ },
            0x54 => {},
            0x55 => {},
            0x56 => {},
            0x57 => {},
            0x58 => {},
            0x59 => {},
            0x5A => {},
            0x5B => { /* LD E, E */ },
            0x5C => {},
            0x5D => {},
            0x5E => {},
            0x5F => {},
            0x60 => {},
            0x61 => {},
            0x62 => {},
            0x63 => {},
            0x64 => { /* LD H, H */ },
            0x65 => {},
            0x66 => {},
            0x67 => {},
            0x68 => {},
            0x69 => {},
            0x6A => {},
            0x6B => {},
            0x6C => {},
            0x6D => { /* LD L, L */ },
            0x6E => {},
            0x6F => {},
            0x70 => {},
            0x71 => {},
            0x72 => {},
            0x73 => {},
            0x74 => {},
            0x75 => {},
            0x76 => {},
            0x77 => {},
            0x78 => {},
            0x79 => {},
            0x7A => {},
            0x7B => {},
            0x7C => {},
            0x7D => {},
            0x7E => {},
            0x7F => { /* LD A, A */ },
            0x80 => {},
            0x81 => {},
            0x82 => {},
            0x83 => {},
            0x84 => {},
            0x85 => {},
            0x86 => {},
            0x87 => {},
            0x88 => {},
            0x89 => {},
            0x8A => {},
            0x8B => {},
            0x8C => {},
            0x8D => {},
            0x8E => {},
            0x8F => {},
            0x90 => {},
            0x91 => {},
            0x92 => {},
            0x93 => {},
            0x94 => {},
            0x95 => {},
            0x96 => {},
            0x97 => {},
            0x98 => {},
            0x99 => {},
            0x9A => {},
            0x9B => {},
            0x9C => {},
            0x9D => {},
            0x9E => {},
            0x9F => {},
            0xA0 => {},
            0xA1 => {},
            0xA2 => {},
            0xA3 => {},
            0xA4 => {},
            0xA5 => {},
            0xA6 => {},
            0xA7 => {},
            0xA8 => {},
            0xA9 => {},
            0xAA => {},
            0xAB => {},
            0xAC => {},
            0xAD => {},
            0xAE => {},
            0xAF => {},
            0xB0 => {},
            0xB1 => {},
            0xB2 => {},
            0xB3 => {},
            0xB4 => {},
            0xB5 => {},
            0xB6 => {},
            0xB7 => {},
            0xB8 => {},
            0xB9 => {},
            0xBA => {},
            0xBB => {},
            0xBC => {},
            0xBD => {},
            0xBE => {},
            0xBF => {},
            0xC0 => {},
            0xC1 => {},
            0xC2 => {},
            0xC3 => {},
            0xC4 => {},
            0xC5 => {},
            0xC6 => {},
            0xC7 => {},
            0xC8 => {},
            0xC9 => {},
            0xCA => {},
            0xCB => {},
            0xCC => {},
            0xCD => {},
            0xCE => {},
            0xCF => {},
            0xD0 => {},
            0xD1 => {},
            0xD2 => {},
            0xD3 => {},
            0xD4 => {},
            0xD5 => {},
            0xD6 => {},
            0xD7 => {},
            0xD8 => {},
            0xD9 => {},
            0xDA => {},
            0xDB => {},
            0xDC => {},
            0xDD => {},
            0xDE => {},
            0xDF => {},
            0xE0 => {},
            0xE1 => {},
            0xE2 => {},
            0xE3 => {},
            0xE4 => {},
            0xE5 => {},
            0xE6 => {},
            0xE7 => {},
            0xE8 => {},
            0xE9 => {},
            0xEA => {},
            0xEB => {},
            0xEC => {},
            0xED => {},
            0xEE => {},
            0xEF => {},
            0xF0 => {},
            0xF1 => {},
            0xF2 => {},
            0xF3 => {},
            0xF4 => {},
            0xF5 => {},
            0xF6 => {},
            0xF7 => {},
            0xF8 => {},
            0xF9 => {},
            0xFA => {},
            0xFB => {},
            0xFC => {},
            0xFD => {},
            0xFE => {},
            0xFF => {},
            _ => panic!("Undefined opcode: {:02X}", opcode)
        }
    }

    /// Getter for the program counter.
    pub fn get_reg_pc(&mut self) -> u16 {
        self.reg_pc
    }

    /// Setter for the program counter.
    pub fn set_reg_pc(&mut self, reg_pc: u16) {
        self.reg_pc = reg_pc;
    } 

    /// Pushes a value onto the stack.
    pub fn stack_push(&mut self, val: u8) {
        let prev = self.reg_sp.get_pair();
        self.reg_sp.set_pair(prev - 1);
        self.memory_manager.borrow_mut().write_memory(self.reg_sp.get_pair(), val);
    }
}