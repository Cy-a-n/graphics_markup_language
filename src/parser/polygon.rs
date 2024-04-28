use self::State::*;
use super::color::Color;
use super::i16::parse_i16;
use super::macros::transition;
use super::macros::transition_peek;
use super::point::Point;
use super::u8::parse_u8;
use super::u8::to_angle_rad;
use crate::error_handling::Error;
use crate::error_handling::Error::Parser;
use crate::error_handling::ParsedType;
use crate::error_handling::ParserError::{UnexpectedEnd, UnexpectedToken};
use crate::token::Token;
use crate::token::TokenValue;
use crate::token::TokenValue::{
    BorderColor, Children, Equals, FillColor, LeftBrace, LeftBracket, Position, RightBrace,
    RightBracket, Rotation, Vertices, Width,
};
use std::iter::Enumerate;
use std::str::FromStr;
use std::{iter::Peekable, slice::Iter};

#[derive(Debug, PartialEq)]
pub(super) struct Polygon {
    pub position: Point,
    pub rotation_rad: f32,
    pub width: i16,
    pub border_color: Color,
    pub fill_color: Color,
    pub vertices: Vec<Point>,
    pub children: Vec<Polygon>,
}

#[allow(unused)]
impl Polygon {
    pub fn default() -> Self {
        Polygon {
            position: Point::default(),
            rotation_rad: 0.0,
            width: 0,
            border_color: Color::default(),
            fill_color: Color::default(),
            vertices: vec![],
            children: vec![],
        }
    }

    /// This function constructs a polygon and all its children from the tokenized source code with a modified pushdown automata.
    #[allow(unused)]
    pub fn from_tokens<'a>(
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
                // Error states
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
}

/// Instead of a stack, we preserve information in the enum variants.
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
    VerticesValue(Polygon),
    Children(Polygon),
    Child(Polygon),
    ChildrenValue(Polygon),
    Return(Polygon),
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
                Position => State::Position,
            ),
            State::Position => transition_peek!(tokens_iter,
                Rotation => {tokens_iter.next(); State::Rotation(Polygon::default())},
                LeftBrace => {
                    let mut value = Polygon::default();
                    // Common operations like parsing points, colors, numbers, etc. are abstracted away to their own automata similar to this one.
                    // However, I don't bother to comment them, hopefully they are self-explanatory. 
                    value.position = match Point::from_token(tokens_iter, tokens) {
                        Ok(value) => value,
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
                    value.rotation_rad = match parse_u8(tokens_iter, tokens) {
                        Ok(value) => to_angle_rad(value),
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
                        Ok(value) => value,
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
                LeftBrace => {
                    value.border_color = match Color::from_token(tokens_iter, tokens) {
                        Ok(value) => value,
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
                LeftBrace => {
                    value.fill_color = match Color::from_token(tokens_iter, tokens) {
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
                Children => State::Children(value),
                LeftBracket => Vertex(value),
            ),
            Vertex(mut value) => transition_peek!(tokens_iter,
                RightBracket => {tokens_iter.next(); VerticesValue(value)},
                LeftBrace => {
                    value.vertices.push(match Point::from_token(tokens_iter, tokens) {
                        Ok(value) => value,
                        Err(error) => return Err(error),
                    });
                    Vertex(value)
                },
            ),
            VerticesValue( value) => transition!(tokens_iter,
                Children => State::Children(value),
            ),
            State::Children(value) => transition!(tokens_iter,
                RightBrace => Return(value),
                LeftBracket => Child(value),
            ),
            Child(mut value) => transition_peek!(tokens_iter,
                RightBracket => {tokens_iter.next(); ChildrenValue(value)},
                LeftBrace => {
                    value.children.push(match Polygon::from_tokens(tokens_iter, tokens) {
                        Ok(value) => value,
                        Err(error) => return Err(error),
                    });
                    Child(value)
                },
            ),
            ChildrenValue(value) => transition!(tokens_iter,
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
    use super::*;
    use crate::{
        error_handling::{
            ParsedType,
            ParserError::{UnexpectedEnd, UnexpectedToken},
        },
        token::{
            Token,
            TokenValue::{
                Blue, BorderColor, Equals, FillColor, Green, LeftBrace, LeftBracket, One, Position,
                Red, RightBrace, RightBracket, Rotation, Vertices, Width, Zero, X, Y,
            },
        },
    };

    #[test]
    fn unexpected_token() {
        let tokens = vec![Token::default(Equals)];
        let expected = Parser(UnexpectedToken {
            parsed_type: ParsedType::Polygon,
            current_value_slice: &tokens,
            expected_tokens: vec![LeftBrace],
        });
        if let Err(actual) =
            Polygon::from_tokens(&mut tokens.iter().enumerate().peekable(), &tokens)
        {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }

    #[test]
    fn tokens_unexpected_end() {
        let tokens = vec![Token::default(LeftBrace)];
        let expected = Error::Parser(UnexpectedEnd {
            parsed_type: ParsedType::Polygon,
            current_value_slice: &tokens,
            expected_tokens: vec![Position],
        });
        if let Err(actual) =
            Polygon::from_tokens(&mut tokens.iter().enumerate().peekable(), &tokens)
        {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }

    #[test]
    fn minimum() {
        let tokens = vec![
            Token::default(LeftBrace),
            Token::default(Position),
            Token::default(Rotation),
            Token::default(Width),
            Token::default(BorderColor),
            Token::default(FillColor),
            Token::default(Vertices),
            Token::default(Children),
            Token::default(RightBrace),
        ];
        let expected = Polygon::default();
        let actual = Polygon::from_tokens(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }

    #[test]
    fn maximum() {
        let tokens = vec![
            Token::default(LeftBrace),
            Token::default(Position),
            Token::default(LeftBrace),
            Token::default(X),
            Token::default(Y),
            Token::default(RightBrace),
            Token::default(Rotation),
            Token::default(Equals),
            Token::default(One),
            Token::default(Width),
            Token::default(Equals),
            Token::default(One),
            Token::default(Zero),
            Token::default(BorderColor),
            Token::default(LeftBrace),
            Token::default(Red),
            Token::default(Green),
            Token::default(Blue),
            Token::default(RightBrace),
            Token::default(FillColor),
            Token::default(LeftBrace),
            Token::default(Red),
            Token::default(Green),
            Token::default(Blue),
            Token::default(RightBrace),
            Token::default(Vertices),
            Token::default(LeftBracket),
            Token::default(LeftBrace),
            Token::default(X),
            Token::default(Y),
            Token::default(RightBrace),
            Token::default(LeftBrace),
            Token::default(X),
            Token::default(Y),
            Token::default(RightBrace),
            Token::default(RightBracket),
            Token::default(Children),
            Token::default(LeftBracket),
            Token::default(LeftBrace),
            Token::default(Position),
            Token::default(Rotation),
            Token::default(Width),
            Token::default(BorderColor),
            Token::default(FillColor),
            Token::default(Vertices),
            Token::default(Children),
            Token::default(RightBrace),
            Token::default(LeftBrace),
            Token::default(Position),
            Token::default(Rotation),
            Token::default(Width),
            Token::default(BorderColor),
            Token::default(FillColor),
            Token::default(Vertices),
            Token::default(Children),
            Token::default(RightBrace),
            Token::default(RightBracket),
            Token::default(RightBrace),
        ];
        let expected = Polygon {
            position: Point::default(),
            rotation_rad: to_angle_rad(1),
            width: 2,
            border_color: Color::default(),
            fill_color: Color::default(),
            vertices: vec![Point::default(), Point::default()],
            children: vec![Polygon::default(), Polygon::default()],
        };
        let actual = Polygon::from_tokens(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }
}
