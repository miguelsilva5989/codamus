use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug)]
pub struct Assign<'a> {
    pub id: &'a str,
    pub constant: bool,
    pub expression: Box<Statement<'a>>,
}

impl Display for Assign<'_> {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        write!(format, "id: {}, constant: {}, expression: {}", self.id, self.constant, self.expression)
    }
}

#[derive(Debug)]
pub enum Statement<'a> {
    Comment(String),
    BooleanLiteral(bool),
    Identifier(Identifier),
    NumericLiteral(NumericLiteral),
    Declaration(Assign<'a>),
    Assign(Assign<'a>),
    ArithmeticExpression(ArithmeticExpression),
    CallExpression(CallExpression<'a>),
    ObjectLiteral(Object<'a>),
    Property(Property<'a>),
    MemberExpression(MemberExpression<'a>),
    // NoneLiteral,
    // UnaryExpression,
    // FunctionDeclaration,
}

impl Display for Statement<'_> {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        use self::Statement::*;
        match *self {
            Comment(ref val) => write!(format, "Comment: \t\t{}", val),
            BooleanLiteral(ref val) => write!(format, "Boolean Literal: \t{}", val),
            Identifier(ref val) => write!(format, "Identifier: \t{}", val),
            NumericLiteral(ref val) => write!(format, "Numeric Literal: \t{}", val),
            Declaration(ref assign) => write!(format, "Declaration: \t\t{}", assign),
            Assign(ref assign) => write!(format, "Assign: \t\t{}", assign),
            ArithmeticExpression(ref expr) => write!(format, "Arithmetic Expression:  {}", expr),
            CallExpression(ref call) => write!(format, "Call Expression: \t{}", call),
            ObjectLiteral(ref val) => write!(format, "Object Literal: \t{}", val),
            Property(ref val) => write!(format, "Property: \t\t{}", val),
            MemberExpression(ref val) => write!(format, "Member Expression: \t{}", val),
        }
    }
}

#[derive(Debug)]
pub enum Oper {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

pub enum ArithmeticExpression {
    Value(f64),
    Identifier(Identifier),
    Add(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Sub(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Mul(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Div(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Mod(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Paren(Box<ArithmeticExpression>),
}

impl Display for ArithmeticExpression {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        use self::ArithmeticExpression::*;
        match *self {
            Value(val) => write!(format, "{}", val),
            Identifier(ref val) => write!(format, "{}", val),
            Add(ref left, ref right) => write!(format, "{} + {}", left, right),
            Sub(ref left, ref right) => write!(format, "{} - {}", left, right),
            Mul(ref left, ref right) => write!(format, "{} * {}", left, right),
            Div(ref left, ref right) => write!(format, "{} / {}", left, right),
            Mod(ref left, ref right) => write!(format, "{} % {}", left, right),
            Paren(ref expr) => write!(format, "({})", expr),
        }
    }
}

impl Debug for ArithmeticExpression {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        use self::ArithmeticExpression::*;
        match *self {
            Value(val) => write!(format, "{}", val),
            Identifier(ref val) => write!(format, "{}", val),
            Add(ref left, ref right) => write!(format, "({:?} + {:?})", left, right),
            Sub(ref left, ref right) => write!(format, "({:?} - {:?})", left, right),
            Mul(ref left, ref right) => write!(format, "({:?} * {:?})", left, right),
            Div(ref left, ref right) => write!(format, "({:?} / {:?})", left, right),
            Mod(ref left, ref right) => write!(format, "({:?} % {:?})", left, right),
            Paren(ref expr) => write!(format, "[{:?}]", expr),
        }
    }
}

#[derive(Debug)]
pub struct CallExpression<'a> {
    pub func: &'a str,
    pub args: Vec<&'a str>,
}

impl Display for CallExpression<'_> {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        write!(format, "fn {}(", self.func)?;
        let mut i = 0;
        for v in &self.args {
            i += 1;
            write!(format, "{}", v)?;
            if &i < &self.args.len() {
                write!(format, ",")?;
            }
        }
        write!(format, ")")
    }
}

#[derive(Debug)]
pub struct MemberExpression<'a> {
    pub object: &'a str,
    pub property: &'a str,
}
impl Display for MemberExpression<'_> {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        write!(format, "object: {} - property: {}", self.object, self.property)
    }
}

#[derive(Debug)]
pub struct Property<'a> {
    pub key: &'a str,
    pub value: Option<Box<Statement<'a>>>,
}
impl Display for Property<'_> {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        if let Some(val) = &self.value {
            write!(format, "key: {} - value: {}", self.key, val)
        } else {
            write!(format, "key: {} - no value", self.key)
        }
    }
}

#[derive(Debug)]
pub struct Object<'a> {
    pub properties: Vec<Property<'a>>,
}
impl Display for Object<'_> {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        write!(format, "{{\n")?;
        for v in &self.properties {
            write!(format, "\t{}\n", v)?;
        }
        write!(format, "}}")
    }
}

pub struct Identifier {
    pub id: String,
}

impl Display for Identifier {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        write!(format, "{}", self.id)
    }
}

impl Debug for Identifier {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        write!(format, "{}", self.id)
    }
}

pub struct NumericLiteral {
    pub value: f64,
}

impl Display for NumericLiteral {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        write!(format, "{}", self.value)
    }
}

impl Debug for NumericLiteral {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        write!(format, "{}", self.value)
    }
}
