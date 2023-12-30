use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, PartialEq)]
pub enum ValueType {
    None,
    Number(usize),
}
impl Into<usize> for ValueType {
    fn into(self) -> usize {
        match self {
            ValueType::None => 0,
            ValueType::Number(x) => x,
        }
    }
}

impl Add for ValueType {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            ValueType::None => ValueType::None,
            ValueType::Number(x) => ValueType::Number(x + <ValueType as Into<usize>>::into(other)),
        }
    }
}
impl Sub for ValueType {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match self {
            ValueType::None => ValueType::None,
            ValueType::Number(x) => ValueType::Number(x - <ValueType as Into<usize>>::into(other)),
        }
    }
}
impl Mul for ValueType {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match self {
            ValueType::None => ValueType::None,
            ValueType::Number(x) => ValueType::Number(x * <ValueType as Into<usize>>::into(other)),
        }
    }
}
impl Div for ValueType {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        todo!("check division by 0");
        match self {
            ValueType::None => ValueType::None,
            ValueType::Number(x) => ValueType::Number(x / <ValueType as Into<usize>>::into(other)),
        }
    }
}
impl Rem for ValueType {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        match self {
            ValueType::None => ValueType::None,
            ValueType::Number(x) => ValueType::Number(x % <ValueType as Into<usize>>::into(other)),
        }
    }
}
