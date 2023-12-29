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

#[derive(Debug, PartialEq)]
enum TokenType {
    Space,
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
    let mut iter = input.chars().into_iter().peekable();

    iter.clone()
        .map(|x| match x {
            '(' => Token::new(x.into(), TokenType::OpenParen),
            ')' => Token::new(x.into(), TokenType::CloseParen),
            '+' | '-' | '*' | '/' => Token::new(x.into(), TokenType::BinaryOperator),
            _ => {
                if x.is_numeric() {
                    let mut num: String = x.into();
                    while let Some(next) = iter.peek() {
                        if next.is_numeric() {
                            num.push(iter.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    Token::new(num, TokenType::Number)
                } else if x == ' ' {
                    Token::new(" ".to_owned(), TokenType::Space)
                } else if x.is_alphabetic() || x == '_' {
                    let mut id: String = x.into();
                    while let Some(next) = iter.peek() {
                        if next.is_alphanumeric() || next == &'_' {
                            id.push(iter.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    Token::new(id, TokenType::Identifier)
                } else {
                    panic!("Token '{}' is not yet implemented", x);
                }
            }
        })
        .filter(|x| x.r#type != TokenType::Space)
        .collect()
}

pub fn parse_dsl(input: &str) -> IResult<&str, Dsl> {
    let (input, statements) = parse_statements(input)?;

    let dsl = Dsl { statements };

    Ok((input, dsl))
}
