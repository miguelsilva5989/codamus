
// const RESERVED: [&str; 1] = ["let"];

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Number,
    Identifier,
    OpenParen,
    CloseParen,
    BinaryOperator,
}

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub r#type: TokenType,
}

impl Token {
    pub fn new(value: String, r#type: TokenType) -> Self {
        Self { value, r#type }
    }
}

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
    BinaryExpression(BinaryExpression<'a>),
    CallExpression(CallExpression<'a>),
    // UnaryExpression,
    // FunctionDeclaration,
}

// #[derive(Debug)]
// struct Expression {}

#[derive(Debug)]
pub struct BinaryExpression<'a> {
    pub left: Box<Expression<'a>>,
    pub right: Box<Expression<'a>>,
    pub operator: String,
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

