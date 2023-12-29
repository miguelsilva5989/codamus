use std::marker::PhantomData;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::recognize,
    multi::many0_count,
    sequence::{pair, tuple},
    IResult,
};

mod generic;

// const RESERVED: [&str; 1] = ["let"];

#[derive(Debug, PartialEq)]
enum TokenType {
    Number,
    Identifier,
    OpenParen,
    CloseParen,
    BinaryOperator,
}

#[derive(Debug)]
struct Token {
    value: String,
    r#type: TokenType,
}

impl Token {
    fn new(value: String, r#type: TokenType) -> Self {
        Self { value, r#type }
    }
}

#[derive(Debug)]
struct Assign<'a> {
    id: &'a str,
    tokens: Vec<Token>,
}

#[derive(Debug)]
enum NodeType<'a> {
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
    operator: String
}

#[derive(Debug)]
struct Identifier {
    id: String
}

#[derive(Debug)]
struct NumericLiteral {
    value: usize
}

#[derive(Debug)]
pub struct Program<'a> {
    statements: Vec<NodeType<'a>>,
}

fn get_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(alt((alpha1, tag("_"))), many0_count(alt((alphanumeric1, tag("_"))))))(input)
}

fn parse_assign(input: &str) -> IResult<&str, NodeType> {
    let (input, (_, _, id, _, _, _, assign, _, _)) = tuple((
        tag("let"),
        multispace0,
        get_identifier,
        multispace0,
        tag("="),
        multispace0,
        take_until1(";"),
        multispace0,
        tag(";"),
    ))(input)?;

    let tokens = tokenzine(assign);

    Ok((input, NodeType::Assign(Assign { id, tokens })))
}

fn parse_statements(input: &str) -> IResult<&str, Vec<NodeType>> {
    let (input, assign) = parse_assign(input)?;

    // let actions = Dsl::new(file, transform);

    let statements: Vec<NodeType> = vec![assign];

    Ok((input, statements))
}

fn tokenzine(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = input.chars().into_iter().peekable();

    while let Some(ch) = iter.next() {
        match ch {
            '(' => tokens.push(Token::new(ch.into(), TokenType::OpenParen)),
            ')' => tokens.push(Token::new(ch.into(), TokenType::CloseParen)),
            '+' | '-' | '*' | '/' => tokens.push(Token::new(ch.into(), TokenType::BinaryOperator)),
            _ => {
                if ch == ' ' || ch == '\n' || ch == '\r' || ch == '\t' {
                    // ignore whitespaces
                } else if ch.is_numeric() {
                    let mut num: String = ch.into();
                    while let Some(next) = iter.peek() {
                        if next.is_numeric() {
                            num.push(iter.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::new(num, TokenType::Number));
                } else if ch.is_alphabetic() {
                    let mut id: String = ch.into();
                    while let Some(next) = iter.peek() {
                        if next.is_alphanumeric() || next == &'_' {
                            id.push(iter.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    tokens.push(Token::new(id, TokenType::Identifier));
                } else {
                    panic!("Token '{}' is not yet implemented", ch);
                }
            }
        }
    }

    tokens
}

pub fn parse_dsl(input: &str) -> IResult<&str, Program> {
    let (input, statements) = parse_statements(input)?;

    let dsl = Program { statements };

    Ok((input, dsl))
}
