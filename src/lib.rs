mod bytecode;
mod dis;
mod value;
mod alloc;

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
        // Push the operand for the instruction
        seq.write_constant(constant, 17);
        // Push return
        seq.push(OpCode::Return, 13).unwrap();
        // Create a new constant
        let constant = Value::from(13.9);
        // Push the operand for the instruction
        seq.write_constant(constant, 13);
        // Push return
        seq.push(OpCode::Return, 17).unwrap();
        seq.push(OpCode::Return, 17).unwrap();
        seq.push(OpCode::Return, 17).unwrap();
        seq.push(OpCode::Return, 14).unwrap();
        seq.push(OpCode::Return, 14).unwrap();
        // Check if constant long works
        for i in 0..300u16 {
            // Create a new constant
            let constant = Value::from(f32::from(i));
            // Push the operand for the instruction
            seq.write_constant(constant, 13);
        }
        Disassembler::dis_sequence(&seq, "test sequence");
    }
}
