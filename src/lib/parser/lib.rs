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

#[derive(Debug)]
enum TokenType {
    Number,
    Identifier,
    OpenParen,
    CloseParen,
    BinaryOperator,
    Let,
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
enum Statement<'a> {
    Assign(Assign<'a>),
}

#[derive(Debug)]
pub struct Dsl<'a> {
    statements: Vec<Statement<'a>>,
}

fn get_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(alt((alpha1, tag("_"))), many0_count(alt((alphanumeric1, tag("_"))))))(input)
}

fn parse_assign(input: &str) -> IResult<&str, Statement> {
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

    Ok((input, Statement::Assign(Assign { id, tokens })))
}

fn parse_statements(input: &str) -> IResult<&str, Vec<Statement>> {
    let (input, assign) = parse_assign(input)?;

    // let actions = Dsl::new(file, transform);

    let statements: Vec<Statement> = vec![assign];

    Ok((input, statements))
}

fn tokenzine(input: &str) -> Vec<Token> {
    input.chars().into_iter().map(|x| match x {
        '(' => Token::new(x.into(), TokenType::OpenParen),
        ')' => Token::new(x.into(), TokenType::CloseParen),
        '+' | '-' | '*' | '/' => Token::new(x.into(), TokenType::BinaryOperator),
        _ => panic!("Token '{}' is not yet implemented", x),
    }).collect()
}

pub fn parse_dsl(input: &str) -> IResult<&str, Dsl> {
    let (input, statements) = parse_statements(input)?;

    let dsl = Dsl { statements };

    Ok((input, dsl))
}
