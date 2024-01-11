use self::States::*;
use super::macros::transition;
use super::macros::transition_peek;
use super::u8_parser::parse_u8;
use crate::draw_elements::Color;
use crate::error_handling::Error;
use crate::error_handling::Error::Parser;
use crate::error_handling::ParsedType;
use crate::error_handling::ParserError::UnexpectedEnd;
use crate::error_handling::ParserError::UnexpectedToken;
use crate::token::Token;
use crate::token::Value;
use crate::token::Value::{
    AttributBlue, AttributGreen, AttributRed, EqualsChar, StructEnd, StructStart,
};
use std::iter::Enumerate;
use std::str::FromStr;
use std::{iter::Peekable, slice::Iter};

#[allow(unused)]
pub(super) fn parse_color<'a>(
    tokens_iter: &mut Peekable<Enumerate<Iter<Token>>>,
    tokens: &'a [Token],
) -> Result<Color, Error<'a>> {
    let slice_from_value_start = &tokens[tokens_iter
        .peek()
        .expect("BUG: 'tokens_iter' should have at least one token.")
        .0..];
    let mut state = Start;

    loop {
        state = match state.next_state(tokens_iter, tokens) {
            Ok(state) => state,
            Err(error) => return Err(error),
        };
        match state {
            Return(value) => return Ok(value),
            UnexpectedEnd(expected_tokens) => {
                return Err(Parser(UnexpectedEnd {
                    parsed_type: ParsedType::Color,
                    current_value_slice: slice_from_value_start,
                    expected_tokens,
                }))
            }
            UnexpectedToken(expected_tokens, i) => {
                return Err(Parser(UnexpectedToken {
                    parsed_type: ParsedType::Color,
                    current_value_slice: &slice_from_value_start[..i + 1],
                    expected_tokens,
                }))
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
enum States {
    Start,
    StructStart,
    AttributRed,
    AttributRedValue(Color),
    AttributGreen(Color),
    AttributGreenValue(Color),
    AttributBlue(Color),
    AttributBlueValue(Color),
    Return(Color),
    UnexpectedEnd(Vec<Value>),
    UnexpectedToken(Vec<Value>, usize),
}

impl States {
    fn next_state<'a>(
        self,
        tokens_iter: &mut Peekable<Enumerate<Iter<Token>>>,
        tokens: &'a [Token],
    ) -> Result<Self, Error<'a>> {
        Ok(match self {
            Start => transition!(tokens_iter,
                StructStart => States::StructStart,
            ),
            States::StructStart => transition!(tokens_iter,
                AttributRed => States::AttributRed,
            ),
            States::AttributRed => transition_peek!(tokens_iter,
                AttributGreen => {tokens_iter.next(); States::AttributGreen(Color::default())},
                EqualsChar => {
                    let mut value = Color::default();
                    value.red = match parse_u8(tokens_iter, tokens) {
                        Ok(value) => value,
                        Err(error) => return Err(error),
                    };
                    AttributRedValue(value)
                },
            ),
            AttributRedValue(value) => transition!(tokens_iter,
                AttributGreen => States::AttributGreen(value),
            ),
            States::AttributGreen(mut value) => transition_peek!(tokens_iter,
                AttributBlue => {tokens_iter.next(); States::AttributBlue(value)},
                EqualsChar => {
                    value.green = match parse_u8(tokens_iter, tokens) {
                        Ok(value) => value,
                        Err(error) => return Err(error),
                    };
                    AttributGreenValue(value)
                },
            ),
            AttributGreenValue(value) => transition!(tokens_iter,
                AttributBlue => States::AttributBlue(value),
            ),
            States::AttributBlue(mut value) => transition_peek!(tokens_iter,
                StructEnd => {tokens_iter.next(); Return(value)},
                EqualsChar => {
                    value.blue = match parse_u8(tokens_iter, tokens) {
                        Ok(value) => value,
                        Err(error) => return Err(error),
                    };
                    AttributBlueValue(value)
                },                
            ),
            AttributBlueValue(value) => transition!(tokens_iter,
                StructEnd => Return(value),
            ),
            Return(_) => panic!("BUG: The `next_state` method should never be called on the `End` state. 'state': '{self:?}'."),
            UnexpectedEnd(_) => panic!("BUG: The `next_state` method should never be called on the `TokensUnexpectedEnd` state. 'state': '{self:?}'."),
            UnexpectedToken(_, _) => panic!("BUG: The `next_state` method should never be called on the `UnexpectedToken` state. 'state': '{self:?}'."),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        draw_elements,
        error_handling::{
            ParsedType::Color,
            ParserError::{UnexpectedEnd, UnexpectedToken},
        },
        token::{
            Token,
            Value::{ EqualsChar, One, StructEnd, StructStart, Zero},
        },
    };

    #[test]
    fn unexpected_token() {
        let tokens = vec![Token::default(EqualsChar)];
        let expected = Parser(UnexpectedToken {
            parsed_type: Color,
            current_value_slice: &tokens,
            expected_tokens: vec![StructStart],
        });
        if let Err(actual) = parse_color(&mut tokens.iter().enumerate().peekable(), &tokens) {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }

    #[test]
    fn tokens_unexpected_end() {
        let tokens = vec![Token::default(StructStart)];
        let expected = Error::Parser(UnexpectedEnd {
            parsed_type: Color,
            current_value_slice: &tokens,
            expected_tokens: vec![AttributRed],
        });
        if let Err(actual) = parse_color(&mut tokens.iter().enumerate().peekable(), &tokens) {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }

    #[test]
    fn minimum() {
        let tokens = vec![
            Token::default(StructStart),
            Token::default(AttributRed),
            Token::default(AttributGreen),
            Token::default(AttributBlue),
            Token::default(StructEnd),
        ];
        let expected = draw_elements::Color::default();
        let actual = parse_color(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }

    #[test]
    fn maximum() {
        let tokens = vec![
            Token::default(StructStart),
            Token::default(AttributRed),
            Token::default(EqualsChar),
            Token::default(One),
            Token::default(AttributGreen),
            Token::default(EqualsChar),
            Token::default(One),
            Token::default(Zero),
            Token::default(AttributBlue),
            Token::default(EqualsChar),
            Token::default(One),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(StructEnd),
        ];
        let expected = draw_elements::Color { red: 1, green: 2, blue: 4 };
        let actual = parse_color(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }
}
