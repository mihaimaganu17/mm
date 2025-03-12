mod bytecode;
mod dis;

pub use bytecode::{Sequence, OpCode};
pub use dis::Disassembler;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_dis() {
        let mut seq = Sequence::new();
        seq.push(0);
        Disassembler::dis_sequence(&seq, "test sequence" );
    }
}
