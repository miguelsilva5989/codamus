#[derive(Debug)]
pub struct Assign<'a> {
    pub id: &'a str,
    // pub tokens: Vec<Token>,
    pub expression: Box<Expression<'a>>
}

#[derive(Debug)]
pub enum Expression<'a> {
    Comment(String),
    Identifier(Identifier),
    NumericLiteral(NumericLiteral),
    Assign(Assign<'a>),
    ArithmeticExpression(ArithmeticExpression),
    CallExpression(CallExpression<'a>),
    NoneLiteral
    // UnaryExpression,
    // FunctionDeclaration,
}

#[derive(Debug)]
pub enum Oper {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
}

// #[derive(Debug)]
// pub struct ArithmeticExpression<'a> {
//     pub left: Box<Expression<'a>>,
//     pub right: Box<Expression<'a>>,
//     pub operator: String,
// }

#[derive(Debug)]
pub enum ArithmeticExpression {
    Value(i64),
    Identifier(Identifier),
    Add(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Sub(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Mul(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Div(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Mod(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Paren(Box<ArithmeticExpression>),
  }

#[derive(Debug)]
pub struct CallExpression<'a> {
    pub func: &'a str,
    pub args: Vec<&'a str>
}

#[derive(Debug)]
pub struct Identifier {
    pub id: String,
}

#[derive(Debug)]
pub struct NumericLiteral {
    pub value: usize,
}