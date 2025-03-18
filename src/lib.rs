mod alloc;
mod bytecode;
mod dis;
mod value;
mod vm;

pub use bytecode::{OpCode, Sequence};
pub use dis::Disassembler;
pub use value::Value;
pub use vm::VM;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_dis() {
        let mut seq = Sequence::new();
        // Create a new constant
        let constant = Value::from(1.2);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push return
        // seq.push(OpCode::Return, 13).unwrap();
        // Create a new constant
        let constant = Value::from(13.9);
        // Push the operand for the instruction
        seq.write_constant(constant, 13).unwrap();
        // Push return
        seq.push(OpCode::Return, 17).unwrap();
        seq.push(OpCode::Return, 17).unwrap();
        seq.push(OpCode::Return, 17).unwrap();
        seq.push(OpCode::Return, 14).unwrap();
        seq.push(OpCode::Return, 14).unwrap();
        // Create a new VM that will execute code
        let mut vm = VM::new(&seq);
        vm.interpret(&seq).unwrap();
        /*
        // Check if constant long works
        for i in 0..300u16 {
            // Create a new constant
            let constant = Value::from(f32::from(i));
            // Push the operand for the instruction
            seq.write_constant(constant, 13);
        }*/
        let mut seq2 = Sequence::new();
        // Create a new constant
        let constant = Value::from(1.2);
        // Push the operand for the instruction
        seq2.write_constant(constant, 99).unwrap();
        // Push return
        seq2.push(OpCode::Return, 1119).unwrap();
        // Create a new constant
        let constant = Value::from(1119.9);
        // Push the operand for the instruction
        seq2.write_constant(constant, 1119).unwrap();
        // Push return
        seq2.push(OpCode::Return, 99).unwrap();
        seq2.push(OpCode::Return, 99).unwrap();
        seq2.push(OpCode::Return, 99).unwrap();
        seq2.push(OpCode::Return, 14).unwrap();
        seq2.push(OpCode::Return, 14).unwrap();
        Disassembler::dis_sequence(&seq, "test sequence");
        Disassembler::dis_sequence(&seq2, "test sequence2");
    }

    #[test]
    fn negate() {
        let mut seq = Sequence::new();
        let constant = Value::from(1.2);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push negation
        seq.push(OpCode::Negate, 17).unwrap();
        // Push return
        seq.push(OpCode::Return, 18).unwrap();
        // Create a new VM that will execute code
        let mut vm = VM::new(&seq);
        vm.interpret(&seq).unwrap();
    }

    #[test]
    fn add() {
        let mut seq = Sequence::new();
        let constant = Value::from(90.0);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        let constant = Value::from(10.0);
        // Push the operand for the instruction
        seq.write_constant(constant, 17).unwrap();
        // Push addition
        seq.push(OpCode::Add, 17).unwrap();
        // Push return
        seq.push(OpCode::Return, 18).unwrap();
        // Create a new VM that will execute code
        let mut vm = VM::new(&seq);
        vm.interpret(&seq).unwrap();
    }
}
