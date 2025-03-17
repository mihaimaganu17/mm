use crate::Sequence;

pub struct VM<'vm> {
    // Sequence of bytecode that the VM executes
    sequence: &'vm Sequence,
    // Offset to the byte opcode that needs executing
    offset: usize,
}

impl<'vm> VM<'vm> {
    fn new(sequence: &'vm Sequence) -> Self {
        Self {
            sequence,
            offset: 0,
        }
    }
}
