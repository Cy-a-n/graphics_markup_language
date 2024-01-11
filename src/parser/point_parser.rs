use self::States::*;
use super::i16_parser::parse_i16;
use super::macros::transition;
use super::macros::transition_peek;
use crate::draw_elements::Point;
use crate::error_handling::Error;
use crate::error_handling::Error::Parser;
use crate::error_handling::ParsedType;
use crate::error_handling::ParserError::UnexpectedEnd;
use crate::error_handling::ParserError::UnexpectedToken;
use crate::token::Token;
use crate::token::Value;
use crate::token::Value::{AttributX, AttributY, EqualsChar, StructEnd, StructStart};
use std::iter::Enumerate;
use std::str::FromStr;
use std::{iter::Peekable, slice::Iter};

#[allow(unused)]
pub(super) fn parse_point<'a>(
    tokens_iter: &mut Peekable<Enumerate<Iter<Token>>>,
    tokens: &'a [Token],
) -> Result<Point, Error<'a>> {
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
                    parsed_type: ParsedType::Point,
                    current_value_slice: slice_from_value_start,
                    expected_tokens,
                }))
            }
            UnexpectedToken(expected_tokens, i) => {
                return Err(Parser(UnexpectedToken {
                    parsed_type: ParsedType::Point,
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
    AttributX,
    AttributXValue(Point),
    AttributY(Point),
    AttributYValue(Point),
    Return(Point),
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
                AttributX => States::AttributX,
            ),
            States::AttributX => transition_peek!(tokens_iter,
                AttributY => {tokens_iter.next(); States::AttributY(Point::default())},
                EqualsChar => {
                    let mut point = Point::default();
                    point.x = match parse_i16(tokens_iter, tokens) {
                        Ok(value) => value,
                        Err(error) => return Err(error),
                    };
                    AttributXValue(point)
                },
            ),
            AttributXValue(point) => transition!(tokens_iter,
                AttributY => States::AttributY(point),
            ),
            States::AttributY(mut point) => transition_peek!(tokens_iter,
                StructEnd => {tokens_iter.next(); Return(point)},
                EqualsChar => {
                    point.y = match parse_i16(tokens_iter, tokens) {
                        Ok(value) => value,
                        Err(error) => return Err(error),
                    };
                    AttributYValue(point)
                },
            ),
            AttributYValue(point) => transition!(tokens_iter,
                StructEnd => Return(point),
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
            ParsedType::Point,
            ParserError::{UnexpectedEnd, UnexpectedToken},
        },
        token::{
            Token,
            Value::{AttributX, AttributY, EqualsChar, One, StructEnd, StructStart, Zero},
        },
    };

    #[test]
    fn unexpected_token() {
        let tokens = vec![Token::default(EqualsChar)];
        let expected = Parser(UnexpectedToken {
            parsed_type: Point,
            current_value_slice: &tokens,
            expected_tokens: vec![StructStart],
        });
        if let Err(actual) = parse_point(&mut tokens.iter().enumerate().peekable(), &tokens) {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }

    #[test]
    fn tokens_unexpected_end() {
        let tokens = vec![Token::default(StructStart)];
        let expected = Error::Parser(UnexpectedEnd {
            parsed_type: Point,
            current_value_slice: &tokens,
            expected_tokens: vec![AttributX],
        });
        if let Err(actual) = parse_point(&mut tokens.iter().enumerate().peekable(), &tokens) {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }

    #[test]
    fn minimum() {
        let tokens = vec![
            Token::default(StructStart),
            Token::default(AttributX),
            Token::default(AttributY),
            Token::default(StructEnd),
        ];
        let expected = draw_elements::Point::default();
        let actual = parse_point(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }

    #[test]
    fn maximum() {
        let tokens = vec![
            Token::default(StructStart),
            Token::default(AttributX),
            Token::default(EqualsChar),
            Token::default(One),
            Token::default(AttributY),
            Token::default(EqualsChar),
            Token::default(One),
            Token::default(Zero),
            Token::default(StructEnd),
        ];
        let expected = draw_elements::Point { x: 1, y: 2 };
        let actual = parse_point(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }
}
