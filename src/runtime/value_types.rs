use std::{ops::{Add, Div, Mul, Rem, Sub}, collections::BTreeMap};

use super::RuntimeValue;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    None,
    Number(f64),
    Bool(bool),
    Object(BTreeMap<String, RuntimeValue>),
}
impl Into<f64> for ValueType {
    fn into(self) -> f64 {
        match self {
            ValueType::None => 0.0,
            ValueType::Number(x) => x,
            ValueType::Bool(_) => panic!("Bool cannot be cast into f64"),
            _ => todo!()
        }
    }
}

impl Add for ValueType {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            ValueType::None => ValueType::None,
            ValueType::Number(x) => ValueType::Number(x + <ValueType as Into<f64>>::into(other)),
            ValueType::Bool(val) => panic!("Cannot Add bool '{val}' with number"),
            _ => todo!()
        }
    }
}
impl Sub for ValueType {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match self {
            ValueType::None => ValueType::None,
            ValueType::Number(x) => ValueType::Number(x - <ValueType as Into<f64>>::into(other)),
            ValueType::Bool(val) => panic!("Cannot Subract bool '{val}' with number"),
            _ => todo!()
        }
    }
}
impl Mul for ValueType {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match self {
            ValueType::None => ValueType::None,
            ValueType::Number(x) => ValueType::Number(x * <ValueType as Into<f64>>::into(other)),
            ValueType::Bool(val) => panic!("Cannot Multiply bool '{val}' with number"),
            _ => todo!()
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
            ValueType::Bool(val) => panic!("Cannot Divide bool '{val}' with number"),
            _ => todo!()
        }
    }
}
impl Rem for ValueType {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        match self {
            ValueType::None => ValueType::None,
            ValueType::Number(x) => ValueType::Number(x % <ValueType as Into<f64>>::into(other)),
            ValueType::Bool(val) => panic!("Cannot Mod bool '{val}' with number"),
            _ => todo!()
        }
    }
}
