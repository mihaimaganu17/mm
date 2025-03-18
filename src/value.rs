use std::fmt;
use core::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone)]
pub struct Value(f32);

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
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
