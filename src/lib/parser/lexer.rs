const RESERVED: [&str; 1] = ["None"];

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    None,
    Number,
    Identifier,
    OpenParen,
    CloseParen,
    BinaryOperator,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub r#type: TokenType,
}

impl Token {
    pub fn new(value: String, r#type: TokenType) -> Self {
        Self { value, r#type }
    }
}

pub fn tokenzine(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = input.chars().into_iter().peekable();

    while let Some(ch) = iter.next() {
        match ch {
            '(' => tokens.push(Token::new(ch.into(), TokenType::OpenParen)),
            ')' => tokens.push(Token::new(ch.into(), TokenType::CloseParen)),
            '+' | '-' | '*' | '/' | '%' => tokens.push(Token::new(ch.into(), TokenType::BinaryOperator)),
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

                    if RESERVED.contains(&id.as_str()) {
                        match id.as_str() {
                            "None" => tokens.push(Token::new(id, TokenType::None)),
                            _ => panic!("need to implement reserverd '{}' keywork in tokenizer", id)
                        }
                    } else {
                        tokens.push(Token::new(id, TokenType::Identifier));
                    }
                } else {
                    panic!("Token '{}' is not yet implemented", ch);
                }
            }
        }
    }

    tokens
}
