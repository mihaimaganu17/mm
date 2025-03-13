pub struct Value(f32);

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
