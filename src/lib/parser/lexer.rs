
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
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub enum NodeType<'a> {
    Identifier(Identifier),
    NumericLiteral(NumericLiteral),
    Assign(Assign<'a>),
    BinaryExpression(BinaryExpression),
    CallExpression,
    UnaryExpression,
    FunctionDeclaration,
}

#[derive(Debug)]
struct Expression {}

#[derive(Debug)]
struct BinaryExpression {
    left: Expression,
    right: Expression,
    operator: String,
}

#[derive(Debug)]
struct Identifier {
    id: String,
}

#[derive(Debug)]
struct NumericLiteral {
    value: usize,
}

