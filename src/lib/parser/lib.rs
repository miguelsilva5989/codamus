// mod generic;
mod lexer;
mod ast;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_until1},
    character::complete::{alpha1, alphanumeric1, multispace0, space0},
    combinator::recognize,
    multi::{many0_count, many1},
    sequence::{pair, tuple},
    IResult,
};

use lexer::{Token, TokenType};
use ast::{Assign, BinaryExpression, CallExpression, Expression, Identifier, NumericLiteral};

#[derive(Debug)]
pub struct Program<'a> {
    pub body: Vec<Expression<'a>>,
}

fn get_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(alt((alpha1, tag("_"))), many0_count(alt((alphanumeric1, tag("_"))))))(input)
}

fn take_until_closing_bracket(opening_bracket: &str, closing_bracket: &str, tokens: &mut Vec<Token>) -> Vec<Token> {
    let mut close_pos = 0;
    let mut counter = 1;
    while counter > 0 {
        if let Some(c) = tokens.get(close_pos) {
            if c.value == opening_bracket {
                counter += 1;
            } else if c.value == closing_bracket {
                counter -= 1;
            }
            close_pos += 1;
        } else {
            break;
        }
    }

    if tokens.get(close_pos - 1).unwrap().value != closing_bracket {
        panic!("Expected closing bracked '{}' but not found in '{:?}'", closing_bracket, tokens);
    }

    // println!("{:?}", tokens[1..close_pos-1].to_vec());
    return tokens[1..close_pos - 1].to_vec();
}

fn parse_primary_expression<'a>(tokens: &mut Vec<Token>) -> Option<Expression<'a>> {
    if let Some(token) = tokens.get(0) {
        match token.r#type {
            TokenType::Identifier => {
                let expr = Some(Expression::Identifier(Identifier { id: token.value.clone() }));
                tokens.remove(0);
                return expr;
            }
            TokenType::None => {
                tokens.remove(0);
                return Some(Expression::NoneLiteral);
            }
            TokenType::Number => {
                let expr = Some(Expression::NumericLiteral(NumericLiteral {
                    value: token
                        .value
                        .parse::<usize>()
                        .expect(format!("Unexpected number {}", token.value).as_str()),
                }));
                tokens.remove(0);
                return expr;
            }
            TokenType::OpenParen => {
                let mut paren_tokens = take_until_closing_bracket("(", ")", tokens);
                // println!("{:?}", paren_tokens);

                return Some(parse_expression(&mut paren_tokens));
            }
            _ => panic!("Unexpected token while parsing primary expression '{:?}'", token),
        }
    }
    None
}

fn parse_multiplicative_expression<'a>(tokens: &mut Vec<Token>) -> Expression<'a> {
    let mut left = parse_primary_expression(tokens);

    if let Some(token) = tokens.get(0) {
        if token.value == "/" || token.value == "*" || token.value == "%" {
            let operator = token.value.clone();
            tokens.remove(0);
            let right = parse_primary_expression(tokens);

            left = Some(Expression::BinaryExpression(BinaryExpression {
                left: Box::new(left.unwrap()),
                right: Box::new(right.unwrap()),
                operator,
            }));
        }
    }

    return left.unwrap();
}

fn parse_additive_expression<'a>(tokens: &mut Vec<Token>) -> Expression<'a> {
    let mut left = parse_multiplicative_expression(tokens);

    if let Some(token) = tokens.get(0) {
        if token.value == "+" || token.value == "-" {
            let operator = token.value.clone();
            tokens.remove(0);
            let right = parse_multiplicative_expression(tokens);

            left = Expression::BinaryExpression(BinaryExpression {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            });
        }
    }

    return left;
}

fn parse_expression<'a>(tokens: &mut Vec<Token>) -> Expression<'a> {
    return parse_additive_expression(tokens);
}

fn parse_assign(input: &str) -> IResult<&str, Expression> {
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

    let mut tokens = lexer::tokenzine(assign);

    let expression = parse_expression(&mut tokens);

    Ok((
        input,
        Expression::Assign(Assign {
            id,
            expression: Box::new(expression),
        }),
    ))
}

fn parse_call_expression(input: &str) -> IResult<&str, Expression> {
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

    Ok((
        input,
        Expression::CallExpression(CallExpression {
            func,
            args: args.trim().split(",").into_iter().map(|x| x.trim()).collect(),
        }),
    ))
}

fn parse_comment(input: &str) -> IResult<&str, Expression> {
    let (input, (_, _, comment)) = tuple((space0, tag("//"), take_till(|c| c == '\n' || c == '\r')))(input)?;

    Ok((input, Expression::Comment(comment.trim().to_owned())))
}

fn parse_program(input: &str) -> IResult<&str, Vec<Expression>> {
    let (input, program) = many1(alt((parse_comment, parse_assign, parse_call_expression)))(input)?;

    let statements: Vec<Expression> = program;

    Ok((input, statements))
}

pub fn parse_ast(input: &str) -> IResult<&str, Program> {
    let (input, statements) = parse_program(input)?;

    let program = Program { body: statements };

    Ok((input, program))
}
