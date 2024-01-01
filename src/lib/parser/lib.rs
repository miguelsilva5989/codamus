mod arithmetic;
pub mod ast;
mod generic;

use generic::get_identifier;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{digit1, multispace0, space0},
    combinator::opt,
    multi::{many0, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};
use std::fmt::{self, Debug, Display, Formatter};

use arithmetic::parse_arithmetic_expression;
use ast::{Assign, CallExpression, Identifier, MemberExpression, NumericLiteral, Object, Property, Statement};
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

fn parse_call_member_expression(input: &str) -> IResult<&str, Statement> {
    let (input, (object, property)) = tuple((
        generic::get_identifier,
        delimited(tag("["), delimited(tag("\""), get_identifier, tag("\"")), tag("]")),
    ))(input)?;

    Ok((input, Statement::MemberExpression(MemberExpression { object, property })))
}

fn parse_call_member_expression_literal(input: &str) -> IResult<&str, Statement> {
    let (input, (_, stat, _, _, _)) = tuple((multispace0, parse_call_member_expression, multispace0, tag(";"), multispace0))(input)?;

    Ok((input, stat))
}

// fn parse_member_expression(input: &str) -> IResult<&str, Statement> {
//     let (_, members) = separated_list1(tag("."), get_identifier)(input)?;

//     Ok((input, Statement::MemberExpression(MemberExpression { object: "", property: "" })))
// }

// fn parse_member_expression_literal(input: &str) -> IResult<&str, Statement> {
//     let (input, (_, stat, _, _, _)) = tuple((multispace0, parse_member_expression, multispace0, tag(";"), multispace0))(input)?;

//     Ok((input, stat))
// }

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

fn get_property_assignment(input: &str) -> IResult<&str, Statement> {
    let (input, (_, _, statement, _)) = tuple((
        tag(":"),
        multispace0,
        alt((
            parse_boolean,
            parse_numeric,
            parse_identifier,
            parse_arithmetic_expression_to_expr,
            parse_object,
        )),
        multispace0,
    ))(input)?;

    Ok((input, statement))
}

fn parse_object_property(input: &str) -> IResult<&str, Property> {
    let (input, (_, key, _, statement, _, _, _)) = tuple((
        multispace0,
        get_identifier,
        multispace0,
        opt(get_property_assignment),
        multispace0,
        opt(tag(",")),
        multispace0,
    ))(input)?;

    if let Some(stat) = statement {
        return Ok((
            input,
            Property {
                key,
                value: Some(Box::new(stat)),
            },
        ));
    }

    return Ok((input, Property { key, value: None }));
}

fn parse_object_properties(input: &str) -> IResult<&str, Vec<Property>> {
    let (input, (_, properties, _)) = tuple((multispace0, many0(parse_object_property), multispace0))(input)?;

    Ok((input, properties))
}

fn parse_object(input: &str) -> IResult<&str, Statement> {
    let (input, obj) = delimited(tag("{"), generic::take_until_unbalanced('{', '}'), tag("}"))(input)?;

    let (rem, properties) = parse_object_properties(obj)?;

    if rem.len() > 0 {
        panic!("Object remainder to be parsed: {}", rem);
    }

    Ok((input, Statement::ObjectLiteral(Object { properties })))
}

fn parse_numeric(input: &str) -> IResult<&str, Statement> {
    let (input, num) = digit1(input)?;

    Ok((
        input,
        Statement::NumericLiteral(NumericLiteral {
            value: num.parse::<f64>().unwrap(),
        }),
    ))
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

fn parse_declaration(input: &str) -> IResult<&str, Statement> {
    let (input, (_, expression, _, id, _, _, _, assign, _, _, _)) = tuple((
        multispace0,
        alt((tag("let"), tag("const"))),
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

    let constant = match expression {
        "let" => false,
        "const" => true,
        _ => panic!("unknown declaration expression '{}'", expression),
    };

    let (rem, expression) = alt((
        parse_boolean,
        parse_object,
        parse_arithmetic_expression_to_expr,
        // parse_member_expression,
        parse_call_member_expression,
    ))(assign)?;

    if rem.len() > 0 {
        panic!("Input remainder to be parsed: {}", rem);
    }

    Ok((
        input,
        Statement::Declaration(Assign {
            id,
            constant,
            expression: Box::new(expression),
        }),
    ))
}

fn parse_assign(input: &str) -> IResult<&str, Statement> {
    let (input, (_, id, _, _, _, assign, _, _, _)) = tuple((
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

    let (rem, expression) = alt((
        parse_boolean,
        parse_object,
        parse_arithmetic_expression_to_expr,
        // parse_member_expression,
        parse_call_member_expression,
    ))(assign)?;

    if rem.len() > 0 {
        panic!("Input remainder to be parsed: {}", rem);
    }

    Ok((
        input,
        Statement::Assign(Assign {
            id,
            constant: false,
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
    let (input, program) = many0(alt((
        parse_comment,
        parse_boolean_literal,
        parse_numeric_literal,
        parse_identifier,
        parse_declaration,
        parse_assign,
        parse_call_expression,
        // parse_member_expression_literal,
        parse_call_member_expression_literal,
        parse_arithmetic_expression_to_expr,
    )))(input)?;

    let statements: Vec<Statement> = program;

    Ok((input, statements))
}

pub fn parse_ast(input: &str) -> IResult<&str, Program> {
    let (input, statements) = parse_program(input)?;

    let program = Program { body: statements };

    if input.len() > 0 {
        panic!("Input remainder to be parsed: {}", input);
    }

    Ok((input, program))
}
