mod bytecode;
mod dis;
mod value;

pub use bytecode::{OpCode, Sequence};
pub use dis::Disassembler;
pub use value::Value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_dis() {
        let mut seq = Sequence::new();
        // Create a new constant
        let constant = Value::from(1.2);
        // Add a new constant to the constants' storage
        let constant_idx = seq.add_constant(constant);
        // Push the new instruction
        seq.push(OpCode::Constant, 17).unwrap();
        // Push the operand for the instruction
        seq.push(constant_idx, 17).unwrap();
        // Push return
        seq.push(OpCode::Return, 13).unwrap();
        seq.push(OpCode::Constant, 13).unwrap();
        // Push the operand for the instruction
        seq.push(constant_idx, 13).unwrap();
        // Push return
        seq.push(OpCode::Return, 17).unwrap();
        seq.push(OpCode::Return, 17).unwrap();
        seq.push(OpCode::Return, 17).unwrap();
        seq.push(OpCode::Return, 14).unwrap();
        seq.push(OpCode::Return, 14).unwrap();
        Disassembler::dis_sequence(&seq, "test sequence");
    }
}
