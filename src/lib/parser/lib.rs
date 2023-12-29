mod generic;
mod lexer;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1, take_until},
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::recognize,
    multi::{many0_count, many1},
    sequence::{pair, tuple, delimited},
    IResult,
};

use lexer::{NodeType, Assign, Token, TokenType, CallExpression};

#[derive(Debug)]
pub struct Program<'a> {
    body: Vec<NodeType<'a>>,
}

fn get_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(alt((alpha1, tag("_"))), many0_count(alt((alphanumeric1, tag("_"))))))(input)
}

fn parse_assign(input: &str) -> IResult<&str, NodeType> {
    let (input, (_, _, _, id, _, _, _, assign, _, _, _)) = tuple((
        multispace0,
        tag("let"),
        multispace0,
        get_identifier,
        multispace0,
        tag("="),
        multispace0,
        take_until1(";"),
        multispace0,
        tag(";"),
        multispace0,
    ))(input)?;

    let tokens = tokenzine(assign);

    Ok((input, NodeType::Assign(Assign { id, tokens })))
}

fn parse_call_expression(input: &str) -> IResult<&str, NodeType> {
    let (input, (_, func, _, _, _, args, _, _, _, _)) = tuple((
        multispace0,
        tag("print"),
        multispace0,
        tag("("),
        multispace0,
        take_until(")"),
        tag(")"),
        multispace0,
        tag(";"),
        multispace0,
    ))(input)?;

    Ok((input, NodeType::CallExpression(CallExpression {func, args: args.trim().split(",").collect()})))
}

fn parse_program(input: &str) -> IResult<&str, Vec<NodeType>> {
    let (input, program) = many1(alt((parse_assign, parse_call_expression)))(input)?;

    let statements: Vec<NodeType> = program;

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

pub fn parse_ast(input: &str) -> IResult<&str, Program> {
    let (input, statements) = parse_program(input)?;

    let program = Program { body: statements };

    program.body.iter().for_each(|st| {
        
    });

    Ok((input, program))
}
