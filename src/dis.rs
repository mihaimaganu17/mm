use crate::bytecode::{OpCode, Sequence};

#[derive(Default)]
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
        // Print offset of the instruction in the bytecode sequence
        print!("{:04} ", offset);
        // Print information about the source code line. If we are not at the first offset and
        // the current offset line is the same as a previous line, we print `|` to avoid noise.
        // Otherwise, we print the source line
        let line_token = if offset > 0 && sequence.line(offset) == sequence.line(offset - 1) {
            "   |".to_string()
        } else {
            format!("{:04}", sequence.line(offset))
        };
        print!("{} ", line_token);

        let instruction = sequence.code()[offset].into();
        match instruction {
            OpCode::Return => Instruction::simple("OP_RETURN", offset),
            OpCode::Constant => Instruction::constant("OP_CONSTANT", sequence, offset),
            OpCode::ConstantLong => {
                Instruction::constant_long("OP_CONSTANT_LONG", sequence, offset)
            }
            OpCode::Negate => Instruction::simple("OP_NEGATE", offset),
            OpCode::Add => Instruction::simple("OP_ADD", offset),
            OpCode::Subtract => Instruction::simple("OP_SUBTRACT", offset),
            OpCode::Multiply => Instruction::simple("OP_MULTIPLY", offset),
            OpCode::Divide => Instruction::simple("OP_DIVIDE", offset),
            OpCode::Unknown(byte) => {
                println!("Unknown opcode {}", byte);
                offset + 1
            }
        }
    }
}

pub struct Instruction;

impl Instruction {
    pub fn simple(name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }

    pub fn constant(name: &str, sequence: &Sequence, offset: usize) -> usize {
        // Get the constant index
        let constant_idx = sequence.code()[offset + 1];
        // Get thec constant, based on index
        let constant = sequence.constant(constant_idx as usize);
        println!("{name} {constant_idx} -> value: {constant}");
        offset + 2
    }

    pub fn constant_long(name: &str, sequence: &Sequence, offset: usize) -> usize {
        // Get the constant index which comprises of the next 3 bytes.
        let mut constant_idx = [0; 4];
        for (idx, byte) in sequence.code()[offset + 1..].iter().take(3).enumerate() {
            constant_idx[idx] = *byte;
        }
        let constant_idx = u32::from_le_bytes(constant_idx);
        // Get thec constant, based on index
        let constant = sequence.constant(constant_idx as usize);
        println!("{name} {constant_idx} -> value: {constant}");
        offset + 4
    }
}
