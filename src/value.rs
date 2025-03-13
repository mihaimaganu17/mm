pub struct Value(f32);

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Default)]
pub struct ValueVec(pub Vec<Value>);

impl ValueVec {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, value: Value) {
        self.0.push(value)
    }

    pub fn values(&self) -> &[Value] {
        self.0.as_slice()
    }
}
