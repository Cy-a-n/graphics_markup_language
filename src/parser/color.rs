use self::State::*;
use super::macros::transition;
use super::macros::transition_peek;
use super::u8::parse_u8;
use crate::error_handling::Error;
use crate::error_handling::Error::Parser;
use crate::error_handling::ParsedType;
use crate::error_handling::ParserError::UnexpectedEnd;
use crate::error_handling::ParserError::UnexpectedToken;
use crate::token::Token;
use crate::token::Value;
use crate::token::Value::{
    Blue, Green, Red, Equals, StructEnd, StructStart,
};
use std::iter::Enumerate;
use std::str::FromStr;
use std::{iter::Peekable, slice::Iter};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub(super) red: u8,
    pub(super) green: u8,
    pub(super) blue: u8,
}

#[allow(unused)]
impl Color {
    pub (super) fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    #[allow(unused)]
    pub(super) fn from_token<'a>(
        tokens_iter: &mut Peekable<Enumerate<Iter<Token>>>,
        tokens: &'a [Token],
    ) -> Result<Self, Error<'a>> {
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



    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }
}


#[derive(Debug)]
enum State {
    Start,
    StructStart,
    Red,
    RedValue(Color),
    Green(Color),
    GreenValue(Color),
    Blue(Color),
    BlueValue(Color),
    Return(Color),
    UnexpectedEnd(Vec<Value>),
    UnexpectedToken(Vec<Value>, usize),
}

impl State {
    fn next_state<'a>(
        self,
        tokens_iter: &mut Peekable<Enumerate<Iter<Token>>>,
        tokens: &'a [Token],
    ) -> Result<Self, Error<'a>> {
        Ok(match self {
            Start => transition!(tokens_iter,
                StructStart => State::StructStart,
            ),
            State::StructStart => transition!(tokens_iter,
                Red => State::Red,
            ),
            State::Red => transition_peek!(tokens_iter,
                Green => {tokens_iter.next(); State::Green(Color::default())},
                Equals => {
                    let mut value = Color::default();
                    value.red = match parse_u8(tokens_iter, tokens) {
                        Ok(value) => value,
                        Err(error) => return Err(error),
                    };
                    RedValue(value)
                },
            ),
            RedValue(value) => transition!(tokens_iter,
                Green => State::Green(value),
            ),
            State::Green(mut value) => transition_peek!(tokens_iter,
                Blue => {tokens_iter.next(); State::Blue(value)},
                Equals => {
                    value.green = match parse_u8(tokens_iter, tokens) {
                        Ok(value) => value,
                        Err(error) => return Err(error),
                    };
                    GreenValue(value)
                },
            ),
            GreenValue(value) => transition!(tokens_iter,
                Blue => State::Blue(value),
            ),
            State::Blue(mut value) => transition_peek!(tokens_iter,
                StructEnd => {tokens_iter.next(); Return(value)},
                Equals => {
                    value.blue = match parse_u8(tokens_iter, tokens) {
                        Ok(value) => value,
                        Err(error) => return Err(error),
                    };
                    BlueValue(value)
                },                
            ),
            BlueValue(value) => transition!(tokens_iter,
                StructEnd => Return(value),
            ),
            Return(_) => panic!("BUG: The 'next_state' method should never be called on the 'End' state. 'state': '{self:?}'."),
            UnexpectedEnd(_) => panic!("BUG: The 'next_state' method should never be called on the 'TokensUnexpectedEnd' state. 'state': '{self:?}'."),
            UnexpectedToken(_, _) => panic!("BUG: The 'next_state' method should never be called on the 'UnexpectedToken' state. 'state': '{self:?}'."),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Color;
    use crate::{
        error_handling::{
            ParsedType,
            ParserError::{UnexpectedEnd, UnexpectedToken},
        },
        token::{
            Token,
            Value::{ Equals, One, StructEnd, StructStart, Zero},
        },
    };

    #[test]
    fn unexpected_token() {
        let tokens = vec![Token::default(Equals)];
        let expected = Parser(UnexpectedToken {
            parsed_type: ParsedType::Color,
            current_value_slice: &tokens,
            expected_tokens: vec![StructStart],
        });
        if let Err(actual) = Color::from_token(&mut tokens.iter().enumerate().peekable(), &tokens) {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }

    #[test]
    fn tokens_unexpected_end() {
        let tokens = vec![Token::default(StructStart)];
        let expected = Error::Parser(UnexpectedEnd {
            parsed_type: ParsedType::Color,
            current_value_slice: &tokens,
            expected_tokens: vec![Red],
        });
        if let Err(actual) = Color::from_token(&mut tokens.iter().enumerate().peekable(), &tokens) {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }

    #[test]
    fn minimum() {
        let tokens = vec![
            Token::default(StructStart),
            Token::default(Red),
            Token::default(Green),
            Token::default(Blue),
            Token::default(StructEnd),
        ];
        let expected = Color::default();
        let actual = Color::from_token(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }

    #[test]
    fn maximum() {
        let tokens = vec![
            Token::default(StructStart),
            Token::default(Red),
            Token::default(Equals),
            Token::default(One),
            Token::default(Green),
            Token::default(Equals),
            Token::default(One),
            Token::default(Zero),
            Token::default(Blue),
            Token::default(Equals),
            Token::default(One),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(StructEnd),
        ];
        let expected = Color { red: 1, green: 2, blue: 4 };
        let actual = Color::from_token(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }
}
