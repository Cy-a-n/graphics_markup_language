use self::State::*;
use super::i16::parse_i16;
use super::macros::transition;
use super::macros::transition_peek;
use crate::error_handling::Error;
use crate::error_handling::Error::Parser;
use crate::error_handling::ParsedType;
use crate::error_handling::ParserError::UnexpectedEnd;
use crate::error_handling::ParserError::UnexpectedToken;
use crate::token::Token;
use crate::token::TokenValue;
use crate::token::TokenValue::{Equals, LeftBrace, RightBrace, X, Y};
use std::iter::Enumerate;
use std::str::FromStr;
use std::{iter::Peekable, slice::Iter};

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[allow(unused)]
impl Point {
    pub(super) fn default() -> Self {
        Self { x: 0, y: 0 }
    }

    #[allow(unused)]
    pub(super) fn from_token<'a>(
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

    pub(super) fn add(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }

    pub fn sub(&mut self, rhs: &Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }

    pub(super) fn from_point_f32(point: &PointF32) -> Self {
        Self {
            x: point.x.round() as i16,
            y: point.y.round() as i16,
        }
    }

    pub fn rotate(&mut self, angle_rad: f32) {
        let mut point_f32 = PointF32::from_point(self);
        point_f32.rotate(angle_rad);
        *self = Self::from_point_f32(&point_f32);
    }

    pub fn rotate_around(&mut self, center: &Self, angle_rad: f32) {
        let mut point_f32 = PointF32::from_point(self);
        point_f32.rotate_around(&PointF32::from_point(center), angle_rad);
        *self = Self::from_point_f32(&point_f32);
    }
}

pub(super) struct PointF32 {
    pub x: f32,
    pub y: f32,
}

impl PointF32 {
    pub fn from_point(point: &Point) -> Self {
        Self {
            x: point.x as f32,
            y: point.y as f32,
        }
    }

    pub fn rotate(&mut self, angle_rad: f32) {
        let new_x = self.x * angle_rad.cos() - self.y * angle_rad.sin();
        let new_y = self.x * angle_rad.sin() + self.y * angle_rad.cos();

        self.x = new_x;
        self.y = new_y;
    }

    pub fn rotate_around(&mut self, center: &Self, angle_rad: f32) {
        // Translate the point and the center to the origin
        self.x -= center.x;
        self.y -= center.y;

        // Rotate the translated point around the origin
        self.rotate(angle_rad);

        // Translate the rotated point back to its original position
        self.x += center.x;
        self.y += center.y;
    }
}

#[derive(Debug)]
enum State {
    Start,
    StructStart,
    X,
    XValue(Point),
    Y(Point),
    YValue(Point),
    Return(Point),
    UnexpectedEnd(Vec<TokenValue>),
    UnexpectedToken(Vec<TokenValue>, usize),
}

impl State {
    fn next_state<'a>(
        self,
        tokens_iter: &mut Peekable<Enumerate<Iter<Token>>>,
        tokens: &'a [Token],
    ) -> Result<Self, Error<'a>> {
        Ok(match self {
            Start => transition!(tokens_iter,
                LeftBrace => State::StructStart,
            ),
            State::StructStart => transition!(tokens_iter,
                X => State::X,
            ),
            State::X => transition_peek!(tokens_iter,
                Y => {tokens_iter.next(); State::Y(Point::default())},
                Equals => {
                    let mut value = Point::default();
                    value.x = match parse_i16(tokens_iter, tokens) {
                        Ok(value) => value,
                        Err(error) => return Err(error),
                    };
                    XValue(value)
                },
            ),
            XValue(value) => transition!(tokens_iter,
                Y => State::Y(value),
            ),
            State::Y(mut value) => transition_peek!(tokens_iter,
                RightBrace => {tokens_iter.next(); Return(value)},
                Equals => {
                    value.y = match parse_i16(tokens_iter, tokens) {
                        Ok(value) => value,
                        Err(error) => return Err(error),
                    };
                    YValue(value)
                },
            ),
            YValue(value) => transition!(tokens_iter,
                RightBrace => Return(value),
            ),
            Return(_) => panic!("BUG: The 'next_state' method should never be called on the 'End' state. 'state': '{self:?}'."),
            UnexpectedEnd(_) => panic!("BUG: The 'next_state' method should never be called on the 'TokensUnexpectedEnd' state. 'state': '{self:?}'."),
            UnexpectedToken(_, _) => panic!("BUG: The 'next_state' method should never be called on the 'UnexpectedToken' state. 'state': '{self:?}'."),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::{
        error_handling::{
            Error::Parser,
            ParsedType,
            ParserError::{UnexpectedEnd, UnexpectedToken},
        },
        token::{
            Token,
            TokenValue::{Equals, LeftBrace, One, RightBrace, Zero, X, Y},
        },
    };

    #[test]
    fn unexpected_token() {
        let tokens = vec![Token::default(Equals)];
        let expected = Parser(UnexpectedToken {
            parsed_type: ParsedType::Point,
            current_value_slice: &tokens,
            expected_tokens: vec![LeftBrace],
        });
        if let Err(actual) = Point::from_token(&mut tokens.iter().enumerate().peekable(), &tokens) {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }

    #[test]
    fn tokens_unexpected_end() {
        let tokens = vec![Token::default(LeftBrace)];
        let expected = Parser(UnexpectedEnd {
            parsed_type: ParsedType::Point,
            current_value_slice: &tokens,
            expected_tokens: vec![X],
        });
        if let Err(actual) = Point::from_token(&mut tokens.iter().enumerate().peekable(), &tokens) {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }

    #[test]
    fn minimum() {
        let tokens = vec![
            Token::default(LeftBrace),
            Token::default(X),
            Token::default(Y),
            Token::default(RightBrace),
        ];
        let expected = Point::default();
        let actual = Point::from_token(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }

    #[test]
    fn maximum() {
        let tokens = vec![
            Token::default(LeftBrace),
            Token::default(X),
            Token::default(Equals),
            Token::default(One),
            Token::default(Y),
            Token::default(Equals),
            Token::default(One),
            Token::default(Zero),
            Token::default(RightBrace),
        ];
        let expected = Point { x: 1, y: 2 };
        let actual = Point::from_token(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }

    // Test Point::rotate()
    #[test]
    fn rotate_around() {
        let mut point = Point { x: 1, y: 0 };
        let center = Point { x: 0, y: 0 };
        let angle_rad = std::f32::consts::PI;

        point.rotate_around(&center, angle_rad);

        assert_eq!(point.x, -1);
        assert_eq!(point.y, 0);
    }
}
