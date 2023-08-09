#![doc = include_str!("../README.md")]

use std::ops::ControlFlow;

pub use memory::*;

mod memory;

const REGISTER_COUNT: usize = 13;
pub const REGISTER_ZERO: u8 = 0b000;
pub const REGISTER_INSTRUCTION_POINTER: u8 = 0b001;
pub const REGISTER_RETURN_ADDRESS: u8 = 0b010;
pub const REGISTER_STACK_POINTER: u8 = 0b011;

pub const INSTRUCTION_CATEGORY_INVALID: u8 = 0x0;
pub const INSTRUCTION_INVALID: u8 = 0x0;

pub const INSTRUCTION_CATEGORY_ARITHMETIC: u8 = 0x1;
pub const INSTRUCTION_ADD: u8 = 0x0;
pub const INSTRUCTION_SUB: u8 = 0x1;
pub const INSTRUCTION_MUL: u8 = 0x2;
pub const INSTRUCTION_DIVMOD: u8 = 0x3;
pub const INSTRUCTION_AND: u8 = 0x4;
pub const INSTRUCTION_OR: u8 = 0x5;
pub const INSTRUCTION_XOR: u8 = 0x6;
pub const INSTRUCTION_NOT: u8 = 0x7;

pub const INSTRUCTION_CATEGORY_MEMORY: u8 = 0x2;
pub const INSTRUCTION_COPY: u8 = 0x0;
pub const INSTRUCTION_LOAD: u8 = 0x1;
pub const INSTRUCTION_READ: u8 = 0x2;
pub const INSTRUCTION_WRITE: u8 = 0x3;
pub const INSTRUCTION_PUSH: u8 = 0x4;
pub const INSTRUCTION_POP: u8 = 0x5;

pub const INSTRUCTION_CATEGORY_CONTROL_FLOW: u8 = 0x3;
pub const INSTRUCTION_HALT: u8 = 0x0;
pub const INSTRUCTION_CALL: u8 = 0x1;

pub struct Cpu {
    memory: Memory,
    registers: [u64; REGISTER_COUNT],
}

pub enum CpuStepError {
    Halt,
    InvalidInstruction,
    InvalidInstructionCategory,
    InvalidInvalidInstruction,
    InvalidArithmeticInstruction,
    InvalidMemoryInstruction,
    InvalidControlFlowInstruction,
}

impl Cpu {
    pub fn new(memory_size: u64, start_address: u64) -> Cpu {
        Cpu {
            memory: Memory::new(memory_size),
            registers: std::array::from_fn(|i| match i as u8 {
                REGISTER_INSTRUCTION_POINTER => start_address,
                _ => 0,
            }),
        }
    }

    fn fetch(&mut self) -> u8 {
        let value = self
            .memory
            .read(self.registers[REGISTER_INSTRUCTION_POINTER as usize]);
        self.registers[REGISTER_INSTRUCTION_POINTER as usize] += 1;
        value
    }

    fn fetch_u64(&mut self) -> u64 {
        let bytes = std::array::from_fn(|_| self.fetch());
        u64::from_le_bytes(bytes)
    }

    pub fn step(&mut self) -> ControlFlow<CpuStepError> {
        self.registers[REGISTER_ZERO as usize] = 0;

        let opcode = self.fetch();
        let category = opcode & 0x0F;
        let instruction = (opcode & 0xF0) >> 4;
        match category {
            INSTRUCTION_CATEGORY_INVALID => match instruction {
                INSTRUCTION_INVALID => return ControlFlow::Break(CpuStepError::InvalidInstruction),
                _ => return ControlFlow::Break(CpuStepError::InvalidInvalidInstruction),
            },

            INSTRUCTION_CATEGORY_ARITHMETIC => match instruction {
                INSTRUCTION_ADD => {
                    let inputs = self.fetch();
                    let a = inputs & 0x0F;
                    let b = (inputs & 0xF0) >> 4;
                    let outputs = self.fetch();
                    let o = outputs & 0x0F;
                    self.registers[o as usize] =
                        self.registers[a as usize].wrapping_add(self.registers[b as usize]);
                }
                INSTRUCTION_SUB => {
                    let inputs = self.fetch();
                    let a = inputs & 0x0F;
                    let b = (inputs & 0xF0) >> 4;
                    let outputs = self.fetch();
                    let o = outputs & 0x0F;
                    self.registers[o as usize] =
                        self.registers[a as usize].wrapping_sub(self.registers[b as usize]);
                }
                INSTRUCTION_MUL => {
                    let inputs = self.fetch();
                    let a = inputs & 0x0F;
                    let b = (inputs & 0xF0) >> 4;
                    let outputs = self.fetch();
                    let o = outputs & 0x0F;
                    self.registers[o as usize] =
                        self.registers[a as usize].wrapping_mul(self.registers[b as usize]);
                }
                INSTRUCTION_DIVMOD => {
                    let inputs = self.fetch();
                    let a = inputs & 0x0F;
                    let b = (inputs & 0xF0) >> 4;
                    let outputs = self.fetch();
                    let o = outputs & 0x0F;
                    let r = (outputs & 0xF0) >> 4;
                    self.registers[o as usize] = self.registers[a as usize]
                        .checked_div(self.registers[b as usize])
                        .unwrap_or(0);
                    self.registers[r as usize] = self.registers[a as usize]
                        .checked_rem(self.registers[b as usize])
                        .unwrap_or(0);
                }
                INSTRUCTION_AND => {
                    let inputs = self.fetch();
                    let a = inputs & 0x0F;
                    let b = (inputs & 0xF0) >> 4;
                    let outputs = self.fetch();
                    let o = outputs & 0x0F;
                    self.registers[o as usize] =
                        self.registers[a as usize] & self.registers[b as usize];
                }
                INSTRUCTION_OR => {
                    let inputs = self.fetch();
                    let a = inputs & 0x0F;
                    let b = (inputs & 0xF0) >> 4;
                    let outputs = self.fetch();
                    let o = outputs & 0x0F;
                    self.registers[o as usize] =
                        self.registers[a as usize] | self.registers[b as usize];
                }
                INSTRUCTION_XOR => {
                    let inputs = self.fetch();
                    let a = inputs & 0x0F;
                    let b = (inputs & 0xF0) >> 4;
                    let outputs = self.fetch();
                    let o = outputs & 0x0F;
                    self.registers[o as usize] =
                        self.registers[a as usize] ^ self.registers[b as usize];
                }
                INSTRUCTION_NOT => {
                    let operands = self.fetch();
                    let a = operands & 0x0F;
                    let o = (operands & 0xF0) >> 4;
                    self.registers[o as usize] = !self.registers[a as usize];
                }
                _ => return ControlFlow::Break(CpuStepError::InvalidArithmeticInstruction),
            },

            INSTRUCTION_CATEGORY_MEMORY => match instruction {
                INSTRUCTION_COPY => {
                    let operands = self.fetch();
                    let a = operands & 0x0F;
                    let b = (operands & 0xF0) >> 4;
                    self.registers[b as usize] = self.registers[a as usize];
                }
                INSTRUCTION_LOAD => {
                    let operand = self.fetch();
                    let a = operand & 0x0F;
                    let imm = self.fetch_u64();
                    self.registers[a as usize] = imm;
                }
                INSTRUCTION_READ => {
                    let operands = self.fetch();
                    let a = operands & 0x0F;
                    let b = (operands & 0xF0) >> 4;
                    self.registers[b as usize] = self.memory.read_u64(self.registers[a as usize]);
                }
                INSTRUCTION_WRITE => {
                    let operands = self.fetch();
                    let a = operands & 0x0F;
                    let b = (operands & 0xF0) >> 4;
                    self.memory
                        .write_u64(self.registers[b as usize], self.registers[a as usize]);
                }
                INSTRUCTION_PUSH => {
                    let operands = self.fetch();
                    let a = operands & 0x0F;
                    self.registers[REGISTER_STACK_POINTER as usize] -= 8;
                    self.memory.write_u64(
                        self.registers[REGISTER_STACK_POINTER as usize],
                        self.registers[a as usize],
                    );
                }
                INSTRUCTION_POP => {
                    let operands = self.fetch();
                    let a = operands & 0x0F;
                    self.registers[a as usize] = self
                        .memory
                        .read_u64(self.registers[REGISTER_STACK_POINTER as usize]);
                    self.registers[REGISTER_STACK_POINTER as usize] += 8;
                }
                _ => return ControlFlow::Break(CpuStepError::InvalidMemoryInstruction),
            },

            INSTRUCTION_CATEGORY_CONTROL_FLOW => match instruction {
                INSTRUCTION_HALT => return ControlFlow::Break(CpuStepError::Halt),
                INSTRUCTION_CALL => {
                    let operands = self.fetch();
                    let a = operands & 0x0F;
                    self.registers[REGISTER_RETURN_ADDRESS as usize] =
                        self.registers[REGISTER_INSTRUCTION_POINTER as usize];
                    self.registers[REGISTER_INSTRUCTION_POINTER as usize] =
                        self.registers[a as usize];
                }
                _ => return ControlFlow::Break(CpuStepError::InvalidControlFlowInstruction),
            },

            _ => return ControlFlow::Break(CpuStepError::InvalidInstructionCategory),
        }
        ControlFlow::Continue(())
    }
}
