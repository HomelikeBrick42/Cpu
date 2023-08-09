#![doc = include_str!("../README.md")]

use std::ops::ControlFlow;

pub use memory::*;

mod memory;

const REGISTER_COUNT: usize = 13;
const REGISTER_ZERO: u8 = 0b000;
const REGISTER_INSTRUCTION_POINTER: u8 = 0b001;
const REGISTER_RETURN_ADDRESS: u8 = 0b010;
const REGISTER_STACK_POINTER: u8 = 0b011;

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
        let category = (opcode & 0xF0) >> 4;
        let instruction = opcode & 0x0F;
        match category {
            // Invalid
            0x0 => match instruction {
                0x0 => return ControlFlow::Break(CpuStepError::InvalidInstruction),
                _ => return ControlFlow::Break(CpuStepError::InvalidInvalidInstruction),
            },

            // Arithmetic
            0x1 => match instruction {
                // Add
                0x0 => {
                    let inputs = self.fetch();
                    let a = inputs & 0x0F;
                    let b = (inputs & 0xF0) >> 4;
                    let outputs = self.fetch();
                    let o = outputs & 0x0F;
                    self.registers[o as usize] =
                        self.registers[a as usize].wrapping_add(self.registers[b as usize]);
                }
                // Sub
                0x1 => {
                    let inputs = self.fetch();
                    let a = inputs & 0x0F;
                    let b = (inputs & 0xF0) >> 4;
                    let outputs = self.fetch();
                    let o = outputs & 0x0F;
                    self.registers[o as usize] =
                        self.registers[a as usize].wrapping_sub(self.registers[b as usize]);
                }
                // Mul
                0x2 => {
                    let inputs = self.fetch();
                    let a = inputs & 0x0F;
                    let b = (inputs & 0xF0) >> 4;
                    let outputs = self.fetch();
                    let o = outputs & 0x0F;
                    self.registers[o as usize] =
                        self.registers[a as usize].wrapping_mul(self.registers[b as usize]);
                }
                // DivMod
                0x3 => {
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
                // And
                0x4 => {
                    let inputs = self.fetch();
                    let a = inputs & 0x0F;
                    let b = (inputs & 0xF0) >> 4;
                    let outputs = self.fetch();
                    let o = outputs & 0x0F;
                    self.registers[o as usize] =
                        self.registers[a as usize] & self.registers[b as usize];
                }
                // Or
                0x5 => {
                    let inputs = self.fetch();
                    let a = inputs & 0x0F;
                    let b = (inputs & 0xF0) >> 4;
                    let outputs = self.fetch();
                    let o = outputs & 0x0F;
                    self.registers[o as usize] =
                        self.registers[a as usize] | self.registers[b as usize];
                }
                // Xor
                0x6 => {
                    let inputs = self.fetch();
                    let a = inputs & 0x0F;
                    let b = (inputs & 0xF0) >> 4;
                    let outputs = self.fetch();
                    let o = outputs & 0x0F;
                    self.registers[o as usize] =
                        self.registers[a as usize] ^ self.registers[b as usize];
                }
                // Not
                0x7 => {
                    let operands = self.fetch();
                    let a = operands & 0x0F;
                    let o = (operands & 0xF0) >> 4;
                    self.registers[o as usize] = !self.registers[a as usize];
                }
                _ => return ControlFlow::Break(CpuStepError::InvalidArithmeticInstruction),
            },

            // Memory
            0x2 => match instruction {
                // Copy
                0x0 => {
                    let operands = self.fetch();
                    let a = operands & 0x0F;
                    let b = (operands & 0xF0) >> 4;
                    self.registers[b as usize] = self.registers[a as usize];
                }
                // Load
                0x1 => {
                    let operand = self.fetch();
                    let a = operand & 0x0F;
                    let imm = self.fetch_u64();
                    self.registers[a as usize] = imm;
                }
                // Read
                0x2 => {
                    let operands = self.fetch();
                    let a = operands & 0x0F;
                    let b = (operands & 0xF0) >> 4;
                    self.registers[b as usize] = self.memory.read_u64(self.registers[a as usize]);
                }
                // Write
                0x3 => {
                    let operands = self.fetch();
                    let a = operands & 0x0F;
                    let b = (operands & 0xF0) >> 4;
                    self.memory
                        .write_u64(self.registers[b as usize], self.registers[a as usize]);
                }
                // Push
                0x4 => {
                    let operands = self.fetch();
                    let a = operands & 0x0F;
                    self.registers[REGISTER_STACK_POINTER as usize] -= 8;
                    self.memory.write_u64(
                        self.registers[REGISTER_STACK_POINTER as usize],
                        self.registers[a as usize],
                    );
                }
                // Pop
                0x5 => {
                    let operands = self.fetch();
                    let a = operands & 0x0F;
                    self.registers[a as usize] = self
                        .memory
                        .read_u64(self.registers[REGISTER_STACK_POINTER as usize]);
                    self.registers[REGISTER_STACK_POINTER as usize] += 8;
                }
                _ => return ControlFlow::Break(CpuStepError::InvalidMemoryInstruction),
            },

            // Control Flow
            0x3 => match instruction {
                // Halt
                0x0 => return ControlFlow::Break(CpuStepError::Halt),
                // Call
                0x1 => {
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
