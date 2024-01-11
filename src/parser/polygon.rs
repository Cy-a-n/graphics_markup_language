use self::State::*;
use super::color_parser::parse_color;
use super::i16_parser::parse_i16;
use super::macros::transition;
use super::macros::transition_peek;
use super::point_parser::parse_point;
use super::u8_parser::parse_u8;
use crate::draw_elements::Polygon;
use crate::error_handling::Error;
use crate::error_handling::Error::Parser;
use crate::error_handling::ParsedType;
use crate::error_handling::ParserError::{UnexpectedEnd, UnexpectedToken};
use crate::token::Token;
use crate::token::Value;
use crate::token::Value::{
    ArrayEnd, ArrayStart, BorderColor, Equals, FillColor, Position, Rotation, StructEnd,
    StructStart, Vertices, Width,
};
use std::iter::Enumerate;
use std::str::FromStr;
use std::{iter::Peekable, slice::Iter};

#[allow(unused)]
pub(super) fn parse_polygon<'a>(
    tokens_iter: &mut Peekable<Enumerate<Iter<Token>>>,
    tokens: &'a [Token],
) -> Result<Polygon, Error<'a>> {
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
            Return(mut value) => return Ok(value),
            UnexpectedEnd(expected_tokens) => {
                return Err(Parser(UnexpectedEnd {
                    parsed_type: ParsedType::Polygon,
                    current_value_slice: slice_from_value_start,
                    expected_tokens,
                }))
            }
            UnexpectedToken(expected_tokens, i) => {
                return Err(Parser(UnexpectedToken {
                    parsed_type: ParsedType::Polygon,
                    current_value_slice: &slice_from_value_start[..i + 1],
                    expected_tokens,
                }))
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
enum State {
    Start,
    StructStart,
    Position,
    PositionValue(Polygon),
    Rotation(Polygon),
    RotationValue(Polygon),
    Width(Polygon),
    WidthValue(Polygon),
    BorderColor(Polygon),
    BorderColorValue(Polygon),
    FillColor(Polygon),
    FillColorValue(Polygon),
    Vertices(Polygon),
    Vertex(Polygon),
    VerticesEnd(Polygon),
    Return(Polygon),
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
                Position => State::Position,
            ),
            State::Position => transition_peek!(tokens_iter,
                Rotation => {tokens_iter.next(); State::Rotation(Polygon::default())},
                StructStart => {
                    let mut value = Polygon::default();
                    value.position = match parse_point(tokens_iter, tokens) {
                        Ok(mut value) => value,
                        Err(error) => return Err(error),
                    };
                    PositionValue(value)
                },
            ),
            PositionValue(value) => transition!(tokens_iter,
                Rotation => State::Rotation(value),
            ),
            State::Rotation(mut value) => transition_peek!(tokens_iter,
                Width => {tokens_iter.next(); State::Width(value)},
                Equals => {
                    value.rotation = match parse_u8(tokens_iter, tokens) {
                        Ok(mut value) => value,
                        Err(error) => return Err(error),
                    };
                    RotationValue(value)
                },
            ),
            RotationValue( value) => transition!(tokens_iter,
                Width => State::Width(value),
            ),
            State::Width(mut value) => transition_peek!(tokens_iter,
                BorderColor => {tokens_iter.next(); State::BorderColor(value)},
                Equals => {
                    value.width = match parse_i16(tokens_iter, tokens) {
                        Ok(mut value) => value,
                        Err(error) => return Err(error),
                    };
                    WidthValue(value)
                },
            ),
            WidthValue( value) => transition!(tokens_iter,
                BorderColor => State::BorderColor(value),
            ),
            State::BorderColor(mut value) => transition_peek!(tokens_iter,
                FillColor => {tokens_iter.next(); State::FillColor(value)},
                StructStart => {
                    value.border_color = match parse_color(tokens_iter, tokens) {
                        Ok(mut value) => value,
                        Err(error) => return Err(error),
                    };
                    BorderColorValue(value)
                },
            ),
            BorderColorValue( value) => transition!(tokens_iter,
                FillColor => State::FillColor(value),
            ),
            State::FillColor(mut value) => transition_peek!(tokens_iter,
                Vertices => {tokens_iter.next(); State::Vertices(value)},
                StructStart => {
                    value.fill_color = match parse_color(tokens_iter, tokens) {
                        Ok(value) => value,
                        Err(error) => return Err(error),
                    };
                    FillColorValue(value)
                },
            ),
            FillColorValue(value) => transition!(tokens_iter,
                Vertices => State::Vertices(value),
            ),
            State::Vertices(value) => transition!(tokens_iter,
                StructEnd => Return(value),
                ArrayStart => Vertex(value),
            ),
            Vertex(mut value) => transition_peek!(tokens_iter,
                ArrayEnd => {tokens_iter.next(); VerticesEnd(value)},
                StructStart => {
                    value.vertices.push(match parse_point(tokens_iter, tokens) {
                        Ok(value) => value,
                        Err(error) => return Err(error),
                    });
                    Vertex(value)
                },
            ),
            VerticesEnd( value) => transition!(tokens_iter,
                StructEnd => Return(value),
            ),
            Return( _) => panic!("BUG: The `next_state` method should never be called on the `End` state. 'state': '{self:?}'."),
            UnexpectedEnd(_) => panic!("BUG: The `next_state` method should never be called on the `TokensUnexpectedEnd` state. 'state': '{self:?}'."),
            UnexpectedToken(_, _) => panic!("BUG: The `next_state` method should never be called on the `UnexpectedToken` state. 'state': '{self:?}'."),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        draw_elements::{self, Color, Point, Shape},
        error_handling::{
            ParsedType::Polygon,
            ParserError::{UnexpectedEnd, UnexpectedToken},
        },
        token::{
            Token,
            Value::{
                ArrayEnd, ArrayStart, Blue, BorderColor, Equals, FillColor, Green, One, Position,
                Red, Rotation, StructEnd, StructStart, Vertices, Width, Zero, X, Y,
            },
        },
    };

    #[test]
    fn unexpected_token() {
        let tokens = vec![Token::default(Equals)];
        let expected = Parser(UnexpectedToken {
            parsed_type: Polygon,
            current_value_slice: &tokens,
            expected_tokens: vec![StructStart],
        });
        if let Err(actual) = parse_polygon(&mut tokens.iter().enumerate().peekable(), &tokens) {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }

    #[test]
    fn tokens_unexpected_end() {
        let tokens = vec![Token::default(StructStart)];
        let expected = Error::Parser(UnexpectedEnd {
            parsed_type: Polygon,
            current_value_slice: &tokens,
            expected_tokens: vec![Position],
        });
        if let Err(actual) = parse_polygon(&mut tokens.iter().enumerate().peekable(), &tokens) {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }

    #[test]
    fn minimum() {
        let tokens = vec![
            Token::default(StructStart),
            Token::default(Position),
            Token::default(Rotation),
            Token::default(Width),
            Token::default(BorderColor),
            Token::default(FillColor),
            Token::default(Vertices),
            Token::default(StructEnd),
        ];
        let expected = draw_elements::Polygon::default();
        let actual = parse_polygon(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }

    #[test]
    fn maximum() {
        let tokens = vec![
            Token::default(StructStart),
            Token::default(Position),
            Token::default(StructStart),
            Token::default(X),
            Token::default(Y),
            Token::default(StructEnd),
            Token::default(Rotation),
            Token::default(Equals),
            Token::default(One),
            Token::default(Width),
            Token::default(Equals),
            Token::default(One),
            Token::default(Zero),
            Token::default(BorderColor),
            Token::default(StructStart),
            Token::default(Red),
            Token::default(Green),
            Token::default(Blue),
            Token::default(StructEnd),
            Token::default(FillColor),
            Token::default(StructStart),
            Token::default(Red),
            Token::default(Green),
            Token::default(Blue),
            Token::default(StructEnd),
            Token::default(Vertices),
            Token::default(ArrayStart),
            Token::default(StructStart),
            Token::default(X),
            Token::default(Y),
            Token::default(StructEnd),
            Token::default(StructStart),
            Token::default(X),
            Token::default(Y),
            Token::default(StructEnd),
            Token::default(ArrayEnd),
            Token::default(StructEnd),
        ];
        let expected = draw_elements::Polygon {
            position: Point::default(),
            rotation: 1,
            width: 2,
            border_color: Color::default(),
            fill_color: Color::default(),
            vertices: vec![Point::default(), Point::default()],
        };
        let actual = parse_polygon(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }
}
