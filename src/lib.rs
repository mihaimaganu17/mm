mod bytecode;
mod dis;

pub use bytecode::{Sequence, OpCode};

#[cfg(test)]
mod tests {
    use super::*;
    use dis::Disassembler;

    #[test]
    fn debug_dis() {
        let mut seq = Sequence::new();
        seq.push(0);
        Disassembler::dis_sequence(&seq, "test sequence" );
    }
}
