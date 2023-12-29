mod generic;
mod lexer;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_until1},
    character::complete::{alpha1, alphanumeric1, multispace0, space0},
    combinator::recognize,
    multi::{many0_count, many1},
    sequence::{pair, tuple},
    IResult,
};

use lexer::{Assign, BinaryExpression, CallExpression, Expression, Identifier, NumericLiteral, Token, TokenType};

#[derive(Debug)]
pub struct Program<'a> {
    pub body: Vec<Expression<'a>>,
}

fn get_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(alt((alpha1, tag("_"))), many0_count(alt((alphanumeric1, tag("_"))))))(input)
}

fn parse_additive_expression<'a>(tokens: &Vec<Token>) -> Expression<'a> {
    let mut iter = tokens.into_iter();

    let mut left = parse_primary_expression(iter.next());

    while let Some(token) = iter.next() {
        if token.value == "+" || token.value == "-" {
            let operator = token.value.clone();
            let right = parse_primary_expression(iter.next());

            left = Some(Expression::BinaryExpression(BinaryExpression {
                left: Box::new(left.unwrap()),
                right: Box::new(right.unwrap()),
                operator,
            }));
        }
    }

    return left.unwrap();
}

fn parse_expression<'a>(tokens: &Vec<Token>) -> Expression<'a> {
    return parse_additive_expression(tokens);
}

fn parse_primary_expression<'a>(token: Option<&Token>) -> Option<Expression<'a>> {
    if let Some(token) = token {
        println!("token {:?}", token);
        match token.r#type {
            TokenType::Identifier => return Some(Expression::Identifier(Identifier { id: token.value.clone() })),
            TokenType::Number => {
                return Some(Expression::NumericLiteral(NumericLiteral {
                    value: token
                        .value
                        .parse::<usize>()
                        .expect(format!("Unexpected number {}", token.value).as_str()),
                }))
            }
            _ => todo!("Unexpected token while parsing primary expression '{:?}'", token),
        }
    }
    None
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

    let tokens = tokenzine(assign);

    let expression = parse_expression(&tokens);

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
