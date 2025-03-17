use std::fmt;
use crate::Sequence;

#[derive(Debug)]
pub struct Value(f32);

impl Value {
    pub fn read_constant(sequence: &[u8])-> Self {
        let value = f32::from(sequence[0]);
        Value(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Default)]
pub struct ValueVec(pub Vec<Value>);

impl ValueVec {
    pub fn push(&mut self, value: Value) {
        self.0.push(value)
    }

    pub fn values(&self) -> &[Value] {
        self.0.as_slice()
    }
}
