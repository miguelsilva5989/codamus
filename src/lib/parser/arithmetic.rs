use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::{digit1 as digit, multispace0 as multispace},
    combinator::{map, map_res},
    multi::many0,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use std::str::FromStr;

use crate::ast::Identifier;

use super::ast::{ArithmeticExpression, Oper};
use super::generic::get_identifier;

fn parens(i: &str) -> IResult<&str, ArithmeticExpression> {
    delimited(
        multispace,
        delimited(
            tag("("),
            map(parse_arithmetic_expression, |e| ArithmeticExpression::Paren(Box::new(e))),
            tag(")"),
        ),
        permutation((multispace, many0(tag(";")), multispace)),
    )
    .parse(i)
}

fn factor(i: &str) -> IResult<&str, ArithmeticExpression> {
    alt((
        // numbers
        map(
            map_res(
                delimited(multispace, digit, permutation((multispace, many0(tag(";")), multispace))),
                FromStr::from_str,
            ),
            ArithmeticExpression::Value,
        ),
        // identifiers
        map(
            delimited(multispace, get_identifier, permutation((multispace, many0(tag(";")), multispace))),
            |s: &str| ArithmeticExpression::Identifier(Identifier { id: s.to_owned() }),
        ),
        parens,
    ))
    .parse(i)
}

fn fold_exprs(initial: ArithmeticExpression, remainder: Vec<(Oper, ArithmeticExpression)>) -> ArithmeticExpression {
    remainder.into_iter().fold(initial, |acc, pair| {
        let (oper, expr) = pair;
        match oper {
            Oper::Add => ArithmeticExpression::Add(Box::new(acc), Box::new(expr)),
            Oper::Sub => ArithmeticExpression::Sub(Box::new(acc), Box::new(expr)),
            Oper::Mul => ArithmeticExpression::Mul(Box::new(acc), Box::new(expr)),
            Oper::Div => ArithmeticExpression::Div(Box::new(acc), Box::new(expr)),
            Oper::Mod => ArithmeticExpression::Mod(Box::new(acc), Box::new(expr)),
        }
    })
}

fn term(i: &str) -> IResult<&str, ArithmeticExpression> {
    let (i, initial) = factor(i)?;
    let (i, remainder) = many0(alt((
        |i| {
            let (i, mul) = preceded(tag("*"), factor).parse(i)?;
            Ok((i, (Oper::Mul, mul)))
        },
        |i| {
            let (i, div) = preceded(tag("/"), factor).parse(i)?;
            Ok((i, (Oper::Div, div)))
        },
        |i| {
            let (i, module) = preceded(tag("%"), factor).parse(i)?;
            Ok((i, (Oper::Mod, module)))
        },
    )))
    .parse(i)?;

    Ok((i, fold_exprs(initial, remainder)))
}

pub fn parse_arithmetic_expression(input: &str) -> IResult<&str, ArithmeticExpression> {
    let (i, initial) = term(input)?;

    let (i, remainder) = many0(alt((
        |i| {
            let (i, add) = preceded(tag("+"), term).parse(i)?;
            Ok((i, (Oper::Add, add)))
        },
        |i| {
            let (i, sub) = preceded(tag("-"), term).parse(i)?;
            Ok((i, (Oper::Sub, sub)))
        },
    )))
    .parse(i)?;

    Ok((i, fold_exprs(initial, remainder)))
}
