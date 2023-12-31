use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::recognize,
    multi::many0_count,
    sequence::pair,
    IResult,
    Err,
    error::{Error, ErrorKind, ParseError},
};

pub fn get_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(alt((alpha1, tag("_"))), many0_count(alt((alphanumeric1, tag("_"))))))(input)
}

// fn take_until_closing_bracket(opening_bracket: &str, closing_bracket: &str, tokens: &mut Vec<Token>) -> Vec<Token> {
//     let mut close_pos = 0;
//     let mut counter = 1;
//     while counter > 0 {
//         if let Some(c) = tokens.get(close_pos) {
//             if c.value == opening_bracket {
//                 counter += 1;
//             } else if c.value == closing_bracket {
//                 counter -= 1;
//             }
//             close_pos += 1;
//         } else {
//             break;
//         }
//     }

//     if tokens.get(close_pos - 1).unwrap().value != closing_bracket {
//         panic!("Expected closing bracked '{}' but not found in '{:?}'", closing_bracket, tokens);
//     }

//     // println!("{:?}", tokens[1..close_pos-1].to_vec());
//     return tokens[1..close_pos - 1].to_vec();
// }

pub fn take_until_unbalanced(opening_bracket: char, closing_bracket: char) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i: &str| {
        let mut index = 0;
        let mut bracket_counter = 0;
        while let Some(n) = &i[index..].find(&[opening_bracket, closing_bracket, '\\'][..]) {
            index += n;
            let mut it = i[index..].chars();
            match it.next().unwrap_or_default() {
                c if c == '\\' => {
                    // Skip the escape char `\`.
                    index += '\\'.len_utf8();
                    // Skip also the following char.
                    let c = it.next().unwrap_or_default();
                    index += c.len_utf8();
                }
                c if c == opening_bracket => {
                    bracket_counter += 1;
                    index += opening_bracket.len_utf8();
                }
                c if c == closing_bracket => {
                    // Closing bracket.
                    bracket_counter -= 1;
                    index += closing_bracket.len_utf8();
                }
                // Can not happen.
                _ => unreachable!(),
            };
            // We found the unmatched closing bracket.
            if bracket_counter == -1 {
                // We do not consume it.
                index -= closing_bracket.len_utf8();
                return Ok((&i[index..], &i[0..index]));
            };
        }

        if bracket_counter == 0 {
            Ok(("", i))
        } else {
            Err(Err::Error(Error::from_error_kind(i, ErrorKind::TakeUntil)))
        }
    }
}
