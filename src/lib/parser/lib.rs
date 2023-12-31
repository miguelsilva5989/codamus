mod arithmetic;
pub mod ast;
mod generic;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{digit1, multispace0, space0},
    multi::many1,
    sequence::tuple,
    IResult,
};
use std::fmt::{self, Debug, Display, Formatter};

use arithmetic::parse_arithmetic_expression;
use ast::{Assign, CallExpression, NumericLiteral, Statement, Identifier};
// use lexer::{Token, TokenType};

#[derive(Debug)]
pub struct Program<'a> {
    pub body: Vec<Statement<'a>>,
}

impl Display for Program<'_> {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        write!(format, "Program:\n")?;
        for v in &self.body {
            write!(format, "\t{}\n", v)?;
        }
        Ok(())
    }
}

fn parse_comment(input: &str) -> IResult<&str, Statement> {
    let (input, (_, _, comment)) = tuple((space0, tag("//"), take_till(|c| c == '\n' || c == '\r')))(input)?;

    Ok((input, Statement::Comment(comment.trim().to_owned())))
}

fn parse_identifier(input: &str) -> IResult<&str, Statement> {
    let (input, (_, id, _, _, _)) = tuple((multispace0, generic::get_identifier, multispace0, tag(";"), multispace0))(input)?;

    Ok((input, Statement::Identifier(Identifier { id: id.to_owned() })))
}

fn parse_boolean(input: &str) -> IResult<&str, Statement> {
    let (input, boolean) = alt((tag("true"), tag("false")))(input)?;

    Ok((input, Statement::BooleanLiteral(boolean.parse::<bool>().unwrap())))
}

fn parse_boolean_literal(input: &str) -> IResult<&str, Statement> {
    let (input, (_, boolean, _, _, _)) = tuple((multispace0, alt((tag("true"), tag("false"))), multispace0, tag(";"), multispace0))(input)?;

    Ok((input, Statement::BooleanLiteral(boolean.parse::<bool>().unwrap())))
}

fn parse_numeric_literal(input: &str) -> IResult<&str, Statement> {
    let (input, (_, num, _, _, _)) = tuple((multispace0, digit1, multispace0, tag(";"), multispace0))(input)?;

    Ok((
        input,
        Statement::NumericLiteral(NumericLiteral {
            value: num.parse::<f64>().unwrap(),
        }),
    ))
}

fn parse_assign(input: &str) -> IResult<&str, Statement> {
    let (input, (_, _, _, id, _, _, _, assign, _, _, _)) = tuple((
        multispace0,
        tag("let"),
        multispace0,
        generic::get_identifier,
        multispace0,
        tag("="),
        multispace0,
        take_until(";"),
        multispace0,
        tag(";"),
        multispace0,
    ))(input)?;

    println!("{}", assign);
    let (_, expression) = alt((parse_boolean, parse_arithmetic_expression_to_expr))(assign)?;
    println!("{}", expression);

    Ok((
        input,
        Statement::Assign(Assign {
            id,
            expression: Box::new(expression),
        }),
    ))
}

fn parse_call_expression(input: &str) -> IResult<&str, Statement> {
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
        Statement::CallExpression(CallExpression {
            func,
            args: args.trim().split(",").into_iter().map(|x| x.trim()).collect(),
        }),
    ))
}

fn parse_arithmetic_expression_to_expr(input: &str) -> IResult<&str, Statement> {
    let (input, parsed) = parse_arithmetic_expression(input)?;

    Ok((input, Statement::ArithmeticExpression(parsed)))
}

fn parse_program(input: &str) -> IResult<&str, Vec<Statement>> {
    let (input, program) = many1(alt((
        parse_comment,
        parse_boolean_literal,
        parse_numeric_literal,
        parse_identifier,
        parse_assign,
        parse_call_expression,
        parse_arithmetic_expression_to_expr,
    )))(input)?;

    let statements: Vec<Statement> = program;

    Ok((input, statements))
}

pub fn parse_ast(input: &str) -> IResult<&str, Program> {
    let (input, statements) = parse_program(input)?;

    let program = Program { body: statements };

    Ok((input, program))
}
