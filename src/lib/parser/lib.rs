mod arithmetic;
mod ast;
mod generic;
// mod lexer;

use std::fmt::{self, Debug, Display, Formatter};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{multispace0, space0},
    multi::many1,
    sequence::tuple,
    IResult,
};

use arithmetic::parse_arithmetic_expression;
use ast::{Assign, CallExpression, Expression};
// use lexer::{Token, TokenType};

#[derive(Debug)]
pub struct Program<'a> {
    pub body: Vec<Expression<'a>>,
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

fn parse_comment(input: &str) -> IResult<&str, Expression> {
    let (input, (_, _, comment)) = tuple((space0, tag("//"), take_till(|c| c == '\n' || c == '\r')))(input)?;

    Ok((input, Expression::Comment(comment.trim().to_owned())))
}

fn parse_assign(input: &str) -> IResult<&str, Expression> {
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

    let (_, expression) = parse_arithmetic_expression_to_expr(assign)?;

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

fn parse_arithmetic_expression_to_expr(input: &str) -> IResult<&str, Expression> {
    let (input, parsed) = parse_arithmetic_expression(input)?;

    Ok((input, Expression::ArithmeticExpression(parsed)))
}

fn parse_program(input: &str) -> IResult<&str, Vec<Expression>> {
    let (input, program) = many1(alt((
        parse_comment,
        parse_assign,
        parse_call_expression,
        parse_arithmetic_expression_to_expr,
    )))(input)?;

    let statements: Vec<Expression> = program;

    Ok((input, statements))
}

pub fn parse_ast(input: &str) -> IResult<&str, Program> {
    let (input, statements) = parse_program(input)?;

    let program = Program { body: statements };

    Ok((input, program))
}
