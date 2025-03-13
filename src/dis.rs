use crate::bytecode::{OpCode, Sequence};

pub struct Disassembler;

impl Disassembler {
    pub fn new() -> Self {
        Self
    }
    pub fn dis_sequence(sequence: &Sequence, name: &str) {
        println!("== {} ==", name);
        // Start at the beginning of the sequence
        let mut offset = 0;
        // While we still have bytes
        while offset < sequence.code().len() {
            // Disassemble an instruction and move the cursor to the first byte after that
            // instruction
            offset = Self::dis_instruction(sequence, offset);
        }
    }

    pub fn dis_instruction(sequence: &Sequence, offset: usize) -> usize {
        print!("{:04} ", offset);

        let instruction = sequence.code()[offset].into();
        let offset = match instruction {
            OpCode::Return => Instruction::simple("OP_RETURN", offset),
            OpCode::Constant => Instruction::constant("OP_CONSTANT", sequence, offset),
            OpCode::Unknown(byte) => {
                println!("Unknown opcode {}", byte);
                offset + 1
            }
        };

        offset
    }
}

pub struct Instruction;

impl Instruction {
    pub fn simple(name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }

    pub fn constant(name: &str, sequence: &Sequence, offset: usize) -> usize {
        println!("{name} {}", sequence.code()[offset+1]);
        offset + 2
    }
}
