use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    None,
    Number(f64),
}
impl Into<f64> for ValueType {
    fn into(self) -> f64 {
        match self {
            ValueType::None => 0.0,
            ValueType::Number(x) => x,
        }
    }
}

impl Add for ValueType {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            ValueType::None => ValueType::None,
            ValueType::Number(x) => ValueType::Number(x + <ValueType as Into<f64>>::into(other)),
        }
    }
}
impl Sub for ValueType {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match self {
            ValueType::None => ValueType::None,
            ValueType::Number(x) => ValueType::Number(x - <ValueType as Into<f64>>::into(other)),
        }
    }
}
impl Mul for ValueType {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match self {
            ValueType::None => ValueType::None,
            ValueType::Number(x) => ValueType::Number(x * <ValueType as Into<f64>>::into(other)),
        }
    }
}
impl Div for ValueType {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let other_val = <ValueType as Into<f64>>::into(other);
        if other_val == 0.0 {
            panic!("Cannot divide by 0")
        }
        match self {
            ValueType::None => ValueType::None,
            ValueType::Number(x) => ValueType::Number(x / other_val),
        }
    }
}
impl Rem for ValueType {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        match self {
            ValueType::None => ValueType::None,
            ValueType::Number(x) => ValueType::Number(x % <ValueType as Into<f64>>::into(other)),
        }
    }
}
